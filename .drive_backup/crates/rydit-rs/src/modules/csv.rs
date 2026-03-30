// crates/rydit-rs/src/modules/csv.rs
// Módulo CSV completo para RyDit - Data Science
//
// Funciones:
// - csv::read(path) - Leer CSV desde archivo
// - csv::write(data, path) - Escribir CSV a archivo
// - csv::to_json(csv_text) - Convertir CSV a JSON
// - csv::from_json(json_text) - Convertir JSON a CSV
// - csv::filter(data, column, value) - Filtrar filas
// - csv::map_column(data, column, fn_name) - Transformar columna
// - csv::columns(data) - Obtener nombres de columnas
// - csv::row_count(data) - Contar filas
// - csv::col_count(data) - Contar columnas
// - csv::join(csv1, csv2, column) - Unir CSVs
// - csv::group_by(data, column) - Agrupar datos
// - csv::aggregate(data, column, operation) - Agregar datos

use blast_core::Valor;
use std::collections::HashMap;
use std::fs;

// ============================================================================
// FUNCIONES BÁSICAS - FILE I/O
// ============================================================================

/// csv::read(path) - Leer CSV desde archivo
pub fn csv_read(
    args: &[lizer::Expr],
    executor: &mut blast_core::Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 1 {
        return Valor::Error("csv::read() requiere 1 argumento: path".to_string());
    }

    let path_val = evaluar_expr(&args[0], executor, funcs);
    let path = match path_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("csv::read() path debe ser texto".to_string()),
    };

    // Leer archivo
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => return Valor::Error(format!("csv::read() no pudo leer '{}': {}", path, e)),
    };

    // Parsear CSV con headers
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());

    let mut headers: Vec<String> = Vec::new();
    if let Ok(h) = reader.headers() {
        headers = h.iter().map(|s| s.to_string()).collect();
    }

    let mut rows = Vec::new();
    // Primera fila: headers
    let header_row: Vec<Valor> = headers.iter().map(|h| Valor::Texto(h.clone())).collect();
    rows.push(Valor::Array(header_row));

    // Filas de datos
    for result in reader.records() {
        match result {
            Ok(record) => {
                let mut row: Vec<Valor> = Vec::new();
                for field in record.iter() {
                    row.push(Valor::Texto(field.to_string()));
                }
                rows.push(Valor::Array(row));
            }
            Err(e) => {
                return Valor::Error(format!("csv::read() error parseando CSV: {}", e));
            }
        }
    }

    Valor::Array(rows)
}

/// csv::write(data, path) - Escribir CSV a archivo
pub fn csv_write(
    args: &[lizer::Expr],
    executor: &mut blast_core::Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 2 {
        return Valor::Error("csv::write() requiere 2 argumentos: data, path".to_string());
    }

    let data_val = evaluar_expr(&args[0], executor, funcs);
    let path_val = evaluar_expr(&args[1], executor, funcs);

    let rows = match data_val {
        Valor::Array(r) => r,
        _ => return Valor::Error("csv::write() data debe ser un array de filas".to_string()),
    };

    let path = match path_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("csv::write() path debe ser texto".to_string()),
    };

    // Construir contenido CSV
    let mut csv_content = String::new();

    for (i, row_val) in rows.iter().enumerate() {
        if let Valor::Array(cells) = row_val {
            let cell_strings: Vec<String> = cells
                .iter()
                .map(|v| match v {
                    Valor::Texto(s) => s.clone(),
                    Valor::Num(n) => n.to_string(),
                    Valor::Bool(b) => b.to_string(),
                    _ => format!("{:?}", v),
                })
                .collect();
            csv_content.push_str(&cell_strings.join(","));
            if i < rows.len() - 1 {
                csv_content.push('\n');
            }
        } else {
            return Valor::Error("csv::write() cada fila debe ser un array".to_string());
        }
    }

    // Escribir archivo
    match fs::write(&path, csv_content) {
        Ok(_) => Valor::Texto(format!(
            "csv::write() - {} filas escritas en '{}'",
            rows.len(),
            path
        )),
        Err(e) => Valor::Error(format!("csv::write() no pudo escribir '{}': {}", path, e)),
    }
}

// ============================================================================
// CONVERSIÓN CSV <-> JSON
// ============================================================================

/// csv::to_json(csv_text) - Convertir CSV a JSON
/// Retorna array de objetos como array de tuplas [(key, value), ...]
pub fn csv_to_json(
    args: &[lizer::Expr],
    executor: &mut blast_core::Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 1 {
        return Valor::Error("csv::to_json() requiere 1 argumento: csv_text".to_string());
    }

    let csv_val = evaluar_expr(&args[0], executor, funcs);
    let csv_text = match csv_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("csv::to_json() requiere texto CSV".to_string()),
    };

    // Parsear CSV
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_text.as_bytes());

    let mut headers: Vec<String> = Vec::new();
    if let Ok(h) = reader.headers() {
        headers = h.iter().map(|s| s.to_string()).collect();
    }

    // Construir array de objetos (cada objeto es un array de tuplas [nombre, valor])
    let mut objects: Vec<Valor> = Vec::new();

    for result in reader.records() {
        match result {
            Ok(record) => {
                let mut obj_fields: Vec<Valor> = Vec::new();
                for (i, field) in record.iter().enumerate() {
                    if i < headers.len() {
                        // Crear tupla [header, valor]
                        let value = if let Ok(n) = field.parse::<f64>() {
                            Valor::Num(n)
                        } else {
                            Valor::Texto(field.to_string())
                        };
                        obj_fields
                            .push(Valor::Array(vec![Valor::Texto(headers[i].clone()), value]));
                    }
                }
                objects.push(Valor::Array(obj_fields));
            }
            Err(e) => {
                return Valor::Error(format!("csv::to_json() error: {}", e));
            }
        }
    }

    Valor::Array(objects)
}

/// csv::from_json(json_text) - Convertir JSON a CSV
/// Formato esperado: array de objetos como array de tuplas [[key, value], ...]
pub fn csv_from_json(
    args: &[lizer::Expr],
    executor: &mut blast_core::Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 1 {
        return Valor::Error("csv::from_json() requiere 1 argumento: json_text".to_string());
    }

    let json_val = evaluar_expr(&args[0], executor, funcs);

    // Parsear JSON manualmente (simplificado)
    // Formato esperado: [[[key1, val1], [key2, val2]], ...]
    let objects = match json_val {
        Valor::Array(objs) => objs,
        _ => return Valor::Error("csv::from_json() requiere array de objetos".to_string()),
    };

    if objects.is_empty() {
        return Valor::Texto("".to_string());
    }

    // Extraer headers del primer objeto
    let mut headers: Vec<String> = Vec::new();
    if let Some(Valor::Array(obj_fields)) = objects.first() {
        for field in obj_fields {
            if let Valor::Array(tuple) = field {
                if let Some(Valor::Texto(h)) = tuple.first() {
                    headers.push(h.clone());
                }
            }
        }
    }

    // Construir CSV
    let mut csv_lines: Vec<String> = Vec::new();

    // Header line
    csv_lines.push(headers.join(","));

    // Data lines
    for obj_val in &objects {
        if let Valor::Array(obj_fields) = obj_val {
            let mut row: Vec<String> = Vec::new();
            for header in &headers {
                // Buscar el valor para este header
                let value = obj_fields
                    .iter()
                    .find_map(|field| {
                        if let Valor::Array(tuple) = field {
                            if let Some(Valor::Texto(key)) = tuple.first() {
                                if key == header {
                                    return tuple.get(1).map(|v| match v {
                                        Valor::Texto(s) => s.clone(),
                                        Valor::Num(n) => n.to_string(),
                                        Valor::Bool(b) => b.to_string(),
                                        _ => format!("{:?}", v),
                                    });
                                }
                            }
                        }
                        None
                    })
                    .unwrap_or_default();
                row.push(value);
            }
            csv_lines.push(row.join(","));
        }
    }

    Valor::Texto(csv_lines.join("\n"))
}

// ============================================================================
// TRANSFORMACIÓN DE DATOS
// ============================================================================

/// csv::filter(data, column, value) - Filtrar filas por columna
pub fn csv_filter(
    args: &[lizer::Expr],
    executor: &mut blast_core::Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 3 {
        return Valor::Error(
            "csv::filter() requiere 3 argumentos: data, column, value".to_string(),
        );
    }

    let data_val = evaluar_expr(&args[0], executor, funcs);
    let column_val = evaluar_expr(&args[1], executor, funcs);
    let value_val = evaluar_expr(&args[2], executor, funcs);

    let rows = match data_val {
        Valor::Array(r) => r,
        _ => return Valor::Error("csv::filter() data debe ser array".to_string()),
    };

    let column = match column_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("csv::filter() column debe ser texto".to_string()),
    };

    // Primera fila son los headers
    if rows.is_empty() {
        return Valor::Array(Vec::new());
    }

    let headers_val = &rows[0];
    let headers = match headers_val {
        Valor::Array(h) => h,
        _ => return Valor::Error("csv::filter() primera fila debe ser headers".to_string()),
    };

    // Encontrar índice de la columna
    let col_idx = headers.iter().position(|h| {
        if let Valor::Texto(s) = h {
            s == &column
        } else {
            false
        }
    });

    let col_idx = match col_idx {
        Some(i) => i,
        None => return Valor::Error(format!("csv::filter() columna '{}' no encontrada", column)),
    };

    // Filtrar filas
    let mut filtered: Vec<Valor> = vec![rows[0].clone()]; // Incluir headers

    for (i, row_val) in rows.iter().enumerate() {
        if i == 0 {
            continue;
        } // Saltar headers

        if let Valor::Array(row) = row_val {
            if col_idx < row.len() {
                let cell = &row[col_idx];
                let matches = match &value_val {
                    Valor::Texto(s) => {
                        if let Valor::Texto(cell_s) = cell {
                            cell_s == s
                        } else {
                            false
                        }
                    }
                    Valor::Num(n) => {
                        if let Valor::Num(cell_n) = cell {
                            cell_n == n
                        } else {
                            false
                        }
                    }
                    _ => false,
                };

                if matches {
                    filtered.push(row_val.clone());
                }
            }
        }
    }

    Valor::Array(filtered)
}

/// csv::columns(data) - Obtener nombres de columnas
pub fn csv_columns(
    args: &[lizer::Expr],
    executor: &mut blast_core::Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 1 {
        return Valor::Error("csv::columns() requiere 1 argumento: data".to_string());
    }

    let data_val = evaluar_expr(&args[0], executor, funcs);
    let rows = match data_val {
        Valor::Array(r) => r,
        _ => return Valor::Error("csv::columns() data debe ser array".to_string()),
    };

    if rows.is_empty() {
        return Valor::Array(Vec::new());
    }

    // Primera fila son los headers
    match &rows[0] {
        Valor::Array(headers) => Valor::Array(headers.clone()),
        _ => Valor::Error("csv::columns() primera fila debe ser headers".to_string()),
    }
}

/// csv::row_count(data) - Contar filas (sin headers)
pub fn csv_row_count(
    args: &[lizer::Expr],
    executor: &mut blast_core::Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 1 {
        return Valor::Error("csv::row_count() requiere 1 argumento: data".to_string());
    }

    let data_val = evaluar_expr(&args[0], executor, funcs);
    let rows = match data_val {
        Valor::Array(r) => r,
        _ => return Valor::Error("csv::row_count() data debe ser array".to_string()),
    };

    // Restar 1 por los headers
    let count = if rows.is_empty() { 0 } else { rows.len() - 1 };
    Valor::Num(count as f64)
}

/// csv::col_count(data) - Contar columnas
pub fn csv_col_count(
    args: &[lizer::Expr],
    executor: &mut blast_core::Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 1 {
        return Valor::Error("csv::col_count() requiere 1 argumento: data".to_string());
    }

    let data_val = evaluar_expr(&args[0], executor, funcs);
    let rows = match data_val {
        Valor::Array(r) => r,
        _ => return Valor::Error("csv::col_count() data debe ser array".to_string()),
    };

    if rows.is_empty() {
        return Valor::Num(0.0);
    }

    match &rows[0] {
        Valor::Array(headers) => Valor::Num(headers.len() as f64),
        _ => Valor::Num(0.0),
    }
}

// ============================================================================
// OPERACIONES AVANZADAS
// ============================================================================

/// csv::join(csv1, csv2, column) - Unir dos CSVs por columna común
pub fn csv_join(
    args: &[lizer::Expr],
    executor: &mut blast_core::Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 3 {
        return Valor::Error("csv::join() requiere 3 argumentos: csv1, csv2, column".to_string());
    }

    let csv1_val = evaluar_expr(&args[0], executor, funcs);
    let csv2_val = evaluar_expr(&args[1], executor, funcs);
    let column_val = evaluar_expr(&args[2], executor, funcs);

    let rows1 = match csv1_val {
        Valor::Array(r) => r,
        _ => return Valor::Error("csv::join() csv1 debe ser array".to_string()),
    };

    let rows2 = match csv2_val {
        Valor::Array(r) => r,
        _ => return Valor::Error("csv::join() csv2 debe ser array".to_string()),
    };

    let join_column = match column_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("csv::join() column debe ser texto".to_string()),
    };

    if rows1.is_empty() || rows2.is_empty() {
        return Valor::Array(Vec::new());
    }

    // Obtener headers de ambos CSVs
    let headers1 = match &rows1[0] {
        Valor::Array(h) => h,
        _ => return Valor::Error("csv::join() headers inválidos en csv1".to_string()),
    };

    let headers2 = match &rows2[0] {
        Valor::Array(h) => h,
        _ => return Valor::Error("csv::join() headers inválidos en csv2".to_string()),
    };

    // Encontrar índices de la columna join
    let col1_idx = headers1.iter().position(|h| {
        if let Valor::Texto(s) = h {
            s == &join_column
        } else {
            false
        }
    });
    let col2_idx = headers2.iter().position(|h| {
        if let Valor::Texto(s) = h {
            s == &join_column
        } else {
            false
        }
    });

    let col1_idx = match col1_idx {
        Some(i) => i,
        None => {
            return Valor::Error(format!(
                "csv::join() columna '{}' no encontrada en csv1",
                join_column
            ))
        }
    };
    let col2_idx = match col2_idx {
        Some(i) => i,
        None => {
            return Valor::Error(format!(
                "csv::join() columna '{}' no encontrada en csv2",
                join_column
            ))
        }
    };

    // Construir headers combinados (evitando duplicar la columna join)
    let mut combined_headers: Vec<Valor> = headers1.clone();
    for (i, h) in headers2.iter().enumerate() {
        if i != col2_idx {
            combined_headers.push(h.clone());
        }
    }

    // Inner join
    let mut result: Vec<Valor> = vec![Valor::Array(combined_headers)];

    for row1_val in &rows1 {
        if let Valor::Array(row1) = row1_val {
            let join_value = row1.get(col1_idx);

            for row2_val in &rows2 {
                if let Valor::Array(row2) = row2_val {
                    let join_value2 = row2.get(col2_idx);

                    // Comparar valores join
                    let matches = match (join_value, join_value2) {
                        (Some(Valor::Texto(a)), Some(Valor::Texto(b))) => a == b,
                        (Some(Valor::Num(a)), Some(Valor::Num(b))) => a == b,
                        _ => false,
                    };

                    if matches {
                        // Combinar filas
                        let mut combined_row: Vec<Valor> = row1.clone();
                        for (i, cell) in row2.iter().enumerate() {
                            if i != col2_idx {
                                combined_row.push(cell.clone());
                            }
                        }
                        result.push(Valor::Array(combined_row));
                    }
                }
            }
        }
    }

    Valor::Array(result)
}

/// csv::group_by(data, column) - Agrupar datos por columna
/// Retorna: [[grupo, [filas]], ...]
pub fn csv_group_by(
    args: &[lizer::Expr],
    executor: &mut blast_core::Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 2 {
        return Valor::Error("csv::group_by() requiere 2 argumentos: data, column".to_string());
    }

    let data_val = evaluar_expr(&args[0], executor, funcs);
    let column_val = evaluar_expr(&args[1], executor, funcs);

    let rows = match data_val {
        Valor::Array(r) => r,
        _ => return Valor::Error("csv::group_by() data debe ser array".to_string()),
    };

    let column = match column_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("csv::group_by() column debe ser texto".to_string()),
    };

    if rows.is_empty() {
        return Valor::Array(Vec::new());
    }

    let headers = match &rows[0] {
        Valor::Array(h) => h,
        _ => return Valor::Error("csv::group_by() primera fila debe ser headers".to_string()),
    };

    let col_idx = headers.iter().position(|h| {
        if let Valor::Texto(s) = h {
            s == &column
        } else {
            false
        }
    });

    let col_idx = match col_idx {
        Some(i) => i,
        None => {
            return Valor::Error(format!(
                "csv::group_by() columna '{}' no encontrada",
                column
            ))
        }
    };

    // Agrupar filas por valor de columna
    let mut groups: HashMap<String, Vec<Valor>> = HashMap::new();

    for (i, row_val) in rows.iter().enumerate() {
        if i == 0 {
            continue;
        } // Saltar headers

        if let Valor::Array(row) = row_val {
            let key_val = row.get(col_idx);
            let key = match key_val {
                Some(Valor::Texto(s)) => s.clone(),
                Some(Valor::Num(n)) => n.to_string(),
                _ => "unknown".to_string(),
            };

            groups
                .entry(key)
                .or_default()
                .push(row_val.clone());
        }
    }

    // Convertir a array de tuplas [[grupo, [filas]], ...]
    let result: Vec<Valor> = groups
        .into_iter()
        .map(|(k, v)| Valor::Array(vec![Valor::Texto(k), Valor::Array(v)]))
        .collect();

    Valor::Array(result)
}

/// csv::aggregate(data, column, operation) - Agregar datos de columna
/// Operaciones: sum, avg, count, min, max
pub fn csv_aggregate(
    args: &[lizer::Expr],
    executor: &mut blast_core::Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Valor {
    use crate::eval::evaluar_expr;

    if args.len() != 3 {
        return Valor::Error(
            "csv::aggregate() requiere 3 argumentos: data, column, operation".to_string(),
        );
    }

    let data_val = evaluar_expr(&args[0], executor, funcs);
    let column_val = evaluar_expr(&args[1], executor, funcs);
    let operation_val = evaluar_expr(&args[2], executor, funcs);

    let rows = match data_val {
        Valor::Array(r) => r,
        _ => return Valor::Error("csv::aggregate() data debe ser array".to_string()),
    };

    let column = match column_val {
        Valor::Texto(s) => s,
        _ => return Valor::Error("csv::aggregate() column debe ser texto".to_string()),
    };

    let operation = match operation_val {
        Valor::Texto(s) => s.to_lowercase(),
        _ => return Valor::Error("csv::aggregate() operation debe ser texto".to_string()),
    };

    if rows.is_empty() {
        return Valor::Num(0.0);
    }

    let headers = match &rows[0] {
        Valor::Array(h) => h,
        _ => return Valor::Error("csv::aggregate() primera fila debe ser headers".to_string()),
    };

    let col_idx = headers.iter().position(|h| {
        if let Valor::Texto(s) = h {
            s == &column
        } else {
            false
        }
    });

    let col_idx = match col_idx {
        Some(i) => i,
        None => {
            return Valor::Error(format!(
                "csv::aggregate() columna '{}' no encontrada",
                column
            ))
        }
    };

    // Extraer valores numéricos de la columna
    let mut values: Vec<f64> = Vec::new();

    for (i, row_val) in rows.iter().enumerate() {
        if i == 0 {
            continue;
        } // Saltar headers

        if let Valor::Array(row) = row_val {
            if let Some(Valor::Texto(s)) = row.get(col_idx) {
                if let Ok(n) = s.parse::<f64>() {
                    values.push(n);
                }
            } else if let Some(Valor::Num(n)) = row.get(col_idx) {
                values.push(*n);
            }
        }
    }

    // Calcular operación
    match operation.as_str() {
        "sum" => {
            let sum: f64 = values.iter().sum();
            Valor::Num(sum)
        }
        "avg" | "average" => {
            if values.is_empty() {
                Valor::Num(0.0)
            } else {
                let sum: f64 = values.iter().sum();
                Valor::Num(sum / values.len() as f64)
            }
        }
        "count" => Valor::Num(values.len() as f64),
        "min" => values.iter().cloned().fold(Valor::Num(0.0), |acc, v| {
            if let Valor::Num(a) = acc {
                if v < a {
                    Valor::Num(v)
                } else {
                    Valor::Num(a)
                }
            } else {
                acc
            }
        }),
        "max" => values.iter().cloned().fold(Valor::Num(0.0), |acc, v| {
            if let Valor::Num(a) = acc {
                if v > a {
                    Valor::Num(v)
                } else {
                    Valor::Num(a)
                }
            } else {
                acc
            }
        }),
        _ => Valor::Error(format!(
            "csv::aggregate() operación '{}' no soportada",
            operation
        )),
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use blast_core::Executor;
    use lizer::{Expr, Stmt};

    fn setup_test() -> (Executor, HashMap<String, (Vec<String>, Vec<Stmt>)>) {
        (Executor::nuevo(), HashMap::new())
    }

    #[test]
    fn test_csv_columns() {
        let (mut executor, mut funcs) = setup_test();

        let result = csv_columns(
            &[Expr::Array(vec![
                Expr::Array(vec![
                    Expr::Texto("nombre".to_string()),
                    Expr::Texto("edad".to_string()),
                ]),
                Expr::Array(vec![
                    Expr::Texto("Juan".to_string()),
                    Expr::Texto("25".to_string()),
                ]),
            ])],
            &mut executor,
            &mut funcs,
        );

        if let Valor::Array(cols) = result {
            assert_eq!(cols.len(), 2);
        } else {
            panic!("csv_columns debería retornar Array");
        }
    }

    #[test]
    fn test_csv_row_count() {
        let (mut executor, mut funcs) = setup_test();

        let result = csv_row_count(
            &[Expr::Array(vec![
                Expr::Array(vec![
                    Expr::Texto("a".to_string()),
                    Expr::Texto("b".to_string()),
                ]),
                Expr::Array(vec![
                    Expr::Texto("1".to_string()),
                    Expr::Texto("2".to_string()),
                ]),
                Expr::Array(vec![
                    Expr::Texto("3".to_string()),
                    Expr::Texto("4".to_string()),
                ]),
            ])],
            &mut executor,
            &mut funcs,
        );

        if let Valor::Num(count) = result {
            assert_eq!(count, 2.0);
        } else {
            panic!("csv_row_count debería retornar Num");
        }
    }

    #[test]
    fn test_csv_col_count() {
        let (mut executor, mut funcs) = setup_test();

        let result = csv_col_count(
            &[Expr::Array(vec![
                Expr::Array(vec![
                    Expr::Texto("a".to_string()),
                    Expr::Texto("b".to_string()),
                    Expr::Texto("c".to_string()),
                ]),
                Expr::Array(vec![
                    Expr::Texto("1".to_string()),
                    Expr::Texto("2".to_string()),
                    Expr::Texto("3".to_string()),
                ]),
            ])],
            &mut executor,
            &mut funcs,
        );

        if let Valor::Num(count) = result {
            assert_eq!(count, 3.0);
        } else {
            panic!("csv_col_count debería retornar Num");
        }
    }

    #[test]
    fn test_csv_filter() {
        let (mut executor, mut funcs) = setup_test();

        let result = csv_filter(
            &[
                Expr::Array(vec![
                    Expr::Array(vec![
                        Expr::Texto("nombre".to_string()),
                        Expr::Texto("edad".to_string()),
                    ]),
                    Expr::Array(vec![
                        Expr::Texto("Juan".to_string()),
                        Expr::Texto("25".to_string()),
                    ]),
                    Expr::Array(vec![
                        Expr::Texto("Maria".to_string()),
                        Expr::Texto("30".to_string()),
                    ]),
                    Expr::Array(vec![
                        Expr::Texto("Pedro".to_string()),
                        Expr::Texto("25".to_string()),
                    ]),
                ]),
                Expr::Texto("edad".to_string()),
                Expr::Texto("25".to_string()),
            ],
            &mut executor,
            &mut funcs,
        );

        if let Valor::Array(filtered) = result {
            assert_eq!(filtered.len(), 3);
        } else {
            panic!("csv_filter debería retornar Array");
        }
    }

    #[test]
    fn test_csv_aggregate_sum() {
        let (mut executor, mut funcs) = setup_test();

        let result = csv_aggregate(
            &[
                Expr::Array(vec![
                    Expr::Array(vec![
                        Expr::Texto("item".to_string()),
                        Expr::Texto("valor".to_string()),
                    ]),
                    Expr::Array(vec![
                        Expr::Texto("a".to_string()),
                        Expr::Texto("10".to_string()),
                    ]),
                    Expr::Array(vec![
                        Expr::Texto("b".to_string()),
                        Expr::Texto("20".to_string()),
                    ]),
                    Expr::Array(vec![
                        Expr::Texto("c".to_string()),
                        Expr::Texto("30".to_string()),
                    ]),
                ]),
                Expr::Texto("valor".to_string()),
                Expr::Texto("sum".to_string()),
            ],
            &mut executor,
            &mut funcs,
        );

        if let Valor::Num(sum) = result {
            assert_eq!(sum, 60.0);
        } else {
            panic!("csv_aggregate debería retornar Num");
        }
    }

    #[test]
    fn test_csv_aggregate_avg() {
        let (mut executor, mut funcs) = setup_test();

        let result = csv_aggregate(
            &[
                Expr::Array(vec![
                    Expr::Array(vec![
                        Expr::Texto("item".to_string()),
                        Expr::Texto("valor".to_string()),
                    ]),
                    Expr::Array(vec![
                        Expr::Texto("a".to_string()),
                        Expr::Texto("10".to_string()),
                    ]),
                    Expr::Array(vec![
                        Expr::Texto("b".to_string()),
                        Expr::Texto("20".to_string()),
                    ]),
                    Expr::Array(vec![
                        Expr::Texto("c".to_string()),
                        Expr::Texto("30".to_string()),
                    ]),
                ]),
                Expr::Texto("valor".to_string()),
                Expr::Texto("avg".to_string()),
            ],
            &mut executor,
            &mut funcs,
        );

        if let Valor::Num(avg) = result {
            assert_eq!(avg, 20.0);
        } else {
            panic!("csv_aggregate debería retornar Num");
        }
    }

    #[test]
    fn test_csv_functions_exist() {
        let _ = csv_read;
        let _ = csv_write;
        let _ = csv_to_json;
        let _ = csv_from_json;
        let _ = csv_filter;
        let _ = csv_columns;
        let _ = csv_row_count;
        let _ = csv_col_count;
        let _ = csv_join;
        let _ = csv_group_by;
        let _ = csv_aggregate;
    }
}
