// ry-config - Config parser for Ry-Dit
// Parsea archivos .rydit como configuración (entidades, niveles, checkpoints)
// Sin dependencias externas - pure Rust

use std::collections::HashMap;
use std::fs;

// ============================================================================
// ESTRUCTURAS DE CONFIGURACIÓN
// ============================================================================

#[derive(Debug, Clone)]
pub struct EntityConfig {
    #[allow(dead_code)]
    pub id: String,
    pub tipo: String,
    pub sprite: String,
    pub x: f32,
    pub y: f32,
    pub ancho: f32,
    pub alto: f32,
    pub propiedades: HashMap<String, ValorConfig>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ValorConfig {
    Texto(String),
    Numero(f32),
    Bool(bool),
    Array(Vec<f32>),
}

#[derive(Debug, Clone)]
pub struct NivelConfig {
    pub nombre: String,
    pub gravedad: f32,
    pub fondo: String,
    pub musica: String,
    pub entidades: Vec<EntityConfig>,
    pub checkpoints: HashMap<String, (f32, f32)>,
}

// ============================================================================
// PARSER
// ============================================================================

pub struct ConfigParser;

impl ConfigParser {
    /// Parsear archivo de configuración
    pub fn parse(ruta: &str) -> Result<NivelConfig, String> {
        let contenido =
            fs::read_to_string(ruta).map_err(|e| format!("Error leyendo '{}': {}", ruta, e))?;

        Self::parse_contenido(&contenido)
    }

    /// Parsear contenido de configuración
    pub fn parse_contenido(contenido: &str) -> Result<NivelConfig, String> {
        let mut config = NivelConfig {
            nombre: String::new(),
            gravedad: 9.8,
            fondo: String::new(),
            musica: String::new(),
            entidades: Vec::new(),
            checkpoints: HashMap::new(),
        };

        let mut entidad_actual: Option<EntityConfig> = None;
        let mut bloque_actual = String::new();

        for linea in contenido.lines() {
            let linea = linea.trim();

            if linea.is_empty() || linea.starts_with('#') {
                continue;
            }

            if linea.starts_with("@nombre") {
                config.nombre = Self::extraer_texto(linea, "@nombre")?;
            } else if linea.starts_with("@descripcion") {
                // Ignorar
            } else if linea.starts_with("@autor") {
                // Ignorar
            } else if linea.starts_with("@version") {
                // Ignorar
            } else if linea.contains("gravedad:") {
                config.gravedad = Self::extraer_numero(linea, "gravedad:")?;
            } else if linea.contains("fondo:") {
                config.fondo = Self::extraer_texto(linea, "fondo:")?;
            } else if linea.contains("musica:") {
                config.musica = Self::extraer_texto(linea, "musica:")?;
            } else if linea.starts_with("entidad") {
                if let Some(ent) = entidad_actual.take() {
                    config.entidades.push(ent);
                }
                let id = Self::extraer_id(linea)?;
                entidad_actual = Some(EntityConfig {
                    id: id.clone(),
                    tipo: String::new(),
                    sprite: String::new(),
                    x: 0.0,
                    y: 0.0,
                    ancho: 32.0,
                    alto: 32.0,
                    propiedades: HashMap::new(),
                });
                bloque_actual = "entidad".to_string();
            } else if !bloque_actual.is_empty() {
                if let Some(ref mut ent) = entidad_actual {
                    if linea.contains("tipo:") {
                        ent.tipo = Self::extraer_texto(linea, "tipo:")?;
                    } else if linea.contains("sprite:") {
                        ent.sprite = Self::extraer_texto(linea, "sprite:")?;
                    } else if linea.contains("x:") {
                        ent.x = Self::extraer_numero(linea, "x:")?;
                    } else if linea.contains("y:") {
                        ent.y = Self::extraer_numero(linea, "y:")?;
                    } else if linea.contains("ancho:") {
                        ent.ancho = Self::extraer_numero(linea, "ancho:")?;
                    } else if linea.contains("alto:") {
                        ent.alto = Self::extraer_numero(linea, "alto:")?;
                    } else if linea.contains("vida:") {
                        ent.propiedades.insert(
                            "vida".to_string(),
                            ValorConfig::Numero(Self::extraer_numero(linea, "vida:")?),
                        );
                    } else if linea.contains("daño:") {
                        ent.propiedades.insert(
                            "daño".to_string(),
                            ValorConfig::Numero(Self::extraer_numero(linea, "daño:")?),
                        );
                    } else if linea.contains("velocidad:") {
                        ent.propiedades.insert(
                            "velocidad".to_string(),
                            ValorConfig::Numero(Self::extraer_numero(linea, "velocidad:")?),
                        );
                    } else if linea.contains("estatica:") {
                        let val = Self::extraer_texto(linea, "estatica:")?;
                        ent.propiedades
                            .insert("estatica".to_string(), ValorConfig::Bool(val == "true"));
                    }
                    if linea == "}" {
                        config.entidades.push(ent.clone());
                        entidad_actual = None;
                        bloque_actual = String::new();
                    }
                }
            } else if linea.starts_with("checkpoint") {
                let _id = Self::extraer_id(linea)?;
                bloque_actual = "checkpoint".to_string();
            } else if bloque_actual == "checkpoint" {
                if linea.contains("x:") && linea.contains("y:") {
                    let partes: Vec<&str> = linea.split(',').collect();
                    if partes.len() == 2 {
                        let x = Self::extraer_numero(partes[0], "x:")?;
                        let y = Self::extraer_numero(partes[1], "y:")?;
                        config.checkpoints.insert(bloque_actual.clone(), (x, y));
                    }
                } else if linea.contains("x:") {
                    let x = Self::extraer_numero(linea, "x:")?;
                    config.checkpoints.insert(bloque_actual.clone(), (x, 0.0));
                } else if linea.contains("y:") {
                    let y = Self::extraer_numero(linea, "y:")?;
                    if let Some(val) = config.checkpoints.get_mut(&bloque_actual) {
                        val.1 = y;
                    }
                }
                if linea == "}" {
                    bloque_actual = String::new();
                }
            }
        }

        if let Some(ent) = entidad_actual {
            config.entidades.push(ent);
        }

        Ok(config)
    }

    // ========================================================================
    // HELPERS
    // ========================================================================

    pub fn extraer_texto(linea: &str, clave: &str) -> Result<String, String> {
        let partes: Vec<&str> = linea.splitn(2, clave).collect();
        if partes.len() < 2 {
            return Err(format!("No se encontró '{}' en: {}", clave, linea));
        }
        let valor = partes[1].trim().trim_matches('"').trim_matches('\'');
        Ok(valor.to_string())
    }

    pub fn extraer_numero(linea: &str, clave: &str) -> Result<f32, String> {
        let texto = Self::extraer_texto(linea, clave)?;
        texto
            .parse::<f32>()
            .map_err(|e| format!("Error parseando número '{}': {}", texto, e))
    }

    pub fn extraer_id(linea: &str) -> Result<String, String> {
        let inicio = linea.find('"').ok_or("No se encontró '\"' en la línea")?;
        let fin = linea[inicio + 1..]
            .find('"')
            .ok_or("No se encontró cierre '\"'")?;
        Ok(linea[inicio + 1..inicio + 1 + fin].to_string())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extraer_texto() {
        let resultado = ConfigParser::extraer_texto("sprite: \"hero.png\"", "sprite:").unwrap();
        assert_eq!(resultado, "hero.png");
    }

    #[test]
    fn test_extraer_numero() {
        let resultado = ConfigParser::extraer_numero("x: 100.5", "x:").unwrap();
        assert!((resultado - 100.5).abs() < 0.01);
    }

    #[test]
    fn test_extraer_id() {
        let resultado = ConfigParser::extraer_id("entidad \"jugador\" {").unwrap();
        assert_eq!(resultado, "jugador");
    }
}
