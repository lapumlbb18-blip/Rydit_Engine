//! RyDit Loader - Carga dinámica de módulos y hot reload
//!
//! Proporciona carga dinámica de módulos compilados (.so/.dll)
//! y sistema de hot reload para el ecosistema RyDit.
//!
//! # Ejemplo
//! ```rust,no_run
//! use rydit_loader::DynamicModuleLoader;
//!
//! let mut loader = DynamicModuleLoader::new();
//!
//! // Cargar módulo dinámico
//! # #[cfg(not(target_os = "android"))]
//! loader.load_library("target/release/libmi_modulo.so").unwrap();
//!
//! // Listar módulos
//! for name in loader.list_modules() {
//!     println!("Módulo cargado: {}", name);
//! }
//!
//! // Hot reload
//! # #[cfg(not(target_os = "android"))]
//! loader.reload("mi_modulo").unwrap();
//! ```

use rydit_core::{ModuleError, ModuleMetadata, ModuleRegistry};
use std::collections::HashMap;

#[cfg(not(target_os = "android"))]
use std::ffi::OsStr;

/// Resultado de operación de loader
pub type LoaderResult = Result<(), LoaderError>;

/// Error de loader
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

/// Información de un módulo cargado dinámicamente
#[derive(Clone)]
pub struct LoadedModuleInfo {
    /// Nombre del módulo
    pub name: String,
    /// Ruta del archivo cargado
    pub path: String,
    /// Metadata del módulo
    pub metadata: ModuleMetadata,
    /// Timestamp de última carga
    pub loaded_at: u64,
}

/// Cargador dinámico de módulos (v0.8.2+)
///
/// Soporta:
/// - Carga de bibliotecas dinámicas (.so/.dll/.dylib)
/// - Hot reload de módulos
/// - Descarga limpia con hooks
///
/// # Ejemplo
/// ```rust
/// use rydit_loader::DynamicModuleLoader;
///
/// let mut loader = DynamicModuleLoader::new();
///
/// // En plataformas con soporte (Linux, Windows, macOS)
/// # #[cfg(not(target_os = "android"))]
/// loader.load_library("libmi_modulo.so").unwrap();
///
/// // Listar módulos cargados
/// for name in loader.list_modules() {
///     println!("{}", name);
/// }
/// ```
pub struct DynamicModuleLoader {
    /// Registro de módulos
    registry: ModuleRegistry,
    /// Información de módulos cargados
    loaded_modules: HashMap<String, LoadedModuleInfo>,
    /// Timestamp actual (para tracking)
    timestamp: u64,
}

impl DynamicModuleLoader {
    /// Crea un nuevo loader vacío
    pub fn new() -> Self {
        Self {
            registry: ModuleRegistry::new(),
            loaded_modules: HashMap::new(),
            timestamp: 0,
        }
    }

    /// Obtiene el registro de módulos
    pub fn registry(&self) -> &ModuleRegistry {
        &self.registry
    }

    /// Obtiene el registro de módulos mutable
    pub fn registry_mut(&mut self) -> &mut ModuleRegistry {
        &mut self.registry
    }

    /// Carga una biblioteca dinámica (.so/.dll/.dylib)
    ///
    /// # Platform Support
    /// - ✅ Linux: `.so`
    /// - ✅ Windows: `.dll`
    /// - ✅ macOS: `.dylib`
    /// - ⚠️ Android: No soportado (requiere permisos especiales)
    ///
    /// # Seguridad
    /// La biblioteca debe exportar una función `create_module` que retorne
    /// un puntero a un objeto que implemente `RyditModule`.
    #[cfg(not(target_os = "android"))]
    pub fn load_library<P: AsRef<OsStr>>(&mut self, path: P) -> Result<&str, LoaderError> {
        use libloading::Library;
        use std::os::unix::ffi::OsStrExt;

        self.timestamp += 1;

        // Cargar biblioteca
        let lib = unsafe { Library::new(path.as_ref()) }
            .map_err(|e| LoaderError {
                code: "LOAD_ERROR".to_string(),
                message: format!("No se pudo cargar la biblioteca: {}", e),
            })?;

        // Buscar símbolo `create_module`
        let create_module: libloading::Symbol<unsafe extern "C" fn() -> *mut dyn RyditModule> =
            unsafe { lib.get(b"create_module") }
                .map_err(|e| LoaderError {
                    code: "SYMBOL_NOT_FOUND".to_string(),
                    message: format!("Símbolo 'create_module' no encontrado: {}", e),
                })?;

        // Crear instancia del módulo
        let module_ptr = create_module();
        let module = unsafe { Box::from_raw(module_ptr) };
        let module_name = module.name().to_string();
        let module_metadata = module.metadata();

        // Registrar módulo
        self.registry.register(module);

        // Guardar información
        let info = LoadedModuleInfo {
            name: module_name.clone(),
            path: path.as_ref().to_string_lossy().to_string(),
            metadata: module_metadata,
            loaded_at: self.timestamp,
        };
        self.loaded_modules.insert(module_name.clone(), info);

        Ok(&module_name)
    }

    /// Recarga un módulo (hot reload)
    ///
    /// Llama al hook `on_reload()` del módulo.
    ///
    /// # Nota
    /// En implementaciones futuras, esto descargará y volverá a cargar
    /// la biblioteca dinámica desde disco.
    pub fn reload(&mut self, name: &str) -> LoaderResult {
        if !self.registry.contains(name) {
            return Err(LoaderError {
                code: "MODULE_NOT_FOUND".to_string(),
                message: format!("Módulo '{}' no encontrado", name),
            });
        }

        self.registry.reload(name);

        // Actualizar timestamp
        if let Some(info) = self.loaded_modules.get_mut(name) {
            info.loaded_at = self.timestamp + 1;
            self.timestamp += 1;
        }

        Ok(())
    }

    /// Descarga un módulo
    ///
    /// Llama al hook `on_unload()` y remueve el módulo del registro.
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

    /// Lista todos los módulos cargados
    pub fn list_modules(&self) -> Vec<&str> {
        self.loaded_modules.keys().map(|s| s.as_str()).collect()
    }

    /// Obtiene información de un módulo cargado
    pub fn get_module_info(&self, name: &str) -> Option<&LoadedModuleInfo> {
        self.loaded_modules.get(name)
    }

    /// Obtiene la metadata de todos los módulos
    pub fn list_with_metadata(&self) -> Vec<(&str, ModuleMetadata)> {
        self.registry.list_with_metadata()
    }

    /// Verifica si un módulo está cargado
    pub fn is_loaded(&self, name: &str) -> bool {
        self.loaded_modules.contains_key(name)
    }

    /// Obtiene el número de módulos cargados
    pub fn len(&self) -> usize {
        self.loaded_modules.len()
    }

    /// Verifica si el loader está vacío
    pub fn is_empty(&self) -> bool {
        self.loaded_modules.is_empty()
    }

    /// Obtiene el timestamp actual
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

impl Default for DynamicModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rydit_core::{RyditModule, ModuleResult};

    // Módulo de prueba
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
                .with_name("test_module")
                .with_version("1.0.0")
                .with_description("Módulo de prueba")
        }
    }

    #[test]
    fn test_loader_new() {
        let loader = DynamicModuleLoader::new();
        assert!(loader.is_empty());
        assert_eq!(loader.len(), 0);
    }

    #[test]
    fn test_loader_register_module() {
        let mut loader = DynamicModuleLoader::new();

        // Simular registro manual (sin carga dinámica)
        loader.registry_mut().register(TestModule);

        // Agregar a loaded_modules manualmente para testing
        loader.loaded_modules.insert(
            "test_module".to_string(),
            LoadedModuleInfo {
                name: "test_module".to_string(),
                path: "test.so".to_string(),
                metadata: ModuleMetadata::new(),
                loaded_at: 1,
            },
        );

        assert_eq!(loader.len(), 1);
        assert!(loader.is_loaded("test_module"));
        assert!(!loader.is_loaded("nonexistent"));
    }

    #[test]
    fn test_loader_list_modules() {
        let mut loader = DynamicModuleLoader::new();

        loader.loaded_modules.insert(
            "mod1".to_string(),
            LoadedModuleInfo {
                name: "mod1".to_string(),
                path: "mod1.so".to_string(),
                metadata: ModuleMetadata::new(),
                loaded_at: 1,
            },
        );
        loader.loaded_modules.insert(
            "mod2".to_string(),
            LoadedModuleInfo {
                name: "mod2".to_string(),
                path: "mod2.so".to_string(),
                metadata: ModuleMetadata::new(),
                loaded_at: 2,
            },
        );

        let modules = loader.list_modules();
        assert_eq!(modules.len(), 2);
        assert!(modules.contains(&"mod1"));
        assert!(modules.contains(&"mod2"));
    }

    #[test]
    fn test_loader_reload() {
        let mut loader = DynamicModuleLoader::new();

        loader.registry_mut().register(TestModule);
        loader.loaded_modules.insert(
            "test_module".to_string(),
            LoadedModuleInfo {
                name: "test_module".to_string(),
                path: "test.so".to_string(),
                metadata: ModuleMetadata::new(),
                loaded_at: 1,
            },
        );

        let old_timestamp = loader.timestamp();
        loader.reload("test_module").unwrap();

        // El timestamp debería haber aumentado
        assert!(loader.timestamp() > old_timestamp);
    }

    #[test]
    fn test_loader_unload() {
        let mut loader = DynamicModuleLoader::new();

        loader.registry_mut().register(TestModule);
        loader.loaded_modules.insert(
            "test_module".to_string(),
            LoadedModuleInfo {
                name: "test_module".to_string(),
                path: "test.so".to_string(),
                metadata: ModuleMetadata::new(),
                loaded_at: 1,
            },
        );

        assert!(loader.is_loaded("test_module"));
        loader.unload("test_module").unwrap();
        assert!(!loader.is_loaded("test_module"));
    }

    #[test]
    fn test_loader_get_module_info() {
        let mut loader = DynamicModuleLoader::new();

        let info = LoadedModuleInfo {
            name: "test_module".to_string(),
            path: "test.so".to_string(),
            metadata: ModuleMetadata::new()
                .with_name("test_module")
                .with_version("1.0.0"),
            loaded_at: 1,
        };

        loader.loaded_modules.insert("test_module".to_string(), info.clone());

        let retrieved_info = loader.get_module_info("test_module").unwrap();
        assert_eq!(retrieved_info.name, info.name);
        assert_eq!(retrieved_info.path, info.path);
    }
}
