// crates/rydit-rs/src/module_loader.rs
// Comandos module::* para carga dinámica y hot reload de módulos (v0.8.2)

use blast_core::{Executor, Valor};
use lizer::{Expr, Stmt};
use rydit_loader::DynamicModuleLoader;
use std::collections::HashMap;

/// Comandos del namespace module::*
///
/// # Comandos disponibles
/// - module::load(path) → Carga un módulo dinámico (.so/.dll)
/// - module::reload(name) → Recarga un módulo (hot reload)
/// - module::unload(name) → Descarga un módulo
/// - module::list() → Lista módulos cargados
/// - module::info(name) → Información de un módulo
pub fn ejecutar_comando_module(
    name: &str,
    args: &[Expr],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
    loader: &mut DynamicModuleLoader,
) -> Valor {
    match name {
        // ====================================================================
        # [cfg(not(target_os = "android"))]
        "module::load" => {
            if args.len() != 1 {
                return Valor::Error("module::load requiere 1 argumento: path".to_string());
            }

            let path_val = crate::evaluar_expr(&args[0], executor, funcs);
            if let Valor::Texto(path) = path_val {
                match loader.load_library(&path) {
                    Ok(module_name) => {
                        Valor::Texto(format!("✅ Módulo '{}' cargado exitosamente", module_name))
                    }
                    Err(e) => {
                        Valor::Error(format!("❌ Error cargando módulo: {}", e))
                    }
                }
            } else {
                Valor::Error("module::load requiere una ruta (texto)".to_string())
            }
        }

        // ====================================================================
        "module::reload" => {
            if args.len() != 1 {
                return Valor::Error("module::reload requiere 1 argumento: name".to_string());
            }

            let name_val = crate::evaluar_expr(&args[0], executor, funcs);
            if let Valor::Texto(mod_name) = name_val {
                match loader.reload(&mod_name) {
                    Ok(_) => {
                        Valor::Texto(format!("✅ Módulo '{}' recargado", mod_name))
                    }
                    Err(e) => {
                        Valor::Error(format!("❌ Error recargando módulo: {}", e))
                    }
                }
            } else {
                Valor::Error("module::reload requiere nombre (texto)".to_string())
            }
        }

        // ====================================================================
        "module::unload" => {
            if args.len() != 1 {
                return Valor::Error("module::unload requiere 1 argumento: name".to_string());
            }

            let name_val = crate::evaluar_expr(&args[0], executor, funcs);
            if let Valor::Texto(mod_name) = name_val {
                match loader.unload(&mod_name) {
                    Ok(_) => {
                        Valor::Texto(format!("✅ Módulo '{}' descargado", mod_name))
                    }
                    Err(e) => {
                        Valor::Error(format!("❌ Error descargando módulo: {}", e))
                    }
                }
            } else {
                Valor::Error("module::unload requiere nombre (texto)".to_string())
            }
        }

        // ====================================================================
        "module::list" => {
            let modules = loader.list_modules();
            if modules.is_empty() {
                return Valor::Texto("No hay módulos cargados".to_string());
            }

            let module_list = modules
                .iter()
                .map(|m| format!("  - {}", m))
                .collect::<Vec<_>>()
                .join("\n");

            Valor::Texto(format!("Módulos cargados:\n{}", module_list))
        }

        // ====================================================================
        "module::info" => {
            if args.len() != 1 {
                return Valor::Error("module::info requiere 1 argumento: name".to_string());
            }

            let name_val = crate::evaluar_expr(&args[0], executor, funcs);
            if let Valor::Texto(mod_name) = name_val {
                if let Some(info) = loader.get_module_info(&mod_name) {
                    let info_text = format!(
                        "Módulo: {}\nVersión: {}\nRuta: {}\nCargado en: {}",
                        info.name,
                        info.metadata.version,
                        info.path,
                        info.loaded_at
                    );
                    Valor::Texto(info_text)
                } else {
                    // Buscar en registry sin info de path
                    let registry = loader.registry();
                    if registry.contains(&mod_name) {
                        let module = registry.get(&mod_name).unwrap();
                        let metadata = module.metadata();
                        let info_text = format!(
                            "Módulo: {}\nVersión: {}\nDescripción: {}\nLicencia: {}",
                            metadata.name,
                            metadata.version,
                            metadata.description,
                            metadata.license
                        );
                        Valor::Texto(info_text)
                    } else {
                        Valor::Error(format!("Módulo '{}' no encontrado", mod_name))
                    }
                }
            } else {
                Valor::Error("module::info requiere nombre (texto)".to_string())
            }
        }

        // ====================================================================
        _ => {
            Valor::Error(format!("Comando desconocido: {}", name))
        }
    }
}

/// Inicializar el loader global
pub fn init_loader() -> DynamicModuleLoader {
    DynamicModuleLoader::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_loader() {
        let loader = init_loader();
        assert!(loader.is_empty());
        assert_eq!(loader.len(), 0);
    }
}
