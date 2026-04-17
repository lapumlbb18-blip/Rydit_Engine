//! RyDit Loader - Carga dinámica de módulos, assets y hot reload
//!
//! Proporciona carga dinámica de módulos compilados (.so/.dll),
//! abstracción de proveedores de assets (SDL2/Raylib) y
//! sistema de hot reload para el ecosistema RyDit.

pub mod compressor;
#[cfg(test)]
mod test_asset_server;

use crate::compressor::{Compressor, BasisCompressor};


use ry_core::{ModuleError, ModuleMetadata, ModuleRegistry};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[cfg(not(target_os = "android"))]
use std::ffi::OsStr;

// ============================================================================
// ASSET PROVIDER TRAIT
// ============================================================================

pub trait AssetProvider {
    fn load_texture(&self, path: &str) -> Result<Vec<u8>, String>;
    fn load_audio(&self, path: &str) -> Result<Vec<u8>, String>;
}

pub enum AssetError {
    NotFound(String),
    LoadFailed(String),
    UnsupportedFormat(String),
}

use serde::{Deserialize, Serialize};

// ============================================================================
// ASSET TYPES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Texture,
    Audio,
    Json,
    Config,
}

// ============================================================================
// ASSET SERVER
// ============================================================================

pub struct AssetServer {
    provider: Arc<dyn AssetProvider + Send + Sync>,
    compressor: Box<dyn Compressor + Send + Sync>,
    cache: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl AssetServer {
    pub fn new(provider: Arc<dyn AssetProvider + Send + Sync>) -> Self {
        Self {
            provider,
            compressor: Box::new(BasisCompressor),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn load_texture(&self, path: &str) -> Result<Vec<u8>, String> {
        self.get_or_load(path, || self.provider.load_texture(path))
    }

    pub fn load_audio(&self, path: &str) -> Result<Vec<u8>, String> {
        self.get_or_load(path, || self.provider.load_audio(path))
    }

    /// Cargar un asset y deserializarlo desde JSON
    pub fn load_typed<T>(&self, path: &str) -> Result<T, String> 
    where T: for<'de> Deserialize<'de> 
    {
        let data = self.get_or_load(path, || {
            // Asumimos que el provider puede leer archivos genéricos como texturas por ahora
            self.provider.load_texture(path) 
        })?;

        serde_json::from_slice(&data).map_err(|e| format!("Error deserializando {}: {}", path, e))
    }

    fn get_or_load<F>(&self, path: &str, loader: F) -> Result<Vec<u8>, String>
    where F: FnOnce() -> Result<Vec<u8>, String>
    {
        let cache = self.cache.read().map_err(|e| e.to_string())?;
        if let Some(data) = cache.get(path) {
            return Ok(data.clone());
        }
        drop(cache);

        let raw_data = loader()?;
        let data = self.compressor.decompress(&raw_data).unwrap_or(raw_data);

        let mut cache = self.cache.write().map_err(|e| e.to_string())?;
        cache.insert(path.to_string(), data.clone());
        Ok(data)
    }

    pub fn reload_asset(&self, path: &str) -> Result<Vec<u8>, String> {
        let raw_data = self.provider.load_texture(path)?;
        let data = self.compressor.decompress(&raw_data).unwrap_or(raw_data);
        
        let mut cache = self.cache.write().map_err(|e| e.to_string())?;
        cache.insert(path.to_string(), data.clone());
        Ok(data)
    }
}

// ============================================================================
// RAYLIB ASSET PROVIDER
// ============================================================================

pub struct RaylibAssetProvider;

impl AssetProvider for RaylibAssetProvider {
    fn load_texture(&self, path: &str) -> Result<Vec<u8>, String> {
        Err(format!("Carga vía Raylib no implementada aún: {}", path))
    }

    fn load_audio(&self, path: &str) -> Result<Vec<u8>, String> {
        Err(format!("Carga de audio Raylib no implementada: {}", path))
    }
}

// ============================================================================
// LOADER Y GESTIÓN
// ============================================================================

pub type LoaderResult = Result<(), LoaderError>;

#[derive(Debug, Clone)]
pub struct LoaderError {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for LoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for LoaderError {}

impl From<ModuleError> for LoaderError {
    fn from(err: ModuleError) -> Self {
        LoaderError {
            code: err.code,
            message: err.message,
        }
    }
}

#[derive(Clone)]
pub struct LoadedModuleInfo {
    pub name: String,
    pub path: String,
    pub metadata: ModuleMetadata,
    pub loaded_at: u64,
}

pub struct DynamicModuleLoader {
    registry: ModuleRegistry,
    loaded_modules: HashMap<String, LoadedModuleInfo>,
    timestamp: u64,
}

impl DynamicModuleLoader {
    pub fn new() -> Self {
        Self {
            registry: ModuleRegistry::new(),
            loaded_modules: HashMap::new(),
            timestamp: 0,
        }
    }

    pub fn registry(&self) -> &ModuleRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut ModuleRegistry {
        &mut self.registry
    }

    #[cfg(not(target_os = "android"))]
    #[cfg(unix)]
    pub fn load_library<P: AsRef<OsStr>>(&mut self, path: P) -> Result<String, LoaderError> {
        use libloading::Library;
        use ry_core::RyditModule;

        self.timestamp += 1;

        let lib = unsafe { Library::new(path.as_ref()) }.map_err(|e| LoaderError {
            code: "LOAD_ERROR".to_string(),
            message: format!("No se pudo cargar la biblioteca: {}", e),
        })?;

        let create_module: libloading::Symbol<unsafe extern "C" fn() -> *mut dyn RyditModule> =
            unsafe { lib.get(b"create_module") }.map_err(|e| LoaderError {
                code: "SYMBOL_NOT_FOUND".to_string(),
                message: format!("Símbolo 'create_module' no encontrado: {}", e),
            })?;

        let module_ptr = unsafe { create_module() };
        let module = unsafe { Box::from_raw(module_ptr) };
        let module_name = module.name().to_string();
        let module_metadata = module.metadata();

        self.registry.register(module);

        let info = LoadedModuleInfo {
            name: module_name.clone(),
            path: path.as_ref().to_string_lossy().to_string(),
            metadata: module_metadata,
            loaded_at: self.timestamp,
        };
        self.loaded_modules.insert(module_name.clone(), info);

        Ok(module_name)
    }

    pub fn reload(&mut self, name: &str) -> LoaderResult {
        if !self.registry.contains(name) {
            return Err(LoaderError {
                code: "MODULE_NOT_FOUND".to_string(),
                message: format!("Módulo '{}' no encontrado", name),
            });
        }

        self.registry.reload(name);

        if let Some(info) = self.loaded_modules.get_mut(name) {
            info.loaded_at = self.timestamp + 1;
            self.timestamp += 1;
        }

        Ok(())
    }

    pub fn unload(&mut self, name: &str) -> LoaderResult {
        if !self.registry.contains(name) {
            return Err(LoaderError {
                code: "MODULE_NOT_FOUND".to_string(),
                message: format!("Módulo '{}' no encontrado", name),
            });
        }

        self.registry.unload(name);
        self.loaded_modules.remove(name);
        self.timestamp += 1;

        Ok(())
    }

    pub fn list_modules(&self) -> Vec<&str> {
        self.loaded_modules.keys().map(|s| s.as_str()).collect()
    }

    pub fn get_module_info(&self, name: &str) -> Option<&LoadedModuleInfo> {
        self.loaded_modules.get(name)
    }

    pub fn list_with_metadata(&self) -> Vec<(&str, ModuleMetadata)> {
        self.registry.list_with_metadata()
    }

    pub fn is_loaded(&self, name: &str) -> bool {
        self.loaded_modules.contains_key(name)
    }

    pub fn len(&self) -> usize {
        self.loaded_modules.len()
    }

    pub fn is_empty(&self) -> bool {
        self.loaded_modules.is_empty()
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

impl Default for DynamicModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ry_core::{ModuleResult, RyditModule};

    struct TestModule;

    impl RyditModule for TestModule {
        fn name(&self) -> &'static str {
            "test_module"
        }
        fn version(&self) -> &'static str {
            "1.0.0"
        }
        fn register(&self) -> HashMap<&'static str, &'static str> {
            HashMap::new()
        }
        fn execute(&self, _command: &str, _params: serde_json::Value) -> ModuleResult {
            Ok(serde_json::Value::Null)
        }
        fn metadata(&self) -> ModuleMetadata {
            ModuleMetadata::new()
        }
    }

    #[test]
    fn test_loader_new() {
        let loader = DynamicModuleLoader::new();
        assert!(loader.is_empty());
    }
}
