// crates/rydit-rs/src/json_helpers.rs
// Helpers para conversión JSON (serde_json)

use blast_core::Valor;

/// Convertir serde_json::Value a Valor (Rydit)
pub fn valor_serde_a_rydit(val: &serde_json::Value) -> Valor {
    match val {
        serde_json::Value::Null => Valor::Vacio,
        serde_json::Value::Bool(b) => Valor::Bool(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Valor::Num(i as f64)
            } else if let Some(f) = n.as_f64() {
                Valor::Num(f)
            } else {
                Valor::Num(0.0)
            }
        }
        serde_json::Value::String(s) => Valor::Texto(s.clone()),
        serde_json::Value::Array(arr) => {
            let valores: Vec<Valor> = arr.iter().map(valor_serde_a_rydit).collect();
            Valor::Array(valores)
        }
        serde_json::Value::Object(obj) => {
            // Los objetos JSON los convertimos a array de pares [key, value]
            let pares: Vec<Valor> = obj
                .iter()
                .map(|(k, v)| Valor::Array(vec![Valor::Texto(k.clone()), valor_serde_a_rydit(v)]))
                .collect();
            Valor::Array(pares)
        }
    }
}

/// Convertir Valor (Rydit) a serde_json::Value
pub fn valor_rydit_a_serde(val: &Valor) -> Result<serde_json::Value, String> {
    match val {
        Valor::Num(n) => Ok(serde_json::Value::Number(
            serde_json::Number::from_f64(*n).unwrap_or(serde_json::Number::from(0)),
        )),
        Valor::Texto(s) => Ok(serde_json::Value::String(s.clone())),
        Valor::Bool(b) => Ok(serde_json::Value::Bool(*b)),
        Valor::Array(arr) => {
            let valores: Result<Vec<serde_json::Value>, _> =
                arr.iter().map(valor_rydit_a_serde).collect();
            Ok(serde_json::Value::Array(valores?))
        }
        Valor::Vacio => Ok(serde_json::Value::Null),
        Valor::Error(msg) => Err(format!("Valor de error: {}", msg)),
    }
}
