// crates/ry-core/src/assets.rs
// Asset Pipeline Unificado para Ry-Dit
// v0.20.0 - Gestión centralizada de recursos con Hot Reload

use crate::{ModuleError, ModuleResult, RyditModule, ModuleMetadata};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::fs;

/// Tipos de recursos soportados
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum AssetType {
    Texture,
    Sound,
    Music,
    Font,
    Model,
    Data,
    SpriteAnimation, // 🆕 v0.20.0
    SpriteSheet,     // 🆕 v0.20.0
    Config,          // 🆕 v0.20.0
    Unknown,
}

/// Representación de un recurso cargado
#[derive(Debug, Clone)]
pub struct Asset {
    pub id: String,
    pub path: String,
    pub asset_type: AssetType,
    pub data: Vec<u8>,
    pub version: u32,
}

/// Servidor de recursos centralizado (Cache global)
pub struct AssetServer {
    assets: RwLock<HashMap<String, Arc<Asset>>>,
}

impl AssetServer {
    pub fn new() -> Self {
        Self {
            assets: RwLock::new(HashMap::new()),
        }
    }

    /// Cargar un recurso desde el disco
    pub fn load(&self, id: &str, path: &str, asset_type: AssetType) -> Result<Arc<Asset>, String> {
        let data = fs::read(path).map_err(|e| format!("Error leyendo {}: {}", path, e))?;
        
        let asset = Arc::new(Asset {
            id: id.to_string(),
            path: path.to_string(),
            asset_type,
            data,
            version: 1,
        });

        let mut assets = self.assets.write().unwrap();
        assets.insert(id.to_string(), Arc::clone(&asset));
        
        Ok(asset)
    }

    /// Obtener un recurso de la caché
    pub fn get(&self, id: &str) -> Option<Arc<Asset>> {
        let assets = self.assets.read().unwrap();
        assets.get(id).cloned()
    }

    /// Obtener un recurso deserializado en un tipo específico
    pub fn get_typed<T: serde::de::DeserializeOwned + 'static>(&self, id: &str) -> Result<T, String> {
        let assets = self.assets.read().unwrap();
        let asset = assets.get(id).ok_or_else(|| format!("Asset no encontrado: {}", id))?;
        serde_json::from_slice(&asset.data).map_err(|e| format!("Error deserializando asset {}: {}", id, e))
    }

    /// Recargar un recurso (Hot Reload)
    pub fn reload(&self, id: &str) -> Result<Arc<Asset>, String> {
        let (path, asset_type, version) = {
            let assets = self.assets.read().unwrap();
            let a = assets.get(id).ok_or_else(|| format!("Asset no encontrado: {}", id))?;
            (a.path.clone(), a.asset_type, a.version)
        };

        let data = fs::read(&path).map_err(|e| format!("Error recargando {}: {}", path, e))?;
        
        let asset = Arc::new(Asset {
            id: id.to_string(),
            path,
            asset_type,
            data,
            version: version + 1,
        });

        let mut assets = self.assets.write().unwrap();
        assets.insert(id.to_string(), Arc::clone(&asset));
        
        Ok(asset)
    }

    /// Listar todos los recursos cargados
    pub fn list(&self) -> Vec<String> {
        let assets = self.assets.read().unwrap();
        assets.keys().cloned().collect()
    }

    /// Eliminar un recurso de la caché
    pub fn unload(&self, id: &str) -> bool {
        let mut assets = self.assets.write().unwrap();
        assets.remove(id).is_some()
    }
}

// ============================================================================
// ASSET MODULE (RyditModule Implementation)
// ============================================================================

pub struct AssetModule {
    server: Arc<AssetServer>,
}

impl AssetModule {
    pub fn new(server: Arc<AssetServer>) -> Self {
        Self { server }
    }
}

impl RyditModule for AssetModule {
    fn name(&self) -> &'static str { "asset" }
    fn version(&self) -> &'static str { "0.20.0" }

    fn register(&self) -> HashMap<&'static str, &'static str> {
        let mut cmds = HashMap::new();
        cmds.insert("asset::load", "Carga un recurso (id, path, type)");
        cmds.insert("asset::get", "Obtiene info de un recurso (id)");
        cmds.insert("asset::reload", "Recarga un recurso desde disco (id)");
        cmds.insert("asset::unload", "Elimina un recurso de la caché (id)");
        cmds.insert("asset::list", "Lista todos los recursos cargados");
        cmds
    }

    fn execute(&self, command: &str, params: Value) -> ModuleResult {
        match command {
            "asset::load" => {
                let arr = params.as_array().ok_or_else(|| ModuleError {
                    code: "INVALID_PARAMS".to_string(),
                    message: "Uso: [id, path, type]".to_string(),
                })?;
                let id = arr.get(0).and_then(|v| v.as_str()).unwrap_or("");
                let path = arr.get(1).and_then(|v| v.as_str()).unwrap_or("");
                let type_str = arr.get(2).and_then(|v| v.as_str()).unwrap_or("data");
                
                let asset_type = match type_str.to_lowercase().as_str() {
                    "texture" => AssetType::Texture,
                    "sound" => AssetType::Sound,
                    "music" => AssetType::Music,
                    "font" => AssetType::Font,
                    "model" => AssetType::Model,
                    "spriteanimation" => AssetType::SpriteAnimation,
                    "spritesheet" => AssetType::SpriteSheet,
                    "config" => AssetType::Config,
                    _ => AssetType::Data,
                };

                match self.server.load(id, path, asset_type) {
                    Ok(a) => Ok(json!({ "id": a.id, "path": a.path, "version": a.version })),
                    Err(e) => Err(ModuleError { code: "LOAD_ERROR".to_string(), message: e }),
                }
            },
            "asset::get" => {
                let id = params.as_str().unwrap_or("");
                match self.server.as_ref().get(id) {
                    Some(a) => Ok(json!({ "id": a.id, "path": a.path, "type": a.asset_type, "version": a.version, "size": a.data.len() })),
                    None => Err(ModuleError { code: "NOT_FOUND".to_string(), message: "Asset no encontrado".to_string() }),
                }
            },
            "asset::reload" => {
                let id = params.as_str().unwrap_or("");
                match self.server.as_ref().reload(id) {
                    Ok(a) => Ok(json!({ "id": a.id, "path": a.path, "version": a.version })),
                    Err(e) => Err(ModuleError { code: "RELOAD_ERROR".to_string(), message: e }),
                }
            },
            "asset::unload" => {
                let id = params.as_str().unwrap_or("");
                Ok(json!(self.server.as_ref().unload(id)))
            },
            "asset::list" => {
                Ok(json!(self.server.as_ref().list()))
            },
            _ => Err(ModuleError {
                code: "UNKNOWN_COMMAND".to_string(),
                message: format!("Comando desconocido: {}", command),
            }),
        }
    }

    fn metadata(&self) -> ModuleMetadata {
        ModuleMetadata::new()
            .with_name("asset")
            .with_version("0.20.0")
            .with_description("Sistema centralizado de gestión de recursos (Asset Pipeline)")
    }
}
