// crates/rydit-rs/src/eval/mod.rs
// Evaluador de expresiones RyDit

use blast_core::{Executor, Valor};
use lizer::{Expr, Stmt};
use std::collections::{HashMap, HashSet};

// Importar funciones auxiliares desde main.rs
use crate::{ejecutar_stmt, valor_a_bool, valor_rydit_a_serde, valor_serde_a_rydit};

/// Algoritmo de De Casteljau para evaluar curvas de Bezier
fn de_casteljau(points: &[(f64, f64)], t: f64) -> (f64, f64) {
    let n = points.len();
    if n == 0 {
        return (0.0, 0.0);
    }
    if n == 1 {
        return points[0];
    }

    // Iterativamente interpolar entre puntos
    let mut current_points = points.to_vec();
    for r in 1..n {
        for i in 0..(n - r) {
            let x = (1.0 - t) * current_points[i].0 + t * current_points[i + 1].0;
            let y = (1.0 - t) * current_points[i].1 + t * current_points[i + 1].1;
            current_points[i] = (x, y);
        }
    }
    current_points[0]
}

/// Evaluar una expresión RyDit
pub fn evaluar_expr(
    expr: &Expr,
    executor: &mut Executor,
    funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    match expr {
        Expr::Num(n) => Valor::Num(*n),
        Expr::Texto(s) => Valor::Texto(s.clone()),
        Expr::Var(name) => {
            // Input especial
            if name == "__INPUT__" {
                return executor.input("> ");
            }
            executor.leer(name).unwrap_or(Valor::Vacio)
        }
        Expr::Bool(b) => Valor::Bool(*b),
        Expr::Array(elements) => {
            // Evaluar cada elemento del array
            let valores: Vec<Valor> = elements
                .iter()
                .map(|e| evaluar_expr(e, executor, funcs))
                .collect();
            Valor::Array(valores)
        }
        Expr::Index { array, index } => {
            // Obtener el array
            let array_val = evaluar_expr(array, executor, funcs);
            let index_val = evaluar_expr(index, executor, funcs);

            // Verificar que sea un array
            if let Valor::Array(arr) = array_val {
                // Verificar que el índice sea un número
                if let Valor::Num(i) = index_val {
                    let idx = i as usize;
                    if idx < arr.len() {
                        arr[idx].clone()
                    } else {
                        Valor::Error(format!("Índice {} fuera de rango (len={})", idx, arr.len()))
                    }
                } else {
                    Valor::Error("El índice debe ser un número".to_string())
                }
            } else {
                Valor::Error("Solo se puede indexar arrays".to_string())
            }
        }
        Expr::Call { name, args } => {
            // Llamada a función builtin: tecla_presionada("tecla")
            if name == "tecla_presionada" && args.len() == 1 {
                // Función especial para input - retorna 0 por defecto (no presionada)
                // El valor real se obtiene del contexto gráfico
                return Valor::Num(0.0);
            }

            // Funciones aritméticas builtin
            if name == "sumar" && args.len() >= 2 {
                let mut suma = 0.0;
                for arg in args {
                    if let Valor::Num(n) = evaluar_expr(arg, executor, funcs) {
                        suma += n;
                    } else {
                        return Valor::Error("sumar() requiere números".to_string());
                    }
                }
                return Valor::Num(suma);
            }

            if name == "restar" && args.len() == 2 {
                let a = evaluar_expr(&args[0], executor, funcs);
                let b = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Num(a), Valor::Num(b)) = (a, b) {
                    return Valor::Num(a - b);
                } else {
                    return Valor::Error("restar() requiere números".to_string());
                }
            }

            if name == "multiplicar" && args.len() >= 2 {
                let mut producto = 1.0;
                for arg in args {
                    if let Valor::Num(n) = evaluar_expr(arg, executor, funcs) {
                        producto *= n;
                    } else {
                        return Valor::Error("multiplicar() requiere números".to_string());
                    }
                }
                return Valor::Num(producto);
            }

            if name == "dividir" && args.len() == 2 {
                let a = evaluar_expr(&args[0], executor, funcs);
                let b = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Num(a), Valor::Num(b)) = (a, b) {
                    if b != 0.0 {
                        return Valor::Num(a / b);
                    } else {
                        return Valor::Error("División por cero".to_string());
                    }
                } else {
                    return Valor::Error("dividir() requiere números".to_string());
                }
            }

            // ========================================================================
            // FUNCIONES MATH AVANZADAS - V0.3.0 (Tank Combat)
            // ========================================================================

            // math::sqrt(x) - Raíz cuadrada
            if (name == "__math_sqrt" || name == "math::sqrt") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    if x >= 0.0 {
                        return Valor::Num(x.sqrt());
                    } else {
                        return Valor::Error("math::sqrt() requiere número >= 0".to_string());
                    }
                } else {
                    return Valor::Error("math::sqrt() requiere número".to_string());
                }
            }

            // math::sin(x) - Seno (x en radianes)
            if (name == "__math_sin" || name == "math::sin") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.sin());
                } else {
                    return Valor::Error("math::sin() requiere número".to_string());
                }
            }

            // math::cos(x) - Coseno (x en radianes)
            if (name == "__math_cos" || name == "math::cos") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.cos());
                } else {
                    return Valor::Error("math::cos() requiere número".to_string());
                }
            }

            // math::tan(x) - Tangente (x en radianes)
            if (name == "__math_tan" || name == "math::tan") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.tan());
                } else {
                    return Valor::Error("math::tan() requiere número".to_string());
                }
            }

            // math::atan2(y, x) - Arcotangente de y/x (retorna radianes)
            if (name == "__math_atan2" || name == "math::atan2") && args.len() == 2 {
                let y_val = evaluar_expr(&args[0], executor, funcs);
                let x_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Num(y), Valor::Num(x)) = (y_val, x_val) {
                    return Valor::Num(y.atan2(x));
                } else {
                    return Valor::Error("math::atan2() requiere dos números".to_string());
                }
            }

            // math::deg2rad(x) - Convertir grados a radianes
            if (name == "__math_deg2rad" || name == "math::deg2rad") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.to_radians());
                } else {
                    return Valor::Error("math::deg2rad() requiere número".to_string());
                }
            }

            // math::rad2deg(x) - Convertir radianes a grados
            if (name == "__math_rad2deg" || name == "math::rad2deg") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.to_degrees());
                } else {
                    return Valor::Error("math::rad2deg() requiere número".to_string());
                }
            }

            // ========== FUNCIONES STRING (v0.1.2) ==========
            // Soporte para strings::length, strings::upper, etc.
            if (name == "__str_length" || name == "strings::length") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(s.len() as f64);
                } else {
                    return Valor::Error("strings::length() requiere string".to_string());
                }
            }

            if (name == "__str_upper" || name == "strings::upper") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Texto(s.to_uppercase());
                } else {
                    return Valor::Error("strings::upper() requiere string".to_string());
                }
            }

            if (name == "__str_lower" || name == "strings::lower") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Texto(s.to_lowercase());
                } else {
                    return Valor::Error("strings::lower() requiere string".to_string());
                }
            }

            if (name == "__str_concat" || name == "strings::concat") && args.len() == 2 {
                let a = evaluar_expr(&args[0], executor, funcs);
                let b = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(a), Valor::Texto(b)) = (a, b) {
                    return Valor::Texto(format!("{}{}", a, b));
                } else {
                    return Valor::Error("strings::concat() requiere dos strings".to_string());
                }
            }

            if (name == "__str_trim" || name == "strings::trim") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Texto(s.trim().to_string());
                } else {
                    return Valor::Error("strings::trim() requiere string".to_string());
                }
            }

            if (name == "__str_substr" || name == "strings::substr") && args.len() == 3 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let start_val = evaluar_expr(&args[1], executor, funcs);
                let len_val = evaluar_expr(&args[2], executor, funcs);
                if let (Valor::Texto(s), Valor::Num(start), Valor::Num(len)) =
                    (s_val, start_val, len_val)
                {
                    let start_idx = start as usize;
                    let length = len as usize;
                    if start_idx + length <= s.len() {
                        return Valor::Texto(s[start_idx..start_idx + length].to_string());
                    } else {
                        return Valor::Error(
                            "strings::substr(): índices fuera de rango".to_string(),
                        );
                    }
                } else {
                    return Valor::Error(
                        "strings::substr() requiere (string, inicio, longitud)".to_string(),
                    );
                }
            }

            if (name == "__str_replace" || name == "strings::replace") && args.len() == 3 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let buscar_val = evaluar_expr(&args[1], executor, funcs);
                let reemplazar_val = evaluar_expr(&args[2], executor, funcs);
                if let (Valor::Texto(s), Valor::Texto(buscar), Valor::Texto(reemplazar)) =
                    (s_val, buscar_val, reemplazar_val)
                {
                    return Valor::Texto(s.replace(&buscar, &reemplazar));
                } else {
                    return Valor::Error("strings::replace() requiere tres strings".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES STRINGS (v0.1.4) ==========
            if (name == "__str_split" || name == "strings::split") && args.len() == 2 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let sep_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(s), Valor::Texto(sep)) = (s_val, sep_val) {
                    let partes: Vec<Valor> =
                        s.split(&sep).map(|p| Valor::Texto(p.to_string())).collect();
                    return Valor::Array(partes);
                } else {
                    return Valor::Error(
                        "strings::split() requiere (string, separador)".to_string(),
                    );
                }
            }

            if (name == "__str_starts_with" || name == "strings::starts_with") && args.len() == 2 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let prefix_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(s), Valor::Texto(prefix)) = (s_val, prefix_val) {
                    return Valor::Bool(s.starts_with(&prefix));
                } else {
                    return Valor::Error("strings::starts_with() requiere dos strings".to_string());
                }
            }

            if (name == "__str_ends_with" || name == "strings::ends_with") && args.len() == 2 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let suffix_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(s), Valor::Texto(suffix)) = (s_val, suffix_val) {
                    return Valor::Bool(s.ends_with(&suffix));
                } else {
                    return Valor::Error("strings::ends_with() requiere dos strings".to_string());
                }
            }

            if (name == "__str_replace_all" || name == "strings::replace_all") && args.len() == 3 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let buscar_val = evaluar_expr(&args[1], executor, funcs);
                let reemplazar_val = evaluar_expr(&args[2], executor, funcs);
                if let (Valor::Texto(s), Valor::Texto(buscar), Valor::Texto(reemplazar)) =
                    (s_val, buscar_val, reemplazar_val)
                {
                    return Valor::Texto(s.replace(&buscar, &reemplazar));
                } else {
                    return Valor::Error(
                        "strings::replace_all() requiere tres strings".to_string(),
                    );
                }
            }

            if (name == "__str_join" || name == "strings::join") && args.len() == 2 {
                let sep_val = evaluar_expr(&args[0], executor, funcs);
                let arr_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(sep), Valor::Array(arr)) = (sep_val, arr_val) {
                    let strs: Result<Vec<String>, _> = arr
                        .iter()
                        .map(|v| {
                            if let Valor::Texto(s) = v {
                                Ok(s.clone())
                            } else {
                                Err("strings::join() requiere array de strings")
                            }
                        })
                        .collect();
                    match strs {
                        Ok(parts) => return Valor::Texto(parts.join(&sep)),
                        Err(msg) => return Valor::Error(msg.to_string()),
                    }
                } else {
                    return Valor::Error("strings::join() requiere (separador, array)".to_string());
                }
            }

            // ========== FUNCIONES IO (v0.1.2) ==========
            if (name == "__file_read" || name == "io::read_file") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => return Valor::Texto(content),
                        Err(e) => return Valor::Error(format!("io::read_file(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::read_file() requiere path (string)".to_string());
                }
            }

            if (name == "__file_write" || name == "io::write_file") && args.len() == 2 {
                let path_val = evaluar_expr(&args[0], executor, funcs);
                let content_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(path), Valor::Texto(content)) = (path_val, content_val) {
                    match std::fs::write(&path, &content) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::write_file(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::write_file() requiere (path, content)".to_string());
                }
            }

            if (name == "__file_exists" || name == "io::file_exists") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Bool(std::path::Path::new(&path).exists());
                } else {
                    return Valor::Error("io::file_exists() requiere path (string)".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES IO (v0.1.4) ==========
            if (name == "__dir_mkdir" || name == "io::mkdir") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    match std::fs::create_dir_all(&path) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::mkdir(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::mkdir() requiere path (string)".to_string());
                }
            }

            if (name == "__file_remove" || name == "io::remove") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    match std::fs::remove_file(&path) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => {
                            // Intentar como directorio
                            match std::fs::remove_dir_all(&path) {
                                Ok(_) => return Valor::Num(1.0),
                                Err(_) => return Valor::Error(format!("io::remove(): {}", e)),
                            }
                        }
                    }
                } else {
                    return Valor::Error("io::remove() requiere path (string)".to_string());
                }
            }

            if (name == "__file_rename" || name == "io::rename") && args.len() == 2 {
                let old_val = evaluar_expr(&args[0], executor, funcs);
                let new_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(old), Valor::Texto(new)) = (old_val, new_val) {
                    match std::fs::rename(&old, &new) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::rename(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::rename() requiere (old, new)".to_string());
                }
            }

            if (name == "__file_copy" || name == "io::copy") && args.len() == 2 {
                let src_val = evaluar_expr(&args[0], executor, funcs);
                let dst_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(src), Valor::Texto(dst)) = (src_val, dst_val) {
                    match std::fs::copy(&src, &dst) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::copy(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::copy() requiere (src, dst)".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES ARRAYS (v0.1.4) ==========
            if (name == "__array_push" || name == "arrays::push") && args.len() == 2 {
                let arr_val = evaluar_expr(&args[0], executor, funcs);
                let elem_val = evaluar_expr(&args[1], executor, funcs);
                if let Valor::Array(mut arr) = arr_val {
                    arr.push(elem_val);
                    return Valor::Array(arr);
                } else {
                    return Valor::Error("arrays::push() requiere (array, elemento)".to_string());
                }
            }

            if (name == "__array_pop" || name == "arrays::pop") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr(&args[0], executor, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("arrays::pop(): array vacío".to_string());
                    }
                    let last = arr.pop().unwrap();
                    return last;
                } else {
                    return Valor::Error("arrays::pop() requiere array".to_string());
                }
            }

            if (name == "__array_shift" || name == "arrays::shift") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr(&args[0], executor, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("arrays::shift(): array vacío".to_string());
                    }
                    let first = arr.remove(0);
                    return first;
                } else {
                    return Valor::Error("arrays::shift() requiere array".to_string());
                }
            }

            if (name == "__array_unshift" || name == "arrays::unshift") && args.len() == 2 {
                let arr_val = evaluar_expr(&args[0], executor, funcs);
                let elem_val = evaluar_expr(&args[1], executor, funcs);
                if let Valor::Array(mut arr) = arr_val {
                    arr.insert(0, elem_val);
                    return Valor::Array(arr);
                } else {
                    return Valor::Error(
                        "arrays::unshift() requiere (array, elemento)".to_string(),
                    );
                }
            }

            if (name == "__array_slice" || name == "arrays::slice") && args.len() == 3 {
                let arr_val = evaluar_expr(&args[0], executor, funcs);
                let start_val = evaluar_expr(&args[1], executor, funcs);
                let end_val = evaluar_expr(&args[2], executor, funcs);
                if let (Valor::Array(arr), Valor::Num(start), Valor::Num(end)) =
                    (arr_val, start_val, end_val)
                {
                    let s = start as usize;
                    let e = end as usize;
                    if s <= e && e <= arr.len() {
                        let sliced: Vec<Valor> = arr[s..e].to_vec();
                        return Valor::Array(sliced);
                    } else {
                        return Valor::Error("arrays::slice(): índices inválidos".to_string());
                    }
                } else {
                    return Valor::Error(
                        "arrays::slice() requiere (array, inicio, fin)".to_string(),
                    );
                }
            }

            if (name == "__array_reverse" || name == "arrays::reverse") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr(&args[0], executor, funcs) {
                    arr.reverse();
                    return Valor::Array(arr);
                } else {
                    return Valor::Error("arrays::reverse() requiere array".to_string());
                }
            }

            // ========== FUNCIONES RANDOM (v0.1.6) ==========
            // PRNG xorshift - sin dependencias externas
            if (name == "__random_int" || name == "random::int") && args.len() == 2 {
                let min_val = evaluar_expr(&args[0], executor, funcs);
                let max_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Num(min), Valor::Num(max)) = (min_val, max_val) {
                    let seed = executor
                        .leer("__random_seed")
                        .unwrap_or(Valor::Num(12345.0));
                    let mut s = if let Valor::Num(n) = seed {
                        n as u32
                    } else {
                        12345
                    };
                    // xorshift PRNG
                    s ^= s << 13;
                    s ^= s >> 17;
                    s ^= s << 5;
                    executor.guardar("__random_seed", Valor::Num(s as f64));
                    // Mapear a rango [min, max] - TRUNCAR A ENTERO
                    let range = (max - min).abs() + 1.0;
                    let random_val = (s as f64 / u32::MAX as f64) * range;
                    // Truncar a entero y asegurar que esté en rango
                    let entero = (min + random_val.floor()) as i64;
                    let min_i = min.min(max) as i64;
                    let max_i = max.min(max) as i64;
                    let result = entero.max(min_i).min(max_i);
                    return Valor::Num(result as f64);
                } else {
                    return Valor::Error("random::int() requiere (min, max) números".to_string());
                }
            }

            if (name == "__random_float" || name == "random::float") && args.is_empty() {
                let seed = executor
                    .leer("__random_seed")
                    .unwrap_or(Valor::Num(12345.0));
                let mut s = if let Valor::Num(n) = seed {
                    n as u32
                } else {
                    12345
                };
                // xorshift PRNG
                s ^= s << 13;
                s ^= s >> 17;
                s ^= s << 5;
                executor.guardar("__random_seed", Valor::Num(s as f64));
                return Valor::Num(s as f64 / u32::MAX as f64);
            }

            if (name == "__random_choice" || name == "random::choice") && args.len() == 1 {
                let arr_val = evaluar_expr(&args[0], executor, funcs);
                if let Valor::Array(arr) = arr_val {
                    if arr.is_empty() {
                        return Valor::Error("random::choice(): array vacío".to_string());
                    }
                    let seed = executor
                        .leer("__random_seed")
                        .unwrap_or(Valor::Num(12345.0));
                    let mut s = if let Valor::Num(n) = seed {
                        n as u32
                    } else {
                        12345
                    };
                    // xorshift PRNG
                    s ^= s << 13;
                    s ^= s >> 17;
                    s ^= s << 5;
                    executor.guardar("__random_seed", Valor::Num(s as f64));
                    let idx = (s as usize) % arr.len();
                    return arr[idx].clone();
                } else {
                    return Valor::Error("random::choice() requiere array".to_string());
                }
            }

            // ========== FUNCIONES JSON (v0.1.5) ==========
            if (name == "__json_parse" || name == "json::parse") && args.len() == 1 {
                if let Valor::Texto(json_str) = evaluar_expr(&args[0], executor, funcs) {
                    match serde_json::from_str::<serde_json::Value>(&json_str) {
                        Ok(val) => {
                            // Convertir serde_json::Value a Valor::Rydit
                            return valor_serde_a_rydit(&val);
                        }
                        Err(e) => return Valor::Error(format!("json::parse(): {}", e)),
                    }
                } else {
                    return Valor::Error("json::parse() requiere string JSON".to_string());
                }
            }

            if (name == "__json_stringify" || name == "json::stringify") && args.len() == 1 {
                let val = evaluar_expr(&args[0], executor, funcs);
                match valor_rydit_a_serde(&val) {
                    Ok(serde_val) => match serde_json::to_string(&serde_val) {
                        Ok(json_str) => return Valor::Texto(json_str),
                        Err(e) => return Valor::Error(format!("json::stringify(): {}", e)),
                    },
                    Err(e) => return Valor::Error(format!("json::stringify(): {}", e)),
                }
            }

            // ========== FUNCIONES TIME (v0.1.6) ==========
            if (name == "__time_now" || name == "time::now") && args.is_empty() {
                use std::time::{SystemTime, UNIX_EPOCH};
                match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(duration) => return Valor::Num(duration.as_secs_f64()),
                    Err(e) => return Valor::Error(format!("time::now(): {}", e)),
                }
            }

            if (name == "__time_sleep" || name == "time::sleep") && args.len() == 1 {
                use std::{thread, time::Duration};
                let ms_val = evaluar_expr(&args[0], executor, funcs);
                if let Valor::Num(ms) = ms_val {
                    thread::sleep(Duration::from_millis(ms as u64));
                    return Valor::Vacio;
                } else {
                    return Valor::Error(
                        "time::sleep() requiere milisegundos (número)".to_string(),
                    );
                }
            }

            // ========== FUNCIONES REGEX (v0.6.2) ==========
            if (name == "__regex_match" || name == "regex::match") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (
                    &evaluar_expr(&args[0], executor, funcs),
                    &evaluar_expr(&args[1], executor, funcs),
                ) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => return Valor::Bool(re.is_match(text)),
                        Err(e) => return Valor::Error(format!("regex::match(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::match() requiere (patrón, texto)".to_string());
                }
            }

            if (name == "__regex_replace" || name == "regex::replace") && args.len() == 3 {
                if let (Valor::Texto(pattern), Valor::Texto(replacement), Valor::Texto(text)) = (
                    &evaluar_expr(&args[0], executor, funcs),
                    &evaluar_expr(&args[1], executor, funcs),
                    &evaluar_expr(&args[2], executor, funcs),
                ) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            return Valor::Texto(
                                re.replace_all(text, replacement.as_str()).to_string(),
                            )
                        }
                        Err(e) => return Valor::Error(format!("regex::replace(): {}", e)),
                    }
                } else {
                    return Valor::Error(
                        "regex::replace() requiere (patrón, reemplazo, texto)".to_string(),
                    );
                }
            }

            if (name == "__regex_split" || name == "regex::split") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (
                    &evaluar_expr(&args[0], executor, funcs),
                    &evaluar_expr(&args[1], executor, funcs),
                ) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            let parts: Vec<Valor> = re
                                .split(text)
                                .map(|s| Valor::Texto(s.to_string()))
                                .collect();
                            return Valor::Array(parts);
                        }
                        Err(e) => return Valor::Error(format!("regex::split(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::split() requiere (patrón, texto)".to_string());
                }
            }

            if (name == "__regex_find_all" || name == "regex::find_all") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (
                    &evaluar_expr(&args[0], executor, funcs),
                    &evaluar_expr(&args[1], executor, funcs),
                ) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            let matches: Vec<Valor> = re
                                .find_iter(text)
                                .map(|m| Valor::Texto(m.as_str().to_string()))
                                .collect();
                            return Valor::Array(matches);
                        }
                        Err(e) => return Valor::Error(format!("regex::find_all(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::find_all() requiere (patrón, texto)".to_string());
                }
            }

            if (name == "__regex_capture" || name == "regex::capture") && args.len() == 2 {
                if let (Valor::Texto(pattern), Valor::Texto(text)) = (
                    &evaluar_expr(&args[0], executor, funcs),
                    &evaluar_expr(&args[1], executor, funcs),
                ) {
                    match regex::Regex::new(pattern) {
                        Ok(re) => {
                            if let Some(caps) = re.captures(text) {
                                // Retornar array: [match completo, grupo1, grupo2, ...]
                                let mut result: Vec<Valor> = Vec::new();
                                // Match completo
                                result
                                    .push(Valor::Texto(caps.get(0).unwrap().as_str().to_string()));
                                // Grupos de captura
                                for i in 1..caps.len() {
                                    if let Some(m) = caps.get(i) {
                                        result.push(Valor::Texto(m.as_str().to_string()));
                                    } else {
                                        result.push(Valor::Vacio);
                                    }
                                }
                                return Valor::Array(result);
                            } else {
                                return Valor::Array(vec![]); // No match
                            }
                        }
                        Err(e) => return Valor::Error(format!("regex::capture(): {}", e)),
                    }
                } else {
                    return Valor::Error("regex::capture() requiere (patrón, texto)".to_string());
                }
            }

            // ========== FUNCIONES FILES (v0.6.3) ==========
            if (name == "__files_read" || name == "files::read") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => return Valor::Texto(content),
                        Err(e) => return Valor::Error(format!("files::read(): {}", e)),
                    }
                } else {
                    return Valor::Error("files::read() requiere ruta (string)".to_string());
                }
            }

            if (name == "__files_write" || name == "files::write") && args.len() == 2 {
                if let (Valor::Texto(path), Valor::Texto(content)) = (
                    &evaluar_expr(&args[0], executor, funcs),
                    &evaluar_expr(&args[1], executor, funcs),
                ) {
                    match std::fs::write(path, content) {
                        Ok(_) => return Valor::Bool(true),
                        Err(e) => return Valor::Error(format!("files::write(): {}", e)),
                    }
                } else {
                    return Valor::Error("files::write() requiere (ruta, contenido)".to_string());
                }
            }

            if (name == "__files_append" || name == "files::append") && args.len() == 2 {
                if let (Valor::Texto(path), Valor::Texto(content)) = (
                    &evaluar_expr(&args[0], executor, funcs),
                    &evaluar_expr(&args[1], executor, funcs),
                ) {
                    use std::io::Write;
                    match std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(path)
                    {
                        Ok(mut file) => match file.write_all(content.as_bytes()) {
                            Ok(_) => return Valor::Bool(true),
                            Err(e) => return Valor::Error(format!("files::append(): {}", e)),
                        },
                        Err(e) => return Valor::Error(format!("files::append(): {}", e)),
                    }
                } else {
                    return Valor::Error("files::append() requiere (ruta, contenido)".to_string());
                }
            }

            if (name == "__files_exists" || name == "files::exists") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    let exists = std::path::Path::new(&path).exists();
                    return Valor::Bool(exists);
                } else {
                    return Valor::Error("files::exists() requiere ruta (string)".to_string());
                }
            }

            if (name == "__files_delete" || name == "files::delete") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    match std::fs::remove_file(&path) {
                        Ok(_) => return Valor::Bool(true),
                        Err(e) => return Valor::Error(format!("files::delete(): {}", e)),
                    }
                } else {
                    return Valor::Error("files::delete() requiere ruta (string)".to_string());
                }
            }

            // Función de usuario - ejecutar y capturar retorno
            // Ahora tenemos &mut Executor, podemos ejecutar la función
            // Clonar datos para evitar borrow checker issues

            // Primero buscar con nombre completo (math::sumar)
            // Si no existe, intentar buscar solo el nombre después de ::
            let func_name = if name.contains("::") {
                // Es nombre con namespace, intentar con nombre completo primero
                if funcs.contains_key(name) {
                    name.clone()
                } else {
                    // Extraer nombre después de ::
                    name.split("::").last().unwrap_or(name).to_string()
                }
            } else {
                name.clone()
            };

            let func_data = funcs.get(&func_name).map(|(p, b)| (p.clone(), b.clone()));

            if let Some((params, body)) = func_data {
                // Evaluar argumentos
                let mut arg_values = vec![];
                for arg in args {
                    arg_values.push(evaluar_expr(arg, executor, funcs));
                }

                // Crear scope local para la función
                executor.push_scope();

                // Mapear parámetros → valores de argumentos en scope local
                for (i, param) in params.iter().enumerate() {
                    if i < arg_values.len() {
                        executor.guardar_local(param, arg_values[i].clone());
                    }
                }

                // Ejecutar body de la función y capturar retorno
                // Nota: las funciones no pueden tener imports, así que usamos contextos vacíos
                let mut empty_loaded: HashSet<String> = HashSet::new();
                let mut empty_stack: Vec<String> = Vec::new();

                let mut return_value: Option<Valor> = None;
                for s in &body {
                    match ejecutar_stmt(s, executor, funcs, &mut empty_loaded, &mut empty_stack) {
                        (Some(true), _) => {
                            // Break en función - salir
                            executor.pop_scope();
                            return Valor::Error(
                                "Break no permitido en función de expresión".to_string(),
                            );
                        }
                        (_, Some(val)) => {
                            return_value = Some(val);
                            break; // Hay retorno
                        }
                        _ => {}
                    }
                }

                // Pop scope al finalizar
                executor.pop_scope();

                // Retornar valor o Vacio si no hubo return explícito
                return return_value.unwrap_or(Valor::Vacio);
            }

            // ========================================================================
            // ANIMACIÓN 2D + ILUSIONES ÓPTICAS (v0.7.1.1)
            // ========================================================================

            // --- EASING FUNCTIONS (Slow In & Slow Out - Principio #6) ---
            if name == "anim::ease_in" && args.len() == 1 {
                if let Valor::Num(t) = evaluar_expr(&args[0], executor, funcs) {
                    let t = t.clamp(0.0, 1.0);
                    return Valor::Num(t * t);
                }
                return Valor::Error("anim::ease_in() requiere número (0.0-1.0)".to_string());
            }
            if name == "anim::ease_out" && args.len() == 1 {
                if let Valor::Num(t) = evaluar_expr(&args[0], executor, funcs) {
                    let t = t.clamp(0.0, 1.0);
                    return Valor::Num(t * (2.0 - t));
                }
                return Valor::Error("anim::ease_out() requiere número (0.0-1.0)".to_string());
            }
            if name == "anim::ease_in_out" && args.len() == 1 {
                if let Valor::Num(t) = evaluar_expr(&args[0], executor, funcs) {
                    let t = t.clamp(0.0, 1.0);
                    return Valor::Num(if t < 0.5 {
                        2.0 * t * t
                    } else {
                        1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                    });
                }
                return Valor::Error("anim::ease_in_out() requiere número (0.0-1.0)".to_string());
            }

            // --- SQUASH & STRETCH (Principio #1) ---
            if name == "anim::squash" && args.len() == 1 {
                if let Valor::Num(factor) = evaluar_expr(&args[0], executor, funcs) {
                    let factor = factor.clamp(0.5, 2.0);
                    return Valor::Array(vec![Valor::Num(factor), Valor::Num(1.0 / factor)]);
                }
                return Valor::Error("anim::squash() requiere número (0.5-2.0)".to_string());
            }
            if name == "anim::stretch" && args.len() == 1 {
                if let Valor::Num(factor) = evaluar_expr(&args[0], executor, funcs) {
                    let factor = factor.clamp(0.5, 2.0);
                    return Valor::Array(vec![Valor::Num(1.0 / factor), Valor::Num(factor)]);
                }
                return Valor::Error("anim::stretch() requiere número (0.5-2.0)".to_string());
            }

            // --- ANTICIPATION (Principio #2) ---
            if name == "anim::anticipate" && args.len() == 3 {
                let pos = evaluar_expr(&args[0], executor, funcs);
                let target = evaluar_expr(&args[1], executor, funcs);
                let amount = evaluar_expr(&args[2], executor, funcs);
                if let (Valor::Num(pos), Valor::Num(target), Valor::Num(ant)) =
                    (pos, target, amount)
                {
                    let dir = if target > pos { -1.0 } else { 1.0 };
                    return Valor::Num(pos + dir * ant);
                }
                return Valor::Error("anim::anticipate() requiere 3 números".to_string());
            }

            // --- ILUSIONES ÓPTICAS ---
            if name == "illusion::muller_lyer" && args.len() == 4 {
                let x = evaluar_expr(&args[0], executor, funcs);
                let y = evaluar_expr(&args[1], executor, funcs);
                let len = evaluar_expr(&args[2], executor, funcs);
                let arrow = evaluar_expr(&args[3], executor, funcs);
                if let (Valor::Num(x), Valor::Num(y), Valor::Num(len), Valor::Bool(arrow)) =
                    (x, y, len, arrow)
                {
                    return Valor::Array(vec![
                        Valor::Num(x),
                        Valor::Num(y),
                        Valor::Num(len),
                        Valor::Bool(arrow),
                    ]);
                }
                return Valor::Error(
                    "illusion::muller_lyer() requiere (x, y, length, arrow_in)".to_string(),
                );
            }
            if name == "illusion::ponzo" && args.len() == 4 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if vals.iter().all(|v| matches!(v, Valor::Num(_))) {
                    return Valor::Array(vals);
                }
                return Valor::Error("illusion::ponzo() requiere 4 números".to_string());
            }
            if name == "illusion::phi_effect" && args.len() == 6 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if let [Valor::Num(x1), Valor::Num(y1), Valor::Num(x2), Valor::Num(y2), Valor::Num(speed), Valor::Num(frame)] =
                    vals.as_slice()
                {
                    let dist = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
                    let t = if dist > 0.0 {
                        ((frame * speed) % (dist * 2.0)) / dist
                    } else {
                        0.0
                    };
                    let t = if t > 1.0 { 2.0 - t } else { t };
                    return Valor::Array(vec![
                        Valor::Num(x1 + (x2 - x1) * t),
                        Valor::Num(y1 + (y2 - y1) * t),
                        Valor::Bool(t < 1.0),
                    ]);
                }
                return Valor::Error("illusion::phi_effect() requiere 6 números".to_string());
            }
            if name == "illusion::fraser_spiral" && args.len() == 5 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if let [Valor::Num(cx), Valor::Num(cy), Valor::Num(min_r), Valor::Num(max_r), Valor::Num(step)] =
                    vals.as_slice()
                {
                    let mut circles = Vec::new();
                    let mut r = *min_r;
                    while r <= *max_r {
                        circles.push(Valor::Num(r));
                        r += step;
                    }
                    return Valor::Array(vec![
                        Valor::Num(*cx),
                        Valor::Num(*cy),
                        Valor::Array(circles),
                    ]);
                }
                return Valor::Error("illusion::fraser_spiral() requiere 5 números".to_string());
            }

            // ========================================================================
            // FÍSICA 2D (v0.7.1.2)
            // ========================================================================

            // --- PROJECTILE MOTION ---
            if name == "physics::projectile" && args.len() == 4 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if let [Valor::Num(x0), Valor::Num(y0), Valor::Num(v0), Valor::Num(angle)] =
                    vals.as_slice()
                {
                    let rad = angle.to_radians();
                    let vx = v0 * rad.cos();
                    let vy = v0 * rad.sin();
                    let g = 9.81; // m/s²
                    let flight_time = 2.0 * vy / g;
                    let max_height = (vy * vy) / (2.0 * g);
                    let range = vx * flight_time;
                    return Valor::Array(vec![
                        Valor::Num(x0 + vx * flight_time), // x final
                        Valor::Num(*y0),                   // y final (asume suelo)
                        Valor::Num(flight_time),           // tiempo vuelo
                        Valor::Num(max_height),            // altura máxima
                        Valor::Num(range),                 // alcance horizontal
                    ]);
                }
                return Valor::Error(
                    "physics::projectile() requiere (x0, y0, v0, angle_grados)".to_string(),
                );
            }

            // --- PROJECTILE AT TIME ---
            if name == "physics::projectile_at" && args.len() == 5 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if let [Valor::Num(x0), Valor::Num(y0), Valor::Num(v0), Valor::Num(angle), Valor::Num(t)] =
                    vals.as_slice()
                {
                    let rad = angle.to_radians();
                    let vx = v0 * rad.cos();
                    let vy = v0 * rad.sin();
                    let g = 9.81;
                    let x = x0 + vx * t;
                    let y = y0 + vy * t - 0.5 * g * t * t;
                    let vy_t = vy - g * t;
                    return Valor::Array(vec![
                        Valor::Num(x),
                        Valor::Num(y),
                        Valor::Num(vx),
                        Valor::Num(vy_t),
                    ]);
                }
                return Valor::Error(
                    "physics::projectile_at() requiere (x0, y0, v0, angle, t)".to_string(),
                );
            }

            // --- N-BODY GRAVITY (2 cuerpos) ---
            if name == "physics::nbody_2" && args.len() == 7 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if let [Valor::Num(m1), Valor::Num(m2), Valor::Num(x1), Valor::Num(y1), Valor::Num(x2), Valor::Num(y2), Valor::Num(g)] =
                    vals.as_slice()
                {
                    let dx = x2 - x1;
                    let dy = y2 - y1;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist > 0.001 {
                        let force = g * m1 * m2 / (dist * dist);
                        let fx = force * dx / dist;
                        let fy = force * dy / dist;
                        return Valor::Array(vec![
                            Valor::Num(fx),
                            Valor::Num(fy),
                            Valor::Num(-fx),
                            Valor::Num(-fy),
                            Valor::Num(dist),
                        ]);
                    }
                    return Valor::Array(vec![
                        Valor::Num(0.0),
                        Valor::Num(0.0),
                        Valor::Num(0.0),
                        Valor::Num(0.0),
                        Valor::Num(dist),
                    ]);
                }
                return Valor::Error(
                    "physics::nbody_2() requiere (m1, m2, x1, y1, x2, y2, G)".to_string(),
                );
            }

            // --- WAVE EQUATION (1D) ---
            if name == "physics::wave_1d" && args.len() == 4 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if let [Valor::Num(x), Valor::Num(t), Valor::Num(lambda), Valor::Num(freq)] =
                    vals.as_slice()
                {
                    let k = 2.0 * std::f64::consts::PI / lambda;
                    let omega = 2.0 * std::f64::consts::PI * freq;
                    let amplitude = (k * x - omega * t).sin();
                    return Valor::Num(amplitude);
                }
                return Valor::Error(
                    "physics::wave_1d() requiere (x, t, lambda, frecuencia)".to_string(),
                );
            }

            // --- WAVE 2D CIRCULAR ---
            if name == "physics::wave_2d" && args.len() == 5 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if let [Valor::Num(x), Valor::Num(y), Valor::Num(t), Valor::Num(lambda), Valor::Num(freq)] =
                    vals.as_slice()
                {
                    let r = (x * x + y * y).sqrt();
                    let k = 2.0 * std::f64::consts::PI / lambda;
                    let omega = 2.0 * std::f64::consts::PI * freq;
                    let amplitude = if r > 0.01 {
                        (k * r - omega * t).sin() / r.sqrt()
                    } else {
                        0.0
                    };
                    return Valor::Num(amplitude);
                }
                return Valor::Error(
                    "physics::wave_2d() requiere (x, y, t, lambda, frecuencia)".to_string(),
                );
            }

            // --- PENDULUM ---
            if name == "physics::pendulum" && args.len() == 3 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if let [Valor::Num(length), Valor::Num(angle0), Valor::Num(t)] = vals.as_slice() {
                    let g = 9.81;
                    let omega = (g / length).sqrt();
                    let angle = angle0 * (omega * t).cos();
                    let angular_vel = -angle0 * omega * (omega * t).sin();
                    return Valor::Array(vec![
                        Valor::Num(angle),
                        Valor::Num(angular_vel),
                        Valor::Num(2.0 * std::f64::consts::PI / omega),
                    ]);
                }
                return Valor::Error(
                    "physics::pendulum() requiere (longitud, ang0, t)".to_string(),
                );
            }

            // ========================================================================
            // CIENCIA DE DATOS (v0.7.1.3)
            // ========================================================================

            // --- CSV PARSE ---
            if name == "csv::parse" && args.len() == 1 {
                if let Valor::Texto(csv_content) = evaluar_expr(&args[0], executor, funcs) {
                    let mut reader = csv::ReaderBuilder::new()
                        .has_headers(true)
                        .from_reader(csv_content.as_bytes());
                    let mut rows = Vec::new();
                    for result in reader.records() {
                        match result {
                            Ok(record) => {
                                let mut row = Vec::new();
                                for field in record.iter() {
                                    row.push(Valor::Texto(field.to_string()));
                                }
                                rows.push(Valor::Array(row));
                            }
                            Err(e) => {
                                return Valor::Error(format!("Error parseando CSV: {}", e));
                            }
                        }
                    }
                    return Valor::Array(rows);
                }
                return Valor::Error("csv::parse() requiere CSV (texto)".to_string());
            }

            // --- CSV PARSE NO HEADERS ---
            if name == "csv::parse_no_headers" && args.len() == 1 {
                if let Valor::Texto(csv_content) = evaluar_expr(&args[0], executor, funcs) {
                    let mut reader = csv::ReaderBuilder::new()
                        .has_headers(false)
                        .from_reader(csv_content.as_bytes());
                    let mut rows = Vec::new();
                    for result in reader.records() {
                        match result {
                            Ok(record) => {
                                let mut row = Vec::new();
                                for field in record.iter() {
                                    row.push(Valor::Texto(field.to_string()));
                                }
                                rows.push(Valor::Array(row));
                            }
                            Err(e) => {
                                return Valor::Error(format!("Error parseando CSV: {}", e));
                            }
                        }
                    }
                    return Valor::Array(rows);
                }
                return Valor::Error("csv::parse_no_headers() requiere CSV (texto)".to_string());
            }

            // --- STATISTICS: MEAN ---
            if name == "stats::mean" && args.len() == 1 {
                if let Valor::Array(arr) = evaluar_expr(&args[0], executor, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("stats::mean() array vacío".to_string());
                    }
                    let mut sum = 0.0;
                    let mut count = 0;
                    for val in arr {
                        if let Valor::Num(n) = val {
                            sum += n;
                            count += 1;
                        }
                    }
                    if count > 0 {
                        return Valor::Num(sum / count as f64);
                    }
                    return Valor::Error("stats::mean() requiere array de números".to_string());
                }
                return Valor::Error("stats::mean() requiere array".to_string());
            }

            // --- STATISTICS: MEDIAN ---
            if name == "stats::median" && args.len() == 1 {
                if let Valor::Array(arr) = evaluar_expr(&args[0], executor, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("stats::median() array vacío".to_string());
                    }
                    let mut nums: Vec<f64> = Vec::new();
                    for val in arr {
                        if let Valor::Num(n) = val {
                            nums.push(n);
                        }
                    }
                    if nums.is_empty() {
                        return Valor::Error(
                            "stats::median() requiere array de números".to_string(),
                        );
                    }
                    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let mid = nums.len() / 2;
                    let median = if nums.len().is_multiple_of(2) {
                        (nums[mid - 1] + nums[mid]) / 2.0
                    } else {
                        nums[mid]
                    };
                    return Valor::Num(median);
                }
                return Valor::Error("stats::median() requiere array".to_string());
            }

            // --- STATISTICS: STD DEV ---
            if name == "stats::std_dev" && args.len() == 1 {
                if let Valor::Array(arr) = evaluar_expr(&args[0], executor, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("stats::std_dev() array vacío".to_string());
                    }
                    let mut sum = 0.0;
                    let mut count = 0;
                    let mut nums: Vec<f64> = Vec::new();
                    for val in arr {
                        if let Valor::Num(n) = val {
                            sum += n;
                            count += 1;
                            nums.push(n);
                        }
                    }
                    if count > 1 {
                        let mean = sum / count as f64;
                        let variance: f64 = nums.iter().map(|x| (x - mean).powi(2)).sum::<f64>()
                            / (count - 1) as f64;
                        return Valor::Num(variance.sqrt());
                    }
                    return Valor::Num(0.0);
                }
                return Valor::Error("stats::std_dev() requiere array".to_string());
            }

            // --- STATISTICS: MIN ---
            if name == "stats::min" && args.len() == 1 {
                if let Valor::Array(arr) = evaluar_expr(&args[0], executor, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("stats::min() array vacío".to_string());
                    }
                    let mut min_val = f64::MAX;
                    let mut found = false;
                    for val in arr {
                        if let Valor::Num(n) = val {
                            if n < min_val {
                                min_val = n;
                            }
                            found = true;
                        }
                    }
                    if found {
                        return Valor::Num(min_val);
                    }
                    return Valor::Error("stats::min() requiere array de números".to_string());
                }
                return Valor::Error("stats::min() requiere array".to_string());
            }

            // --- STATISTICS: MAX ---
            if name == "stats::max" && args.len() == 1 {
                if let Valor::Array(arr) = evaluar_expr(&args[0], executor, funcs) {
                    if arr.is_empty() {
                        return Valor::Error("stats::max() array vacío".to_string());
                    }
                    let mut max_val = f64::MIN;
                    let mut found = false;
                    for val in arr {
                        if let Valor::Num(n) = val {
                            if n > max_val {
                                max_val = n;
                            }
                            found = true;
                        }
                    }
                    if found {
                        return Valor::Num(max_val);
                    }
                    return Valor::Error("stats::max() requiere array de números".to_string());
                }
                return Valor::Error("stats::max() requiere array".to_string());
            }

            // --- PLOT: GENERATE ASCII CHART ---
            if name == "plot::ascii_chart" && args.len() == 2 {
                if let (Valor::Array(data), Valor::Num(width)) = (
                    evaluar_expr(&args[0], executor, funcs),
                    evaluar_expr(&args[1], executor, funcs),
                ) {
                    let height = 20.0;
                    let mut nums: Vec<f64> = Vec::new();
                    for val in data {
                        if let Valor::Num(n) = val {
                            nums.push(n);
                        }
                    }
                    if nums.is_empty() {
                        return Valor::Texto("[]".to_string());
                    }
                    let min_val = nums.iter().cloned().fold(f64::MAX, f64::min);
                    let max_val = nums.iter().cloned().fold(f64::MIN, f64::max);
                    let range = if max_val > min_val {
                        max_val - min_val
                    } else {
                        1.0
                    };
                    let w = width as usize;
                    let h = height as usize;
                    let mut chart = vec![vec![' '; w]; h];
                    for (i, &val) in nums.iter().enumerate() {
                        let x = (i as f64 / (nums.len() - 1) as f64 * (w - 1) as f64) as usize;
                        let y = ((val - min_val) / range * (h - 1) as f64) as usize;
                        let y = h - 1 - y;
                        if x < w && y < h {
                            chart[y][x] = '*';
                        }
                    }
                    let mut result = String::new();
                    for row in chart {
                        result.push_str(&row.iter().collect::<String>());
                        result.push('\n');
                    }
                    return Valor::Texto(result);
                }
                return Valor::Error("plot::ascii_chart() requiere [datos], ancho".to_string());
            }

            // --- PLOT: GENERATE SVG CHART (simple line chart) ---
            if name == "plot::svg_chart" && args.len() == 3 {
                if let (Valor::Array(data), Valor::Num(width), Valor::Num(height)) = (
                    evaluar_expr(&args[0], executor, funcs),
                    evaluar_expr(&args[1], executor, funcs),
                    evaluar_expr(&args[2], executor, funcs),
                ) {
                    let mut nums: Vec<f64> = Vec::new();
                    for val in data {
                        if let Valor::Num(n) = val {
                            nums.push(n);
                        }
                    }
                    if nums.is_empty() {
                        return Valor::Texto("<svg></svg>".to_string());
                    }
                    let min_val = nums.iter().cloned().fold(f64::MAX, f64::min);
                    let max_val = nums.iter().cloned().fold(f64::MIN, f64::max);
                    let range = if max_val > min_val {
                        max_val - min_val
                    } else {
                        1.0
                    };
                    let w = width as i32;
                    let h = height as i32;
                    let padding = 10;
                    let mut svg = format!(
                        "<svg width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>",
                        w, h
                    );
                    svg.push_str(&format!(
                        "<rect width='{}' height='{}' fill='white'/>",
                        w, h
                    ));
                    let mut path = format!(
                        "<polyline points='{}' fill='none' stroke='blue' stroke-width='2'/>",
                        format!(
                            "{},{}",
                            padding,
                            h - padding
                                - ((nums[0] - min_val) / range * (h - 2 * padding) as f64) as i32
                        )
                    );
                    for (i, &val) in nums.iter().enumerate().skip(1) {
                        let x = padding
                            + (i as f64 / (nums.len() - 1) as f64 * (w - 2 * padding) as f64)
                                as i32;
                        let y = h
                            - padding
                            - ((val - min_val) / range * (h - 2 * padding) as f64) as i32;
                        path.push_str(&format!(",{},{}", x, y));
                    }
                    svg.push_str(&path);
                    svg.push_str("</svg>");
                    return Valor::Texto(svg);
                }
                return Valor::Error("plot::svg_chart() requiere [datos], ancho, alto".to_string());
            }

            // ========================================================================
            // CURVAS DE BEZIER (v0.7.1.4)
            // ========================================================================

            // --- BEZIER LINEAL (2 puntos de control) ---
            if name == "bezier::linear" && args.len() == 5 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if let [Valor::Num(p0_x), Valor::Num(p0_y), Valor::Num(p1_x), Valor::Num(p1_y), Valor::Num(t)] =
                    vals.as_slice()
                {
                    let t = t.clamp(0.0, 1.0);
                    let x = (1.0 - t) * p0_x + t * p1_x;
                    let y = (1.0 - t) * p0_y + t * p1_y;
                    return Valor::Array(vec![Valor::Num(x), Valor::Num(y)]);
                }
                return Valor::Error(
                    "bezier::linear() requiere (p0_x, p0_y, p1_x, p1_y, t)".to_string(),
                );
            }

            // --- BEZIER CUADRÁTICA (3 puntos de control) ---
            if name == "bezier::quadratic" && args.len() == 7 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if let [Valor::Num(p0_x), Valor::Num(p0_y), Valor::Num(p1_x), Valor::Num(p1_y), Valor::Num(p2_x), Valor::Num(p2_y), Valor::Num(t)] =
                    vals.as_slice()
                {
                    let t = t.clamp(0.0, 1.0);
                    let mt = 1.0 - t;
                    let x = mt * mt * p0_x + 2.0 * mt * t * p1_x + t * t * p2_x;
                    let y = mt * mt * p0_y + 2.0 * mt * t * p1_y + t * t * p2_y;
                    return Valor::Array(vec![Valor::Num(x), Valor::Num(y)]);
                }
                return Valor::Error(
                    "bezier::quadratic() requiere (p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, t)"
                        .to_string(),
                );
            }

            // --- BEZIER CÚBICA (4 puntos de control) ---
            if name == "bezier::cubic" && args.len() == 9 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if let [Valor::Num(p0_x), Valor::Num(p0_y), Valor::Num(p1_x), Valor::Num(p1_y), Valor::Num(p2_x), Valor::Num(p2_y), Valor::Num(p3_x), Valor::Num(p3_y), Valor::Num(t)] =
                    vals.as_slice()
                {
                    let t = t.clamp(0.0, 1.0);
                    let mt = 1.0 - t;
                    let mt2 = mt * mt;
                    let t2 = t * t;
                    let x = mt2 * mt * p0_x
                        + 3.0 * mt2 * t * p1_x
                        + 3.0 * mt * t2 * p2_x
                        + t2 * t * p3_x;
                    let y = mt2 * mt * p0_y
                        + 3.0 * mt2 * t * p1_y
                        + 3.0 * mt * t2 * p2_y
                        + t2 * t * p3_y;
                    return Valor::Array(vec![Valor::Num(x), Valor::Num(y)]);
                }
                return Valor::Error(
                    "bezier::cubic() requiere (p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, p3_x, p3_y, t)"
                        .to_string(),
                );
            }

            // --- DERIVADA BEZIER CÚBICA (tangente) ---
            if name == "bezier::cubic_derivative" && args.len() == 9 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if let [Valor::Num(p0_x), Valor::Num(p0_y), Valor::Num(p1_x), Valor::Num(p1_y), Valor::Num(p2_x), Valor::Num(p2_y), Valor::Num(p3_x), Valor::Num(p3_y), Valor::Num(t)] =
                    vals.as_slice()
                {
                    let t = t.clamp(0.0, 1.0);
                    let mt = 1.0 - t;
                    // Derivada de Bezier cúbica: B'(t) = 3(1-t)²(P1-P0) + 6(1-t)t(P2-P1) + 3t²(P3-P2)
                    let dx = 3.0 * mt * mt * (p1_x - p0_x)
                        + 6.0 * mt * t * (p2_x - p1_x)
                        + 3.0 * t * t * (p3_x - p2_x);
                    let dy = 3.0 * mt * mt * (p1_y - p0_y)
                        + 6.0 * mt * t * (p2_y - p1_y)
                        + 3.0 * t * t * (p3_y - p2_y);
                    return Valor::Array(vec![Valor::Num(dx), Valor::Num(dy)]);
                }
                return Valor::Error(
                    "bezier::cubic_derivative() requiere (p0_x, p0_y, p1_x, p1_y, p2_x, p2_y, p3_x, p3_y, t)".to_string(),
                );
            }

            // --- GENERAR PUNTOS DE CURVA BEZIER ---
            if name == "bezier::generate_points" && args.len() == 2 {
                if let (Valor::Array(control_points), Valor::Num(steps)) = (
                    evaluar_expr(&args[0], executor, funcs),
                    evaluar_expr(&args[1], executor, funcs),
                ) {
                    let n = steps as usize;
                    if n < 2 {
                        return Valor::Error(
                            "bezier::generate_points() requiere steps >= 2".to_string(),
                        );
                    }

                    // Extraer puntos de control
                    let mut points: Vec<(f64, f64)> = Vec::new();
                    for cp in &control_points {
                        if let Valor::Array(coord) = cp {
                            if coord.len() == 2 {
                                if let (Valor::Num(x), Valor::Num(y)) = (&coord[0], &coord[1]) {
                                    points.push((*x, *y));
                                }
                            }
                        }
                    }

                    if points.is_empty() {
                        return Valor::Error(
                            "bezier::generate_points() requiere puntos de control".to_string(),
                        );
                    }

                    // Generar puntos usando algoritmo de De Casteljau
                    let mut result = Vec::new();
                    for i in 0..n {
                        let t = i as f64 / (n - 1) as f64;
                        let point = de_casteljau(&points, t);
                        result.push(Valor::Array(vec![Valor::Num(point.0), Valor::Num(point.1)]));
                    }
                    return Valor::Array(result);
                }
                return Valor::Error(
                    "bezier::generate_points() requiere [puntos_control], steps".to_string(),
                );
            }

            Valor::Error(format!("Función '{}' no soportada en expresiones", name))
        }
        Expr::BinOp { left, op, right } => {
            let left_val = evaluar_expr(left, executor, funcs);
            let right_val = evaluar_expr(right, executor, funcs);

            // Operadores lógicos (usan referencias, no mueven valores)
            match op {
                lizer::BinOp::And => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool && r_bool);
                }
                lizer::BinOp::Or => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool || r_bool);
                }
                _ => {}
            }

            // Concatenación de strings con + (con coerción automática de números)
            if matches!(op, lizer::BinOp::Suma) {
                match (&left_val, &right_val) {
                    (Valor::Texto(l), Valor::Texto(r)) => {
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Texto(l), Valor::Num(r)) => {
                        // "texto" + numero -> "texto123"
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Num(l), Valor::Texto(r)) => {
                        // numero + "texto" -> "123texto"
                        return Valor::Texto(format!("{}{}", l, r));
                    }
                    (Valor::Num(_), Valor::Num(_)) => {
                        // numero + numero -> suma aritmética (comportamiento normal, se maneja abajo)
                    }
                    _ => {}
                }
            }

            // Operadores aritméticos/comparación (mueven valores Num)
            if let (Valor::Num(l), Valor::Num(r)) = (left_val, right_val) {
                return match op {
                    lizer::BinOp::Suma => Valor::Num(l + r),
                    lizer::BinOp::Resta => Valor::Num(l - r),
                    lizer::BinOp::Mult => Valor::Num(l * r),
                    lizer::BinOp::Div => {
                        if r != 0.0 {
                            Valor::Num(l / r)
                        } else {
                            Valor::Error("División por cero".to_string())
                        }
                    }
                    lizer::BinOp::Mayor => Valor::Bool(l > r),
                    lizer::BinOp::Menor => Valor::Bool(l < r),
                    lizer::BinOp::Igual => Valor::Bool((l - r).abs() < 0.0001),
                    lizer::BinOp::MayorIgual => Valor::Bool(l >= r),
                    lizer::BinOp::MenorIgual => Valor::Bool(l <= r),
                    _ => Valor::Error("Operador no soportado".to_string()),
                };
            }

            Valor::Error("Operación inválida".to_string())
        }
        Expr::Unary { op, expr } => {
            let val = evaluar_expr(expr, executor, funcs);
            match op {
                lizer::UnaryOp::Not => {
                    let b = valor_a_bool(&val);
                    Valor::Bool(!b)
                }
                lizer::UnaryOp::Neg => {
                    if let Valor::Num(n) = val {
                        Valor::Num(-n)
                    } else {
                        Valor::Error("Neg requiere número".to_string())
                    }
                }
            }
        }
    }
}

// ============================================================================
// EVALUAR EXPRESIÓN (MODO GRÁFICO CON INPUT)
// ============================================================================
