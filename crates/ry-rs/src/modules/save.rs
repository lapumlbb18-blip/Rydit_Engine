// crates/rydit-rs/src/modules/save.rs
// Save/Load System - Guardar y cargar progreso de juegos
// v0.13.1 - JSON serialización con std::fs

use blast_core::{Executor, Valor};
use ry_parser::{Expr, Stmt};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::eval::evaluar_expr;

// ============================================================================
// SAVE DATA STRUCT
// ============================================================================

/// Datos de un slot de guardado
#[derive(Debug, Clone)]
pub struct SaveData {
    pub slot: String,
    pub timestamp: String,
    pub level: String,
    pub player_x: f64,
    pub player_y: f64,
    pub variables: HashMap<String, String>, // serializado como JSON strings
    pub metadata: HashMap<String, String>,
}

impl SaveData {
    pub fn new(slot: &str) -> Self {
        let now = chrono_timestamp();
        Self {
            slot: slot.to_string(),
            timestamp: now,
            level: "default".to_string(),
            player_x: 0.0,
            player_y: 0.0,
            variables: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn to_json(&self) -> Result<String, String> {
        let vars_json: Vec<String> = self.variables.iter()
            .map(|(k, v)| format!("\"{}\":{}", escape_json(k), v))
            .collect();
        let meta_json: Vec<String> = self.metadata.iter()
            .map(|(k, v)| format!("\"{}\":\"{}\"", escape_json(k), escape_json(v)))
            .collect();

        Ok(format!(
            r#"{{"slot":"{}","timestamp":"{}","level":"{}","player_x":{},"player_y":{},"variables":{{{}}},"metadata":{{{}}}}}"#,
            escape_json(&self.slot),
            escape_json(&self.timestamp),
            escape_json(&self.level),
            self.player_x,
            self.player_y,
            vars_json.join(","),
            meta_json.join(",")
        ))
    }

    pub fn from_json(json: &str) -> Result<Self, String> {
        // Parser JSON simple - para uso interno
        let mut data = SaveData::new("temp");
        
        // Extraer campos simples con string matching
        if let Some(v) = extract_string_field(json, "slot") { data.slot = v; }
        if let Some(v) = extract_string_field(json, "timestamp") { data.timestamp = v; }
        if let Some(v) = extract_string_field(json, "level") { data.level = v; }
        if let Some(v) = extract_number_field(json, "player_x") { data.player_x = v; }
        if let Some(v) = extract_number_field(json, "player_y") { data.player_y = v; }

        // Extraer variables (simplificado)
        if let Some(vars_str) = extract_object(json, "variables") {
            for (k, v) in parse_flat_json_object(&vars_str) {
                data.variables.insert(k, v);
            }
        }
        if let Some(meta_str) = extract_object(json, "metadata") {
            for (k, v) in parse_flat_json_object(&meta_str) {
                data.metadata.insert(k, v);
            }
        }

        Ok(data)
    }
}

fn chrono_timestamp() -> String {
    // Simple timestamp sin dependencias externas
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{}", now)
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('"', "\\\"")
     .replace('\n', "\\n")
     .replace('\t', "\\t")
}

fn extract_string_field<'a>(json: &'a str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\":\"", key);
    if let Some(start) = json.find(&pattern) {
        let start = start + pattern.len();
        if let Some(end) = json[start..].find('"') {
            return Some(json[start..start+end].replace("\\\"", "\"").replace("\\\\", "\\"));
        }
    }
    None
}

fn extract_number_field(json: &str, key: &str) -> Option<f64> {
    let pattern = format!("\"{}\":", key);
    if let Some(start) = json.find(&pattern) {
        let start = start + pattern.len();
        let rest = &json[start..];
        let end = rest.find(|c: char| c == ',' || c == '}' || c == '\n').unwrap_or(rest.len());
        if let Ok(n) = rest[..end].trim().parse::<f64>() {
            return Some(n);
        }
    }
    None
}

fn extract_object<'a>(json: &'a str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\":{{", key);
    if let Some(start) = json.find(&pattern) {
        let start = start + pattern.len() - 1;
        let mut depth = 0;
        for (i, c) in json[start..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(json[start..start+i+1].to_string());
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn parse_flat_json_object(json: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let content = json.trim().trim_start_matches('{').trim_end_matches('}');
    
    let mut in_string = false;
    let mut escape_next = false;
    let mut key_start = 0;
    let mut value_start = 0;
    let mut current_key = String::new();
    let mut current_value = String::new();
    let mut parsing_key = true;
    
    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        
        if escape_next {
            escape_next = false;
            if in_string {
                if parsing_key { current_key.push(c); } else { current_value.push(c); }
            }
            i += 1;
            continue;
        }
        
        if c == '\\' && in_string {
            escape_next = true;
            i += 1;
            continue;
        }
        
        if c == '"' {
            if in_string {
                // Fin de string
                if parsing_key && !current_key.is_empty() {
                    parsing_key = false;
                    // Saltar hasta ':'
                    while i + 1 < chars.len() && chars[i + 1] != ':' { i += 1; }
                    i += 2; // saltar ":
                    value_start = i;
                    continue;
                } else if !parsing_key && !current_value.is_empty() {
                    current_value.pop(); // remove trailing quote
                    map.insert(current_key.clone(), current_value.clone());
                    current_key.clear();
                    current_value.clear();
                    parsing_key = true;
                    in_string = false;
                    // Saltar hasta siguiente key o fin
                    while i + 1 < chars.len() && chars[i + 1] != '"' && chars[i + 1] != '}' { i += 1; }
                    i += 1;
                    continue;
                }
            }
            in_string = !in_string;
        } else if in_string {
            if parsing_key { current_key.push(c); } else { current_value.push(c); }
        }
        
        i += 1;
    }
    
    map
}

// ============================================================================
// SAVE MANAGER
// ============================================================================

fn get_save_path(slot: &str) -> String {
    format!("saves/{}.rysave", slot)
}

pub fn save_create<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() < 1 {
        return Valor::Error("save::create(slot) requiere 1 argumento".to_string());
    }

    let slot = evaluar_expr(&args[0], executor, funcs);
    if let Valor::Texto(slot) = slot {
        // Crear directorio saves si no existe
        let _ = fs::create_dir_all("saves");
        
        let path = get_save_path(&slot);
        if Path::new(&path).exists() {
            Valor::Texto(format!("Slot '{}' ya existe, usa save::overwrite", slot))
        } else {
            let data = SaveData::new(&slot);
            match fs::write(&path, data.to_json().unwrap_or_default()) {
                Ok(_) => Valor::Texto(format!("Slot '{}' creado", slot)),
                Err(e) => Valor::Error(format!("Error creando slot '{}': {}", slot, e))
            }
        }
    } else {
        Valor::Error("save::create requiere texto (slot)".to_string())
    }
}

pub fn save_overwrite<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() < 1 {
        return Valor::Error("save::overwrite(slot) requiere 1 argumento".to_string());
    }

    let slot = evaluar_expr(&args[0], executor, funcs);
    if let Valor::Texto(slot) = slot {
        let _ = fs::create_dir_all("saves");
        let path = get_save_path(&slot);
        let data = SaveData::new(&slot);
        match fs::write(&path, data.to_json().unwrap_or_default()) {
            Ok(_) => Valor::Texto(format!("Slot '{}' guardado", slot)),
            Err(e) => Valor::Error(format!("Error guardando slot '{}': {}", slot, e))
        }
    } else {
        Valor::Error("save::overwrite requiere texto (slot)".to_string())
    }
}

pub fn save_set_var<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() < 3 {
        return Valor::Error("save::set_var(slot, key, value) requiere 3 argumentos".to_string());
    }

    let slot = evaluar_expr(&args[0], executor, funcs);
    let key = evaluar_expr(&args[1], executor, funcs);
    let value = evaluar_expr(&args[2], executor, funcs);

    if let (Valor::Texto(slot), Valor::Texto(key)) = (slot, key) {
        let path = get_save_path(&slot);
        
        // Leer datos existentes
        let mut data = if Path::new(&path).exists() {
            match fs::read_to_string(&path) {
                Ok(json) => SaveData::from_json(&json).unwrap_or_else(|_| SaveData::new(&slot)),
                Err(_) => SaveData::new(&slot),
            }
        } else {
            SaveData::new(&slot)
        };

        // Set variable
        let value_str = match &value {
            Valor::Num(n) => format!("{}", n),
            Valor::Texto(t) => format!("\"{}\"", escape_json(t)),
            Valor::Bool(b) => format!("{}", b),
            _ => "\"\"".to_string(),
        };
        let value_str_clone = value_str.clone();
        data.variables.insert(key.clone(), value_str_clone);

        match fs::write(&path, data.to_json().unwrap_or_default()) {
            Ok(_) => Valor::Texto(format!("{}={} guardado en slot '{}'", key, value_str, slot)),
            Err(e) => Valor::Error(format!("Error guardando variable: {}", e))
        }
    } else {
        Valor::Error("save::set_var requiere (slot, key, value)".to_string())
    }
}

pub fn save_get_var<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() < 2 {
        return Valor::Error("save::get_var(slot, key) requiere 2 argumentos".to_string());
    }

    let slot = evaluar_expr(&args[0], executor, funcs);
    let key = evaluar_expr(&args[1], executor, funcs);

    if let (Valor::Texto(slot), Valor::Texto(key)) = (slot, key) {
        let path = get_save_path(&slot);
        if !Path::new(&path).exists() {
            return Valor::Error(format!("Slot '{}' no existe", slot));
        }

        match fs::read_to_string(&path) {
            Ok(json) => {
                match SaveData::from_json(&json) {
                    Ok(data) => {
                        if let Some(val) = data.variables.get(&key) {
                            // Intentar parsear como número
                            if let Ok(n) = val.parse::<f64>() {
                                Valor::Num(n)
                            } else {
                                // Remover quotes si es string
                                let s = val.trim_matches('"');
                                Valor::Texto(s.to_string())
                            }
                        } else {
                            Valor::Vacio
                        }
                    }
                    Err(_) => Valor::Error(format!("Error parseando slot '{}'", slot)),
                }
            }
            Err(e) => Valor::Error(format!("Error leyendo slot '{}': {}", slot, e)),
        }
    } else {
        Valor::Error("save::get_var requiere (slot, key)".to_string())
    }
}

pub fn save_set_player_pos<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() < 3 {
        return Valor::Error("save::set_player_pos(slot, x, y) requiere 3 argumentos".to_string());
    }

    let slot = evaluar_expr(&args[0], executor, funcs);
    let x = evaluar_expr(&args[1], executor, funcs);
    let y = evaluar_expr(&args[2], executor, funcs);

    if let (Valor::Texto(slot), Valor::Num(x), Valor::Num(y)) = (slot, x, y) {
        let path = get_save_path(&slot);
        
        let mut data = if Path::new(&path).exists() {
            match fs::read_to_string(&path) {
                Ok(json) => SaveData::from_json(&json).unwrap_or_else(|_| SaveData::new(&slot)),
                Err(_) => SaveData::new(&slot),
            }
        } else {
            SaveData::new(&slot)
        };

        data.player_x = x;
        data.player_y = y;

        match fs::write(&path, data.to_json().unwrap_or_default()) {
            Ok(_) => Valor::Texto(format!("Posición ({}, {}) guardada en '{}'", x, y, slot)),
            Err(e) => Valor::Error(format!("Error guardando posición: {}", e)),
        }
    } else {
        Valor::Error("save::set_player_pos requiere (slot, x, y)".to_string())
    }
}

pub fn save_load<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() < 1 {
        return Valor::Error("save::load(slot) requiere 1 argumento".to_string());
    }

    let slot = evaluar_expr(&args[0], executor, funcs);
    if let Valor::Texto(slot) = slot {
        let path = get_save_path(&slot);
        if !Path::new(&path).exists() {
            return Valor::Error(format!("Slot '{}' no existe", slot));
        }

        match fs::read_to_string(&path) {
            Ok(json) => {
                match SaveData::from_json(&json) {
                    Ok(data) => {
                        // Guardar variables en el executor
                        for (k, v) in &data.variables {
                            if let Ok(n) = v.parse::<f64>() {
                                executor.guardar(k, Valor::Num(n));
                            } else {
                                let s = v.trim_matches('"');
                                executor.guardar(k, Valor::Texto(s.to_string()));
                            }
                        }

                        let info = format!(
                            "Slot '{}' cargado: player=({}, {}) timestamp={}",
                            slot, data.player_x, data.player_y, data.timestamp
                        );
                        Valor::Texto(info)
                    }
                    Err(e) => Valor::Error(format!("Error parseando slot '{}': {}", slot, e)),
                }
            }
            Err(e) => Valor::Error(format!("Error leyendo slot '{}': {}", slot, e)),
        }
    } else {
        Valor::Error("save::load requiere texto (slot)".to_string())
    }
}

pub fn save_get_player_pos<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() < 1 {
        return Valor::Error("save::get_player_pos(slot) requiere 1 argumento".to_string());
    }

    let slot = evaluar_expr(&args[0], executor, funcs);
    if let Valor::Texto(slot) = slot {
        let path = get_save_path(&slot);
        if !Path::new(&path).exists() {
            return Valor::Error(format!("Slot '{}' no existe", slot));
        }

        match fs::read_to_string(&path) {
            Ok(json) => {
                match SaveData::from_json(&json) {
                    Ok(data) => {
                        let result = vec![
                            Valor::Num(data.player_x),
                            Valor::Num(data.player_y),
                        ];
                        Valor::Array(result)
                    }
                    Err(_) => Valor::Error(format!("Error parseando slot '{}'", slot)),
                }
            }
            Err(e) => Valor::Error(format!("Error leyendo slot '{}': {}", slot, e)),
        }
    } else {
        Valor::Error("save::get_player_pos requiere texto (slot)".to_string())
    }
}

pub fn save_list<'a>(
    _args: &[Expr<'a>],
    _executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    // Listar todos los slots guardados
    let mut slots = Vec::new();
    
    if let Ok(entries) = fs::read_dir("saves") {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".rysave") {
                    let slot_name = name.trim_end_matches(".rysave");
                    slots.push(Valor::Texto(slot_name.to_string()));
                }
            }
        }
    }
    
    Valor::Array(slots)
}

pub fn save_delete<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() < 1 {
        return Valor::Error("save::delete(slot) requiere 1 argumento".to_string());
    }

    let slot = evaluar_expr(&args[0], executor, funcs);
    if let Valor::Texto(slot) = slot {
        let path = get_save_path(&slot);
        if Path::new(&path).exists() {
            match fs::remove_file(&path) {
                Ok(_) => Valor::Texto(format!("Slot '{}' eliminado", slot)),
                Err(e) => Valor::Error(format!("Error eliminando slot '{}': {}", slot, e)),
            }
        } else {
            Valor::Error(format!("Slot '{}' no existe", slot))
        }
    } else {
        Valor::Error("save::delete requiere texto (slot)".to_string())
    }
}

pub fn save_exists<'a>(
    args: &[Expr<'a>],
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt<'a>>)>,
) -> Valor {
    if args.len() < 1 {
        return Valor::Error("save::exists(slot) requiere 1 argumento".to_string());
    }

    let slot = evaluar_expr(&args[0], executor, funcs);
    if let Valor::Texto(slot) = slot {
        let path = get_save_path(&slot);
        Valor::Bool(Path::new(&path).exists())
    } else {
        Valor::Error("save::exists requiere texto (slot)".to_string())
    }
}
