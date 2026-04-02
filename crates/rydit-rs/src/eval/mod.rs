// crates/rydit-rs/src/eval/mod.rs
// Evaluador de expresiones RyDit
// ✅ v0.10.4: Reconectado con módulos
// ✅ v0.11.4: Migrado a rydit-parser

use blast_core::{Executor, Valor};
use rydit_parser::{Expr, Stmt, Program, BinaryOp, UnaryOp};
use std::collections::{HashMap, HashSet};

// Importar funciones auxiliares desde main.rs
use crate::{ejecutar_stmt, valor_a_bool, valor_rydit_a_serde, valor_serde_a_rydit};

// ✅ v0.10.4: Conectar módulos
use crate::modules::assets::{assets_draw, assets_load, assets_sprite};
use crate::modules::audio::{audio_play, audio_stop};
use crate::modules::camera::{
    camera_apply_sdl2, camera_follow, camera_set_position, camera_set_zoom,
};
use crate::modules::entity::{entity_create, entity_get_position};
use crate::modules::input_map::{input_map_is_pressed, input_map_register};
use crate::modules::level::level_load;
use crate::modules::physics::physics_apply_gravity;

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
pub fn evaluar_expr<'a>(
    expr: &Expr<'a>,
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
        Expr::Call { callee, args } => {
            // Llamada a función builtin: tecla_presionada("tecla")
            // callee puede ser Expr::Var("nombre") o expresión compleja
            let func_name = if let Expr::Var(name) = callee.as_ref() {
                *name
            } else {
                return Valor::Error("Call requiere función válida".to_string());
            };
            
            if func_name == "tecla_presionada" && args.len() == 1 {
                // Función especial para input - retorna 0 por defecto (no presionada)
                // El valor real se obtiene del contexto gráfico
                return Valor::Num(0.0);
            }

            // Funciones aritméticas builtin
            if func_name == "sumar" && args.len() >= 2 {
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

            if func_name == "restar" && args.len() == 2 {
                let a = evaluar_expr(&args[0], executor, funcs);
                let b = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Num(a), Valor::Num(b)) = (a, b) {
                    return Valor::Num(a - b);
                } else {
                    return Valor::Error("restar() requiere números".to_string());
                }
            }

            if func_name == "multiplicar" && args.len() >= 2 {
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

            if func_name == "dividir" && args.len() == 2 {
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
            if (func_name == "__math_sqrt" || func_name == "math::sqrt") && args.len() == 1 {
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
            if (func_name == "__math_sin" || func_name == "math::sin") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.sin());
                } else {
                    return Valor::Error("math::sin() requiere número".to_string());
                }
            }

            // math::cos(x) - Coseno (x en radianes)
            if (func_name == "__math_cos" || func_name == "math::cos") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.cos());
                } else {
                    return Valor::Error("math::cos() requiere número".to_string());
                }
            }

            // math::tan(x) - Tangente (x en radianes)
            if (func_name == "__math_tan" || func_name == "math::tan") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.tan());
                } else {
                    return Valor::Error("math::tan() requiere número".to_string());
                }
            }

            // math::atan2(y, x) - Arcotangente de y/x (retorna radianes)
            if (func_name == "__math_atan2" || func_name == "math::atan2") && args.len() == 2 {
                let y_val = evaluar_expr(&args[0], executor, funcs);
                let x_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Num(y), Valor::Num(x)) = (y_val, x_val) {
                    return Valor::Num(y.atan2(x));
                } else {
                    return Valor::Error("math::atan2() requiere dos números".to_string());
                }
            }

            // math::deg2rad(x) - Convertir grados a radianes
            if (func_name == "__math_deg2rad" || func_name == "math::deg2rad") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.to_radians());
                } else {
                    return Valor::Error("math::deg2rad() requiere número".to_string());
                }
            }

            // math::rad2deg(x) - Convertir radianes a grados
            if (func_name == "__math_rad2deg" || func_name == "math::rad2deg") && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.to_degrees());
                } else {
                    return Valor::Error("math::rad2deg() requiere número".to_string());
                }
            }

            // ================================================================
            // ALIAS SIN PREFIJO math:: (para compatibilidad con demos)
            // ================================================================

            // sin(x) - Alias de math::sin(x)
            if func_name == "sin" && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.sin());
                } else {
                    return Valor::Error("sin() requiere número".to_string());
                }
            }

            // cos(x) - Alias de math::cos(x)
            if func_name == "cos" && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.cos());
                } else {
                    return Valor::Error("cos() requiere número".to_string());
                }
            }

            // tan(x) - Alias de math::tan(x)
            if func_name == "tan" && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(x.tan());
                } else {
                    return Valor::Error("tan() requiere número".to_string());
                }
            }

            // sqrt(x) - Alias de math::sqrt(x)
            if func_name == "sqrt" && args.len() == 1 {
                if let Valor::Num(x) = evaluar_expr(&args[0], executor, funcs) {
                    if x >= 0.0 {
                        return Valor::Num(x.sqrt());
                    } else {
                        return Valor::Error("sqrt() requiere número >= 0".to_string());
                    }
                } else {
                    return Valor::Error("sqrt() requiere número".to_string());
                }
            }

            // ================================================================
            // FIN ALIAS MATH
            // ================================================================

            // ========== FUNCIONES STRING (v0.1.2) ==========
            // Soporte para strings::length, strings::upper, etc.
            if (func_name == "__str_length" || func_name == "strings::length") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Num(s.len() as f64);
                } else {
                    return Valor::Error("strings::length() requiere string".to_string());
                }
            }

            if (func_name == "__str_upper" || func_name == "strings::upper") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Texto(s.to_uppercase());
                } else {
                    return Valor::Error("strings::upper() requiere string".to_string());
                }
            }

            if (func_name == "__str_lower" || func_name == "strings::lower") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Texto(s.to_lowercase());
                } else {
                    return Valor::Error("strings::lower() requiere string".to_string());
                }
            }

            if (func_name == "__str_concat" || func_name == "strings::concat") && args.len() == 2 {
                let a = evaluar_expr(&args[0], executor, funcs);
                let b = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(a), Valor::Texto(b)) = (a, b) {
                    return Valor::Texto(format!("{}{}", a, b));
                } else {
                    return Valor::Error("strings::concat() requiere dos strings".to_string());
                }
            }

            if (func_name == "__str_trim" || func_name == "strings::trim") && args.len() == 1 {
                if let Valor::Texto(s) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Texto(s.trim().to_string());
                } else {
                    return Valor::Error("strings::trim() requiere string".to_string());
                }
            }

            if (func_name == "__str_substr" || func_name == "strings::substr") && args.len() == 3 {
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

            if (func_name == "__str_replace" || func_name == "strings::replace") && args.len() == 3 {
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
            if (func_name == "__str_split" || func_name == "strings::split") && args.len() == 2 {
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

            if (func_name == "__str_starts_with" || func_name == "strings::starts_with") && args.len() == 2 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let prefix_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(s), Valor::Texto(prefix)) = (s_val, prefix_val) {
                    return Valor::Bool(s.starts_with(&prefix));
                } else {
                    return Valor::Error("strings::starts_with() requiere dos strings".to_string());
                }
            }

            if (func_name == "__str_ends_with" || func_name == "strings::ends_with") && args.len() == 2 {
                let s_val = evaluar_expr(&args[0], executor, funcs);
                let suffix_val = evaluar_expr(&args[1], executor, funcs);
                if let (Valor::Texto(s), Valor::Texto(suffix)) = (s_val, suffix_val) {
                    return Valor::Bool(s.ends_with(&suffix));
                } else {
                    return Valor::Error("strings::ends_with() requiere dos strings".to_string());
                }
            }

            if (func_name == "__str_replace_all" || func_name == "strings::replace_all") && args.len() == 3 {
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

            if (func_name == "__str_join" || func_name == "strings::join") && args.len() == 2 {
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
            if (func_name == "__file_read" || func_name == "io::read_file") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => return Valor::Texto(content),
                        Err(e) => return Valor::Error(format!("io::read_file(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::read_file() requiere path (string)".to_string());
                }
            }

            if (func_name == "__file_write" || func_name == "io::write_file") && args.len() == 2 {
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

            if (func_name == "__file_exists" || func_name == "io::file_exists") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    return Valor::Bool(std::path::Path::new(&path).exists());
                } else {
                    return Valor::Error("io::file_exists() requiere path (string)".to_string());
                }
            }

            // ========== NUEVAS FUNCIONES IO (v0.1.4) ==========
            if (func_name == "__dir_mkdir" || func_name == "io::mkdir") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    match std::fs::create_dir_all(&path) {
                        Ok(_) => return Valor::Num(1.0),
                        Err(e) => return Valor::Error(format!("io::mkdir(): {}", e)),
                    }
                } else {
                    return Valor::Error("io::mkdir() requiere path (string)".to_string());
                }
            }

            if (func_name == "__file_remove" || func_name == "io::remove") && args.len() == 1 {
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

            if (func_name == "__file_rename" || func_name == "io::rename") && args.len() == 2 {
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

            if (func_name == "__file_copy" || func_name == "io::copy") && args.len() == 2 {
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
            if (func_name == "__array_push" || func_name == "arrays::push") && args.len() == 2 {
                let arr_val = evaluar_expr(&args[0], executor, funcs);
                let elem_val = evaluar_expr(&args[1], executor, funcs);
                if let Valor::Array(mut arr) = arr_val {
                    arr.push(elem_val);
                    return Valor::Array(arr);
                } else {
                    return Valor::Error("arrays::push() requiere (array, elemento)".to_string());
                }
            }

            if (func_name == "__array_pop" || func_name == "arrays::pop") && args.len() == 1 {
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

            if (func_name == "__array_shift" || func_name == "arrays::shift") && args.len() == 1 {
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

            if (func_name == "__array_unshift" || func_name == "arrays::unshift") && args.len() == 2 {
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

            if (func_name == "__array_slice" || func_name == "arrays::slice") && args.len() == 3 {
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

            if (func_name == "__array_reverse" || func_name == "arrays::reverse") && args.len() == 1 {
                if let Valor::Array(mut arr) = evaluar_expr(&args[0], executor, funcs) {
                    arr.reverse();
                    return Valor::Array(arr);
                } else {
                    return Valor::Error("arrays::reverse() requiere array".to_string());
                }
            }

            // ========== FUNCIONES RANDOM (v0.1.6) ==========
            // PRNG xorshift - sin dependencias externas
            if (func_name == "__random_int" || func_name == "random::int") && args.len() == 2 {
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

            if (func_name == "__random_float" || func_name == "random::float") && args.is_empty() {
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

            if (func_name == "__random_choice" || func_name == "random::choice") && args.len() == 1 {
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
            if (func_name == "__json_parse" || func_name == "json::parse") && args.len() == 1 {
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

            if (func_name == "__json_stringify" || func_name == "json::stringify") && args.len() == 1 {
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
            if (func_name == "__time_now" || func_name == "time::now") && args.is_empty() {
                use std::time::{SystemTime, UNIX_EPOCH};
                match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(duration) => return Valor::Num(duration.as_secs_f64()),
                    Err(e) => return Valor::Error(format!("time::now(): {}", e)),
                }
            }

            if (func_name == "__time_sleep" || func_name == "time::sleep") && args.len() == 1 {
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
            if (func_name == "__regex_match" || func_name == "regex::match") && args.len() == 2 {
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

            if (func_name == "__regex_replace" || func_name == "regex::replace") && args.len() == 3 {
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

            if (func_name == "__regex_split" || func_name == "regex::split") && args.len() == 2 {
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

            if (func_name == "__regex_find_all" || func_name == "regex::find_all") && args.len() == 2 {
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

            if (func_name == "__regex_capture" || func_name == "regex::capture") && args.len() == 2 {
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
            if (func_name == "__files_read" || func_name == "files::read") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    match std::fs::read_to_string(&path) {
                        Ok(content) => return Valor::Texto(content),
                        Err(e) => return Valor::Error(format!("files::read(): {}", e)),
                    }
                } else {
                    return Valor::Error("files::read() requiere ruta (string)".to_string());
                }
            }

            if (func_name == "__files_write" || func_name == "files::write") && args.len() == 2 {
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

            if (func_name == "__files_append" || func_name == "files::append") && args.len() == 2 {
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

            if (func_name == "__files_exists" || func_name == "files::exists") && args.len() == 1 {
                if let Valor::Texto(path) = evaluar_expr(&args[0], executor, funcs) {
                    let exists = std::path::Path::new(&path).exists();
                    return Valor::Bool(exists);
                } else {
                    return Valor::Error("files::exists() requiere ruta (string)".to_string());
                }
            }

            if (func_name == "__files_delete" || func_name == "files::delete") && args.len() == 1 {
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
            let func_name_final = if func_name.contains("::") {
                // Es nombre con namespace, intentar con nombre completo primero
                if funcs.contains_key(&func_name) {
                    func_name.clone()
                } else {
                    // Extraer nombre después de ::
                    func_name.split("::").last().unwrap_or(&func_name).to_string()
                }
            } else {
                func_name.clone()
            };

            let func_data = funcs.get(&func_name_final).map(|(p, b)| (p.clone(), b.clone()));

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
            if func_name == "anim::ease_in" && args.len() == 1 {
                if let Valor::Num(t) = evaluar_expr(&args[0], executor, funcs) {
                    let t = t.clamp(0.0, 1.0);
                    return Valor::Num(t * t);
                }
                return Valor::Error("anim::ease_in() requiere número (0.0-1.0)".to_string());
            }
            if func_name == "anim::ease_out" && args.len() == 1 {
                if let Valor::Num(t) = evaluar_expr(&args[0], executor, funcs) {
                    let t = t.clamp(0.0, 1.0);
                    return Valor::Num(t * (2.0 - t));
                }
                return Valor::Error("anim::ease_out() requiere número (0.0-1.0)".to_string());
            }
            if func_name == "anim::ease_in_out" && args.len() == 1 {
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
            if func_name == "anim::squash" && args.len() == 1 {
                if let Valor::Num(factor) = evaluar_expr(&args[0], executor, funcs) {
                    let factor = factor.clamp(0.5, 2.0);
                    return Valor::Array(vec![Valor::Num(factor), Valor::Num(1.0 / factor)]);
                }
                return Valor::Error("anim::squash() requiere número (0.5-2.0)".to_string());
            }
            if func_name == "anim::stretch" && args.len() == 1 {
                if let Valor::Num(factor) = evaluar_expr(&args[0], executor, funcs) {
                    let factor = factor.clamp(0.5, 2.0);
                    return Valor::Array(vec![Valor::Num(1.0 / factor), Valor::Num(factor)]);
                }
                return Valor::Error("anim::stretch() requiere número (0.5-2.0)".to_string());
            }

            // --- ANTICIPATION (Principio #2) ---
            if func_name == "anim::anticipate" && args.len() == 3 {
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
            if func_name == "illusion::muller_lyer" && args.len() == 4 {
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
            if func_name == "illusion::ponzo" && args.len() == 4 {
                let vals: Vec<Valor> = args
                    .iter()
                    .map(|a| evaluar_expr(a, executor, funcs))
                    .collect();
                if vals.iter().all(|v| matches!(v, Valor::Num(_))) {
                    return Valor::Array(vals);
                }
                return Valor::Error("illusion::ponzo() requiere 4 números".to_string());
            }
            if func_name == "illusion::phi_effect" && args.len() == 6 {
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
            if func_name == "illusion::fraser_spiral" && args.len() == 5 {
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
            if func_name == "physics::projectile" && args.len() == 4 {
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
            if func_name == "physics::projectile_at" && args.len() == 5 {
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
            if func_name == "physics::nbody_2" && args.len() == 7 {
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
            if func_name == "physics::wave_1d" && args.len() == 4 {
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
            if func_name == "physics::wave_2d" && args.len() == 5 {
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
            if func_name == "physics::pendulum" && args.len() == 3 {
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
            if func_name == "csv::parse" && args.len() == 1 {
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
            if func_name == "csv::parse_no_headers" && args.len() == 1 {
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

            // --- CSV MODULE (v0.8.6) ---
            // csv::read(path) - Leer CSV desde archivo
            if func_name == "csv::read" && args.len() == 1 {
                use crate::modules::csv;
                return csv::csv_read(args, executor, funcs);
            }

            // csv::write(data, path) - Escribir CSV a archivo
            if func_name == "csv::write" && args.len() == 2 {
                use crate::modules::csv;
                return csv::csv_write(args, executor, funcs);
            }

            // csv::to_json(csv_text) - Convertir CSV a JSON
            if func_name == "csv::to_json" && args.len() == 1 {
                use crate::modules::csv;
                return csv::csv_to_json(args, executor, funcs);
            }

            // csv::from_json(json_text) - Convertir JSON a CSV
            if func_name == "csv::from_json" && args.len() == 1 {
                use crate::modules::csv;
                return csv::csv_from_json(args, executor, funcs);
            }

            // csv::filter(data, column, value) - Filtrar filas
            if func_name == "csv::filter" && args.len() == 3 {
                use crate::modules::csv;
                return csv::csv_filter(args, executor, funcs);
            }

            // csv::columns(data) - Obtener columnas
            if func_name == "csv::columns" && args.len() == 1 {
                use crate::modules::csv;
                return csv::csv_columns(args, executor, funcs);
            }

            // csv::row_count(data) - Contar filas
            if func_name == "csv::row_count" && args.len() == 1 {
                use crate::modules::csv;
                return csv::csv_row_count(args, executor, funcs);
            }

            // csv::col_count(data) - Contar columnas
            if func_name == "csv::col_count" && args.len() == 1 {
                use crate::modules::csv;
                return csv::csv_col_count(args, executor, funcs);
            }

            // csv::join(csv1, csv2, column) - Unir CSVs
            if func_name == "csv::join" && args.len() == 3 {
                use crate::modules::csv;
                return csv::csv_join(args, executor, funcs);
            }

            // csv::group_by(data, column) - Agrupar datos
            if func_name == "csv::group_by" && args.len() == 2 {
                use crate::modules::csv;
                return csv::csv_group_by(args, executor, funcs);
            }

            // csv::aggregate(data, column, operation) - Agregar datos
            if func_name == "csv::aggregate" && args.len() == 3 {
                use crate::modules::csv;
                return csv::csv_aggregate(args, executor, funcs);
            }

            // --- HTTP + WEBSOCKET (v0.8.7) ---
            // http::get(url) - GET request
            if func_name == "http::get" && args.len() == 1 {
                use crate::eval::evaluar_expr;
                let url_val = evaluar_expr(&args[0], executor, funcs);
                let url = match url_val {
                    Valor::Texto(s) => s,
                    _ => return Valor::Error("http::get() url debe ser texto".to_string()),
                };
                return match ureq::get(&url).call().into_string() {
                    Ok(response) => Valor::Texto(response),
                    Err(e) => Valor::Error(e),
                };
            }

            // http::post(url, data) - POST request
            if func_name == "http::post" && args.len() == 2 {
                use crate::eval::evaluar_expr;
                let url_val = evaluar_expr(&args[0], executor, funcs);
                let data_val = evaluar_expr(&args[1], executor, funcs);
                let url = match url_val {
                    Valor::Texto(s) => s,
                    _ => return Valor::Error("http::post() url debe ser texto".to_string()),
                };
                let data = match data_val {
                    Valor::Texto(s) => s,
                    Valor::Num(n) => n.to_string(),
                    _ => {
                        return Valor::Error(
                            "http::post() data debe ser texto o número".to_string(),
                        )
                    }
                };
                return match ureq::post(&url).send_string(&data).map(|r| r.into_string()).unwrap_or(Err("POST error".to_string())) {
                    Ok(response) => Valor::Texto(response),
                    Err(e) => Valor::Error(e),
                };
            }

            // http::put(url, data) - PUT request
            if func_name == "http::put" && args.len() == 2 {
                use crate::eval::evaluar_expr;
                let url_val = evaluar_expr(&args[0], executor, funcs);
                let data_val = evaluar_expr(&args[1], executor, funcs);
                let url = match url_val {
                    Valor::Texto(s) => s,
                    _ => return Valor::Error("http::put() url debe ser texto".to_string()),
                };
                let data = match data_val {
                    Valor::Texto(s) => s,
                    Valor::Num(n) => n.to_string(),
                    _ => {
                        return Valor::Error("http::put() data debe ser texto o número".to_string())
                    }
                };
                return match ureq::put(&url).send_string(&data).map(|r| r.into_string()).unwrap_or(Err("PUT error".to_string())) {
                    Ok(response) => Valor::Texto(response),
                    Err(e) => Valor::Error(e),
                };
            }

            // http::delete(url) - DELETE request
            if func_name == "http::delete" && args.len() == 1 {
                use crate::eval::evaluar_expr;
                let url_val = evaluar_expr(&args[0], executor, funcs);
                let url = match url_val {
                    Valor::Texto(s) => s,
                    _ => return Valor::Error("http::delete() url debe ser texto".to_string()),
                };
                return match ureq::delete(&url).call().into_string() {
                    Ok(response) => Valor::Texto(response),
                    Err(e) => Valor::Error(e),
                };
            }

            // ws::connect(url) - Conectar a WebSocket
            // 🔮 TODO: WebSocket functions - Implementar con tungstenite directo
            // if func_name == "ws::connect" && args.len() == 1 {
            //     use crate::eval::evaluar_expr;
            //     let url_val = evaluar_expr(&args[0], executor, funcs);
            //     let url = match url_val {
            //         Valor::Texto(s) => s,
            //         _ => return Valor::Error("ws::connect() url debe ser texto".to_string()),
            //     };
            //     return match TODO_ws_connect(&url) {
            //         Ok(msg) => Valor::Texto(msg),
            //         Err(e) => Valor::Error(e),
            //     };
            // }
            // ws::disconnect() - Desconectar WebSocket
            // if func_name == "ws::disconnect" {
            //     return match TODO_ws_disconnect() {
            //         Ok(msg) => Valor::Texto(msg),
            //         Err(e) => Valor::Error(e),
            //     };
            // }
            // ws::send(message) - Enviar mensaje
            // if func_name == "ws::send" && args.len() == 1 {
            //     use crate::eval::evaluar_expr;
            //     let msg_val = evaluar_expr(&args[0], executor, funcs);
            //     let msg = match msg_val {
            //         Valor::Texto(s) => s,
            //         Valor::Num(n) => n.to_string(),
            //         _ => {
            //             return Valor::Error(
            //                 "ws::send() message debe ser texto o número".to_string(),
            //             )
            //         }
            //     };
            //     return match TODO_ws_send(&msg) {
            //         Ok(msg) => Valor::Texto(msg),
            //         Err(e) => Valor::Error(e),
            //     };
            // }
            // ws::recv() - Recibir mensaje
            // if func_name == "ws::recv" {
            //     return match TODO_ws_recv() {
            //         Ok(msg) => Valor::Texto(msg),
            //         Err(e) => Valor::Error(e),
            //     };
            // }
            // ws::is_connected() - Verificar conexión
            // if func_name == "ws::is_connected" {
            //     return Valor::Bool(TODO_ws_is_connected());
            // }
            // ws::get_url() - Obtener URL actual
            // if func_name == "ws::get_url" {
            //     return match TODO_ws_get_url() {
            //         Some(url) => Valor::Texto(url),
            //         None => Valor::Vacio,
            //     };
            // }

            // --- ASSETS MANAGER (v0.5.1) ---
            // assets::load(id, path) - Cargar textura
            if func_name == "assets::load" && args.len() == 2 {
                use crate::modules::assets;

                // Evaluar ID
                let id_val = evaluar_expr(&args[0], executor, funcs);
                let id = match id_val {
                    Valor::Texto(s) => s,
                    Valor::Num(n) => n.to_string(),
                    _ => {
                        return Valor::Error(
                            "assets::load() el primer argumento debe ser un ID (texto)".to_string(),
                        )
                    }
                };

                // Evaluar path
                let path_val = evaluar_expr(&args[1], executor, funcs);
                let path = match path_val {
                    Valor::Texto(s) => s,
                    _ => {
                        return Valor::Error(
                            "assets::load() el segundo argumento debe ser el path (texto)"
                                .to_string(),
                        )
                    }
                };

                // Cargar textura
                match rydit_gfx::Assets::load_texture_from_path(&path) {
                    Ok(texture) => {
                        let assets = assets::get_assets();
                        let mut assets_ref = assets.borrow_mut();
                        assets_ref.insert_texture(id.clone(), texture);
                        println!("[ASSETS] Textura '{}' cargada desde '{}'", id, path);
                        return Valor::Texto(format!(
                            "assets::load() - '{}' cargado exitosamente",
                            id
                        ));
                    }
                    Err(e) => return Valor::Error(format!("assets::load() Error: {}", e)),
                }
            }

            // assets::sprite(id, path) - Alias de load
            if func_name == "assets::sprite" && args.len() == 2 {
                use crate::modules::assets;

                // Evaluar ID
                let id_val = evaluar_expr(&args[0], executor, funcs);
                let id = match id_val {
                    Valor::Texto(s) => s,
                    Valor::Num(n) => n.to_string(),
                    _ => {
                        return Valor::Error(
                            "assets::sprite() el primer argumento debe ser un ID (texto)"
                                .to_string(),
                        )
                    }
                };

                // Evaluar path
                let path_val = evaluar_expr(&args[1], executor, funcs);
                let path = match path_val {
                    Valor::Texto(s) => s,
                    _ => {
                        return Valor::Error(
                            "assets::sprite() el segundo argumento debe ser el path (texto)"
                                .to_string(),
                        )
                    }
                };

                // Cargar textura
                match rydit_gfx::Assets::load_texture_from_path(&path) {
                    Ok(texture) => {
                        let assets = assets::get_assets();
                        let mut assets_ref = assets.borrow_mut();
                        assets_ref.insert_texture(id.clone(), texture);
                        println!("[ASSETS] Sprite '{}' cargado desde '{}'", id, path);
                        return Valor::Texto(format!(
                            "assets::sprite() - '{}' cargado exitosamente",
                            id
                        ));
                    }
                    Err(e) => return Valor::Error(format!("assets::sprite() Error: {}", e)),
                }
            }

            // assets::exists(id) - Verificar si existe textura
            if func_name == "assets::exists" && args.len() == 1 {
                use crate::modules::assets;

                let id_val = evaluar_expr(&args[0], executor, funcs);
                let id = match id_val {
                    Valor::Texto(s) => s,
                    Valor::Num(n) => n.to_string(),
                    _ => {
                        return Valor::Error(
                            "assets::exists() el argumento debe ser el ID".to_string(),
                        )
                    }
                };

                let assets = assets::get_assets();
                let assets_ref = assets.borrow();

                if assets_ref.has_texture(&id) {
                    return Valor::Bool(true);
                } else {
                    return Valor::Bool(false);
                }
            }

            // assets::unload(id) - Descargar textura
            if func_name == "assets::unload" && args.len() == 1 {
                use crate::modules::assets;

                let id_val = evaluar_expr(&args[0], executor, funcs);
                let id = match id_val {
                    Valor::Texto(s) => s,
                    Valor::Num(n) => n.to_string(),
                    _ => {
                        return Valor::Error(
                            "assets::unload() el argumento debe ser el ID".to_string(),
                        )
                    }
                };

                let assets = assets::get_assets();
                let mut assets_ref = assets.borrow_mut();

                if assets_ref.unload_texture(&id) {
                    println!("[ASSETS] Textura '{}' descargada", id);
                    return Valor::Texto(format!("assets::unload() - '{}' descargado", id));
                } else {
                    return Valor::Error(format!("assets::unload() La textura '{}' no existe", id));
                }
            }

            // assets::count() - Cantidad de texturas cargadas
            if func_name == "assets::count" {
                use crate::modules::assets;

                let assets = assets::get_assets();
                let assets_ref = assets.borrow();
                return Valor::Num(assets_ref.texture_count() as f64);
            }

            // assets::set_position(id, x, y) - Actualizar posición
            if func_name == "assets::set_position" && args.len() == 3 {
                use crate::modules::assets;
                return assets::assets_set_position(args, executor, funcs);
            }

            // assets::set_rotation(id, angle) - Rotar sprite
            if func_name == "assets::set_rotation" && args.len() == 2 {
                use crate::modules::assets;
                return assets::assets_set_rotation(args, executor, funcs);
            }

            // assets::set_scale(id, scale_x, scale_y) - Escalar sprite
            if func_name == "assets::set_scale" && args.len() == 3 {
                use crate::modules::assets;
                return assets::assets_set_scale(args, executor, funcs);
            }

            // assets::set_color(id, color) - Cambiar color/tinte
            if func_name == "assets::set_color" && args.len() == 2 {
                use crate::modules::assets;
                return assets::assets_set_color(args, executor, funcs);
            }

            // assets::set_flip(id, horizontal, vertical) - Flip
            if func_name == "assets::set_flip" && args.len() == 3 {
                use crate::modules::assets;
                return assets::assets_set_flip(args, executor, funcs);
            }

            // assets::set_origin(id, origin_x, origin_y) - Punto de origen
            if func_name == "assets::set_origin" && args.len() == 3 {
                use crate::modules::assets;
                return assets::assets_set_origin(args, executor, funcs);
            }

            // assets::draw(id, x, y) - Dibujar textura en posición (usando RenderQueue)
            if func_name == "assets::draw" && args.len() >= 3 {
                use crate::modules::assets;
                use rydit_gfx::ColorRydit;
                use std::str::FromStr;

                let id_val = evaluar_expr(&args[0], executor, funcs);
                let x_val = evaluar_expr(&args[1], executor, funcs);
                let y_val = evaluar_expr(&args[2], executor, funcs);

                let id = match id_val {
                    Valor::Texto(s) => s,
                    Valor::Num(n) => n.to_string(),
                    _ => return Valor::Error("assets::draw() el ID debe ser texto".to_string()),
                };

                let x = match x_val {
                    Valor::Num(n) => n as f32,
                    _ => return Valor::Error("assets::draw() x debe ser número".to_string()),
                };

                let y = match y_val {
                    Valor::Num(n) => n as f32,
                    _ => return Valor::Error("assets::draw() y debe ser número".to_string()),
                };

                // Color opcional (default: blanco)
                let color = if args.len() >= 4 {
                    let color_val = evaluar_expr(&args[3], executor, funcs);
                    match color_val {
                        Valor::Texto(c) => ColorRydit::from_str(&c).unwrap_or(ColorRydit::Blanco),
                        _ => ColorRydit::Blanco,
                    }
                } else {
                    ColorRydit::Blanco
                };

                // ✅ v0.10.4: Usar RenderQueue en vez de FFI directo
                // NOTA: El dibujo real se hace en executor.rs cuando ejecuta la queue
                let assets = assets::get_assets();
                let assets_ref = assets.borrow();

                if assets_ref.has_texture(&id) {
                    // Guardar comando en contexto para que executor.rs lo dibuje
                    // Por ahora, retornamos éxito - el dibujo se maneja en ejecutar_stmt_gfx
                    return Valor::Texto(format!(
                        "assets::draw() - '{}' listo para dibujar en ({}, {})",
                        id, x, y
                    ));
                } else {
                    return Valor::Error(format!("assets::draw() La textura '{}' no existe", id));
                }
            }

            // assets::draw_scaled(id, x, y, scale) - Dibujar textura escalada
            if func_name == "assets::draw_scaled" && args.len() >= 4 {
                use crate::modules::assets;
                use rydit_gfx::ColorRydit;
                use std::str::FromStr;
                use v_shield::ffi;

                let id_val = evaluar_expr(&args[0], executor, funcs);
                let x_val = evaluar_expr(&args[1], executor, funcs);
                let y_val = evaluar_expr(&args[2], executor, funcs);
                let scale_val = evaluar_expr(&args[3], executor, funcs);

                let id = match id_val {
                    Valor::Texto(s) => s,
                    Valor::Num(n) => n.to_string(),
                    _ => {
                        return Valor::Error(
                            "assets::draw_scaled() el ID debe ser texto".to_string(),
                        )
                    }
                };

                let x = match x_val {
                    Valor::Num(n) => n as f32,
                    _ => {
                        return Valor::Error("assets::draw_scaled() x debe ser número".to_string())
                    }
                };

                let y = match y_val {
                    Valor::Num(n) => n as f32,
                    _ => {
                        return Valor::Error("assets::draw_scaled() y debe ser número".to_string())
                    }
                };

                let scale = match scale_val {
                    Valor::Num(n) => n as f32,
                    _ => {
                        return Valor::Error(
                            "assets::draw_scaled() scale debe ser número".to_string(),
                        )
                    }
                };

                // Color opcional
                let color = if args.len() >= 5 {
                    let color_val = evaluar_expr(&args[4], executor, funcs);
                    match color_val {
                        Valor::Texto(c) => ColorRydit::from_str(&c).unwrap_or(ColorRydit::Blanco),
                        _ => ColorRydit::Blanco,
                    }
                } else {
                    ColorRydit::Blanco
                };

                // Dibujar escalado usando FFI
                let assets = assets::get_assets();
                let assets_ref = assets.borrow();

                if let Some(texture) = assets_ref.get_texture(&id) {
                    unsafe {
                        ffi::DrawTextureEx(
                            **texture,
                            ffi::Vector2 { x, y },
                            0.0,
                            scale,
                            color.to_color().into(),
                        );
                    }
                    return Valor::Texto(format!(
                        "assets::draw_scaled() - '{}' dibujado en ({}, {}) con escala {}",
                        id, x, y, scale
                    ));
                } else {
                    return Valor::Error(format!(
                        "assets::draw_scaled() La textura '{}' no existe",
                        id
                    ));
                }
            }

            // --- AUDIO MANAGER (v0.5.1) ---
            // audio::beep(frecuencia, duracion) - Generar beep
            if func_name == "audio::beep" && args.len() == 2 {
                use crate::modules::audio;
                return audio::audio_beep(args, executor, funcs);
            }

            // audio::click() - Sonido de click UI
            if func_name == "audio::click" && args.is_empty() {
                use crate::modules::audio;
                return audio::audio_click(args, executor, funcs);
            }

            // audio::load(id, path) - Cargar sonido
            if func_name == "audio::load" && args.len() == 2 {
                use crate::modules::audio;
                return audio::audio_load_sound(args, executor, funcs);
            }

            // audio::play(id) - Reproducir sonido
            if func_name == "audio::play" && args.len() == 1 {
                use crate::modules::audio;
                return audio::audio_play(args, executor, funcs);
            }

            // audio::stop(id) - Detener sonido
            if func_name == "audio::stop" && args.len() == 1 {
                use crate::modules::audio;
                return audio::audio_stop(args, executor, funcs);
            }

            // audio::volume(id, level) - Configurar volumen
            if func_name == "audio::volume" && args.len() == 2 {
                use crate::modules::audio;
                return audio::audio_volume(args, executor, funcs);
            }

            // audio::load_music(path) - Cargar música
            if func_name == "audio::load_music" && args.len() == 1 {
                use crate::modules::audio;
                return audio::audio_load_music(args, executor, funcs);
            }

            // audio::play_music() - Reproducir música
            if func_name == "audio::play_music" {
                use crate::modules::audio;
                return audio::audio_play_music(args, executor, funcs);
            }

            // audio::stop_music() - Detener música
            if func_name == "audio::stop_music" {
                use crate::modules::audio;
                return audio::audio_stop_music(args, executor, funcs);
            }

            // audio::is_playing() - Verificar si hay música
            if func_name == "audio::is_playing" {
                use crate::modules::audio;
                return audio::audio_is_playing(args, executor, funcs);
            }

            // audio::music_volume(level) - Volumen de música
            if func_name == "audio::music_volume" && args.len() == 1 {
                use crate::modules::audio;
                return audio::audio_music_volume(args, executor, funcs);
            }

            // audio::count() - Cantidad de sonidos cargados
            if func_name == "audio::count" {
                use crate::modules::audio;
                return audio::audio_count(args, executor, funcs);
            }

            // audio::list() - Listar sonidos cargados
            if func_name == "audio::list" {
                use crate::modules::audio;
                return audio::audio_list(args, executor, funcs);
            }

            // ========================================================================
            // CÁMARA 2D (v0.9.0)
            // ========================================================================

            // camera::set_position(x, y) - Establecer posición
            if func_name == "camera::set_position" && args.len() == 2 {
                use crate::modules::camera;
                return camera::camera_set_position(args, executor, funcs);
            }

            // camera::get_position() - Obtener posición
            if func_name == "camera::get_position" {
                use crate::modules::camera;
                return camera::camera_get_position(args, executor, funcs);
            }

            // camera::set_zoom(level) - Establecer zoom
            if func_name == "camera::set_zoom" && args.len() == 1 {
                use crate::modules::camera;
                return camera::camera_set_zoom(args, executor, funcs);
            }

            // camera::get_zoom() - Obtener zoom
            if func_name == "camera::get_zoom" {
                use crate::modules::camera;
                return camera::camera_get_zoom(args, executor, funcs);
            }

            // camera::set_rotation(angle) - Establecer rotación
            if func_name == "camera::set_rotation" && args.len() == 1 {
                use crate::modules::camera;
                return camera::camera_set_rotation(args, executor, funcs);
            }

            // camera::get_rotation() - Obtener rotación
            if func_name == "camera::get_rotation" {
                use crate::modules::camera;
                return camera::camera_get_rotation(args, executor, funcs);
            }

            // camera::scroll(dx, dy) - Scroll relativo
            if func_name == "camera::scroll" && args.len() == 2 {
                use crate::modules::camera;
                return camera::camera_scroll(args, executor, funcs);
            }

            // camera::scroll_to(x, y) - Scroll absoluto
            if func_name == "camera::scroll_to" && args.len() == 2 {
                use crate::modules::camera;
                return camera::camera_scroll_to(args, executor, funcs);
            }

            // camera::set_bounds(min_x, min_y, max_x, max_y) - Límites
            if func_name == "camera::set_bounds" && args.len() == 4 {
                use crate::modules::camera;
                return camera::camera_set_bounds(args, executor, funcs);
            }

            // camera::clear_bounds() - Limpiar límites
            if func_name == "camera::clear_bounds" {
                use crate::modules::camera;
                return camera::camera_clear_bounds(args, executor, funcs);
            }

            // camera::follow(target_x, target_y) - Seguir objetivo
            if func_name == "camera::follow" && args.len() == 2 {
                use crate::modules::camera;
                return camera::camera_follow(args, executor, funcs);
            }

            // camera::follow_smooth(target_x, target_y, smooth) - Seguir suave
            if func_name == "camera::follow_smooth" && args.len() == 3 {
                use crate::modules::camera;
                return camera::camera_follow_smooth(args, executor, funcs);
            }

            // camera::set_follow_offset(offset_x, offset_y) - Offset seguimiento
            if func_name == "camera::set_follow_offset" && args.len() == 2 {
                use crate::modules::camera;
                return camera::camera_set_follow_offset(args, executor, funcs);
            }

            // camera::world_to_screen(wx, wy) - Convertir mundo a pantalla
            if func_name == "camera::world_to_screen" && args.len() == 2 {
                use crate::modules::camera;
                return camera::camera_world_to_screen(args, executor, funcs);
            }

            // camera::screen_to_world(sx, sy) - Convertir pantalla a mundo
            if func_name == "camera::screen_to_world" && args.len() == 2 {
                use crate::modules::camera;
                return camera::camera_screen_to_world(args, executor, funcs);
            }

            // camera::reset() - Resetear cámara
            if func_name == "camera::reset" {
                use crate::modules::camera;
                return camera::camera_reset(args, executor, funcs);
            }

            // camera::apply_sdl2(x, y, screen_width, screen_height) - Aplicar cámara SDL2
            if func_name == "camera::apply_sdl2" && args.len() == 4 {
                return camera_apply_sdl2(args, executor, funcs);
            }

            // ========================================================================
            // LEVEL MANAGER (v0.9.4)
            // ========================================================================

            // level::load(ruta) - Cargar nivel
            if func_name == "level::load" && args.len() == 1 {
                use crate::modules::level;
                return level::level_load(args, executor, funcs);
            }

            // level::unload() - Descargar nivel
            if func_name == "level::unload" {
                use crate::modules::level;
                return level::level_unload(args, executor, funcs);
            }

            // level::transition(ruta) - Transición a otro nivel
            if func_name == "level::transition" && args.len() == 1 {
                use crate::modules::level;
                return level::level_transition(args, executor, funcs);
            }

            // level::get_current() - Obtener nivel actual
            if func_name == "level::get_current" {
                use crate::modules::level;
                return level::level_get_current(args, executor, funcs);
            }

            // level::reload() - Recargar nivel
            if func_name == "level::reload" {
                use crate::modules::level;
                return level::level_reload(args, executor, funcs);
            }

            // level::get_name() - Obtener nombre del nivel
            if func_name == "level::get_name" {
                use crate::modules::level;
                return level::level_get_name(args, executor, funcs);
            }

            // level::set_checkpoint(nombre, x, y) - Establecer checkpoint
            if func_name == "level::set_checkpoint" && args.len() == 3 {
                use crate::modules::level;
                return level::level_set_checkpoint(args, executor, funcs);
            }

            // level::load_checkpoint(nombre) - Cargar checkpoint
            if func_name == "level::load_checkpoint" && args.len() == 1 {
                use crate::modules::level;
                return level::level_load_checkpoint(args, executor, funcs);
            }

            // level::get_checkpoint(nombre) - Obtener posición de checkpoint
            if func_name == "level::get_checkpoint" && args.len() == 1 {
                use crate::modules::level;
                return level::level_get_checkpoint(args, executor, funcs);
            }

            // level::list_checkpoints() - Listar checkpoints
            if func_name == "level::list_checkpoints" {
                use crate::modules::level;
                return level::level_list_checkpoints(args, executor, funcs);
            }

            // level::transition_fade(duracion) - Transición fade
            if func_name == "level::transition_fade" && args.len() == 1 {
                use crate::modules::level;
                return level::level_transition_fade(args, executor, funcs);
            }

            // level::transition_slide(direccion, duracion) - Transición slide
            if func_name == "level::transition_slide" && args.len() == 2 {
                use crate::modules::level;
                return level::level_transition_slide(args, executor, funcs);
            }

            // ========================================================================
            // TILEMAP SYSTEM (v0.9.4)
            // ========================================================================

            // tilemap::load(ruta, tile_size) - Cargar tilemap
            if func_name == "tilemap::load" && args.len() == 2 {
                use crate::modules::tilemap;
                return tilemap::tilemap_load(args, executor, funcs);
            }

            // tilemap::create(width, height, tile_size) - Crear tilemap vacío
            if func_name == "tilemap::create" && args.len() == 3 {
                use crate::modules::tilemap;
                return tilemap::tilemap_create(args, executor, funcs);
            }

            // tilemap::set_tile(x, y, tile_id) - Colocar tile
            if func_name == "tilemap::set_tile" && args.len() == 3 {
                use crate::modules::tilemap;
                return tilemap::tilemap_set_tile(args, executor, funcs);
            }

            // tilemap::get_tile(x, y) - Obtener tile
            if func_name == "tilemap::get_tile" && args.len() == 2 {
                use crate::modules::tilemap;
                return tilemap::tilemap_get_tile(args, executor, funcs);
            }

            // tilemap::draw() - Dibujar tilemap
            if func_name == "tilemap::draw" {
                use crate::modules::tilemap;
                return tilemap::tilemap_draw(args, executor, funcs);
            }

            // tilemap::fill_rect(x, y, w, h, tile_id) - Llenar rectángulo
            if func_name == "tilemap::fill_rect" && args.len() == 5 {
                use crate::modules::tilemap;
                return tilemap::tilemap_fill_rect(args, executor, funcs);
            }

            // tilemap::clear() - Limpiar tilemap
            if func_name == "tilemap::clear" {
                use crate::modules::tilemap;
                return tilemap::tilemap_clear(args, executor, funcs);
            }

            // tilemap::set_layer_count(count) - Establecer capas
            if func_name == "tilemap::set_layer_count" && args.len() == 1 {
                use crate::modules::tilemap;
                return tilemap::tilemap_set_layer_count(args, executor, funcs);
            }

            // tilemap::get_size() - Obtener tamaño
            if func_name == "tilemap::get_size" {
                use crate::modules::tilemap;
                return tilemap::tilemap_get_size(args, executor, funcs);
            }

            // tilemap::set_tileset(ruta) - Cambiar tileset
            if func_name == "tilemap::set_tileset" && args.len() == 1 {
                use crate::modules::tilemap;
                return tilemap::tilemap_set_tileset(args, executor, funcs);
            }

            // tilemap::set_offset(x, y) - Establecer offset
            if func_name == "tilemap::set_offset" && args.len() == 2 {
                use crate::modules::tilemap;
                return tilemap::tilemap_set_offset(args, executor, funcs);
            }

            // tilemap::set_visible(visible) - Establecer visibilidad
            if func_name == "tilemap::set_visible" && args.len() == 1 {
                use crate::modules::tilemap;
                return tilemap::tilemap_set_visible(args, executor, funcs);
            }

            // ========================================================================
            // COLLISION SYSTEM (v0.9.4)
            // ========================================================================

            // collision::check_rect_rect(...) - Colisión rect vs rect
            if func_name == "collision::check_rect_rect" && args.len() == 8 {
                use crate::modules::collision;
                return collision::collision_check_rect_rect(args, executor, funcs);
            }

            // collision::check_circle_circle(...) - Colisión circle vs circle
            if func_name == "collision::check_circle_circle" && args.len() == 6 {
                use crate::modules::collision;
                return collision::collision_check_circle_circle(args, executor, funcs);
            }

            // collision::check_rect_circle(...) - Colisión rect vs circle
            if func_name == "collision::check_rect_circle" && args.len() == 7 {
                use crate::modules::collision;
                return collision::collision_check_rect_circle(args, executor, funcs);
            }

            // collision::check_point_rect(...) - Punto vs rect
            if func_name == "collision::check_point_rect" && args.len() == 6 {
                use crate::modules::collision;
                return collision::collision_check_point_rect(args, executor, funcs);
            }

            // collision::check_point_circle(...) - Punto vs circle
            if func_name == "collision::check_point_circle" && args.len() == 5 {
                use crate::modules::collision;
                return collision::collision_check_point_circle(args, executor, funcs);
            }

            // collision::resolve(...) - Resolver colisión
            if func_name == "collision::resolve" && args.len() == 8 {
                use crate::modules::collision;
                return collision::collision_resolve(args, executor, funcs);
            }

            // area2d::create(id, x, y, w, h) - Crear área 2D
            if func_name == "area2d::create" && args.len() == 5 {
                use crate::modules::collision;
                return collision::area2d_create(args, executor, funcs);
            }

            // area2d::set_position(id, x, y) - Mover área
            if func_name == "area2d::set_position" && args.len() == 3 {
                use crate::modules::collision;
                return collision::area2d_set_position(args, executor, funcs);
            }

            // area2d::get_position(id) - Obtener posición
            if func_name == "area2d::get_position" && args.len() == 1 {
                use crate::modules::collision;
                return collision::area2d_get_position(args, executor, funcs);
            }

            // area2d::check(id, other_id) - Verificar colisión
            if func_name == "area2d::check" && args.len() == 2 {
                use crate::modules::collision;
                return collision::area2d_check(args, executor, funcs);
            }

            // area2d::get_overlapping(id) - Obtener superpuestas
            if func_name == "area2d::get_overlapping" && args.len() == 1 {
                use crate::modules::collision;
                return collision::area2d_get_overlapping(args, executor, funcs);
            }

            // area2d::set_active(id, active) - Activar/desactivar
            if func_name == "area2d::set_active" && args.len() == 2 {
                use crate::modules::collision;
                return collision::area2d_set_active(args, executor, funcs);
            }

            // area2d::destroy(id) - Eliminar área
            if func_name == "area2d::destroy" && args.len() == 1 {
                use crate::modules::collision;
                return collision::area2d_destroy(args, executor, funcs);
            }

            // area2d::count() - Contar áreas
            if func_name == "area2d::count" {
                use crate::modules::collision;
                return collision::area2d_count(args, executor, funcs);
            }

            // ========================================================================
            // WINDOW MANAGER (v0.9.4)
            // ========================================================================

            // window::set_title(titulo) - Establecer título
            if func_name == "window::set_title" && args.len() == 1 {
                use crate::modules::window;
                return window::window_set_title(args, executor, funcs);
            }

            // window::get_title() - Obtener título
            if func_name == "window::get_title" {
                use crate::modules::window;
                return window::window_get_title(args, executor, funcs);
            }

            // window::set_size(width, height) - Establecer tamaño
            if func_name == "window::set_size" && args.len() == 2 {
                use crate::modules::window;
                return window::window_set_size(args, executor, funcs);
            }

            // window::get_size() - Obtener tamaño
            if func_name == "window::get_size" {
                use crate::modules::window;
                return window::window_get_size(args, executor, funcs);
            }

            // window::get_width() - Obtener ancho
            if func_name == "window::get_width" {
                use crate::modules::window;
                return window::window_get_width(args, executor, funcs);
            }

            // window::get_height() - Obtener alto
            if func_name == "window::get_height" {
                use crate::modules::window;
                return window::window_get_height(args, executor, funcs);
            }

            // window::set_fullscreen(enabled) - Establecer fullscreen
            if func_name == "window::set_fullscreen" && args.len() == 1 {
                use crate::modules::window;
                return window::window_set_fullscreen(args, executor, funcs);
            }

            // window::is_fullscreen() - Verificar fullscreen
            if func_name == "window::is_fullscreen" {
                use crate::modules::window;
                return window::window_is_fullscreen(args, executor, funcs);
            }

            // window::toggle_fullscreen() - Alternar fullscreen
            if func_name == "window::toggle_fullscreen" {
                use crate::modules::window;
                return window::window_toggle_fullscreen(args, executor, funcs);
            }

            // window::set_windowed() - Forzar modo ventana
            if func_name == "window::set_windowed" {
                use crate::modules::window;
                return window::window_set_windowed(args, executor, funcs);
            }

            // window::set_vsync(enabled) - Establecer VSync
            if func_name == "window::set_vsync" && args.len() == 1 {
                use crate::modules::window;
                return window::window_set_vsync(args, executor, funcs);
            }

            // window::is_vsync_enabled() - Verificar VSync
            if func_name == "window::is_vsync_enabled" {
                use crate::modules::window;
                return window::window_is_vsync_enabled(args, executor, funcs);
            }

            // window::set_resizable(enabled) - Establecer redimensionable
            if func_name == "window::set_resizable" && args.len() == 1 {
                use crate::modules::window;
                return window::window_set_resizable(args, executor, funcs);
            }

            // window::minimize() - Minimizar ventana
            if func_name == "window::minimize" {
                use crate::modules::window;
                return window::window_minimize(args, executor, funcs);
            }

            // window::maximize() - Maximizar ventana
            if func_name == "window::maximize" {
                use crate::modules::window;
                return window::window_maximize(args, executor, funcs);
            }

            // window::restore() - Restaurar ventana
            if func_name == "window::restore" {
                use crate::modules::window;
                return window::window_restore(args, executor, funcs);
            }

            // window::set_fps_limit(fps) - Establecer límite de FPS
            if func_name == "window::set_fps_limit" && args.len() == 1 {
                use crate::modules::window;
                return window::window_set_fps_limit(args, executor, funcs);
            }

            // window::get_fps() - Obtener FPS
            if func_name == "window::get_fps" {
                use crate::modules::window;
                return window::window_get_fps(args, executor, funcs);
            }

            // window::get_delta_time() - Obtener delta time
            if func_name == "window::get_delta_time" {
                use crate::modules::window;
                return window::window_get_delta_time(args, executor, funcs);
            }

            // ========================================================================
            // ENTITY SYSTEM (v0.9.0)
            // ========================================================================

            // entity::create(type, x, y) - Crear entidad
            if func_name == "entity::create" && args.len() == 3 {
                use crate::modules::entity;
                return entity::entity_create(args, executor, funcs);
            }

            // entity::destroy(id) - Destruir entidad
            if func_name == "entity::destroy" && args.len() == 1 {
                use crate::modules::entity;
                return entity::entity_destroy(args, executor, funcs);
            }

            // entity::get_by_type(type) - Obtener IDs por tipo
            if func_name == "entity::get_by_type" && args.len() == 1 {
                use crate::modules::entity;
                return entity::entity_get_by_type(args, executor, funcs);
            }

            // entity::count() - Contar entidades activas
            if func_name == "entity::count" {
                use crate::modules::entity;
                return entity::entity_count(args, executor, funcs);
            }

            // entity::count_by_type(type) - Contar por tipo
            if func_name == "entity::count_by_type" && args.len() == 1 {
                use crate::modules::entity;
                return entity::entity_count_by_type(args, executor, funcs);
            }

            // entity::get_all() - Obtener todas las entidades activas
            if func_name == "entity::get_all" {
                use crate::modules::entity;
                return entity::entity_get_all(args, executor, funcs);
            }

            // entity::set_position(id, x, y) - Establecer posición
            if func_name == "entity::set_position" && args.len() == 3 {
                use crate::modules::entity;
                return entity::entity_set_position(args, executor, funcs);
            }

            // entity::get_position(id) - Obtener posición
            if func_name == "entity::get_position" && args.len() == 1 {
                use crate::modules::entity;
                return entity::entity_get_position(args, executor, funcs);
            }

            // entity::set_sprite(id, sprite_id) - Establecer sprite
            if func_name == "entity::set_sprite" && args.len() == 2 {
                use crate::modules::entity;
                return entity::entity_set_sprite(args, executor, funcs);
            }

            // entity::set_collidable(id, collidable) - Establecer colisión
            if func_name == "entity::set_collidable" && args.len() == 2 {
                use crate::modules::entity;
                return entity::entity_set_collidable(args, executor, funcs);
            }

            // entity::is_collidable(id) - Verificar colisión
            if func_name == "entity::is_collidable" && args.len() == 1 {
                use crate::modules::entity;
                return entity::entity_is_collidable(args, executor, funcs);
            }

            // entity::set_active(id, active) - Activar/desactivar
            if func_name == "entity::set_active" && args.len() == 2 {
                use crate::modules::entity;
                return entity::entity_set_active(args, executor, funcs);
            }

            // entity::is_active(id) - Verificar si está activa
            if func_name == "entity::is_active" && args.len() == 1 {
                use crate::modules::entity;
                return entity::entity_is_active(args, executor, funcs);
            }

            // player::set_speed(id, speed) - Velocidad
            if func_name == "player::set_speed" && args.len() == 2 {
                use crate::modules::entity;
                return entity::player_set_speed(args, executor, funcs);
            }

            // player::get_speed(id) - Obtener velocidad
            if func_name == "player::get_speed" && args.len() == 1 {
                use crate::modules::entity;
                return entity::player_get_speed(args, executor, funcs);
            }

            // player::move_left(id) - Mover izquierda
            if func_name == "player::move_left" && args.len() == 1 {
                use crate::modules::entity;
                return entity::player_move_left(args, executor, funcs);
            }

            // player::move_right(id) - Mover derecha
            if func_name == "player::move_right" && args.len() == 1 {
                use crate::modules::entity;
                return entity::player_move_right(args, executor, funcs);
            }

            // player::move_up(id) - Mover arriba
            if func_name == "player::move_up" && args.len() == 1 {
                use crate::modules::entity;
                return entity::player_move_up(args, executor, funcs);
            }

            // player::move_down(id) - Mover abajo
            if func_name == "player::move_down" && args.len() == 1 {
                use crate::modules::entity;
                return entity::player_move_down(args, executor, funcs);
            }

            // player::jump(id) - Saltar
            if func_name == "player::jump" && args.len() == 1 {
                use crate::modules::entity;
                return entity::player_jump(args, executor, funcs);
            }

            // player::apply_gravity(id) - Aplicar gravedad
            if func_name == "player::apply_gravity" && args.len() == 1 {
                use crate::modules::entity;
                return entity::player_apply_gravity(args, executor, funcs);
            }

            // player::get_state(id) - Obtener estado
            if func_name == "player::get_state" && args.len() == 1 {
                use crate::modules::entity;
                return entity::player_get_state(args, executor, funcs);
            }

            // player::is_grounded(id) - Verificar suelo
            if func_name == "player::is_grounded" && args.len() == 1 {
                use crate::modules::entity;
                return entity::player_is_grounded(args, executor, funcs);
            }

            // player::set_health(id, health) - Establecer vida
            if func_name == "player::set_health" && args.len() == 2 {
                use crate::modules::entity;
                return entity::player_set_health(args, executor, funcs);
            }

            // player::get_health(id) - Obtener vida
            if func_name == "player::get_health" && args.len() == 1 {
                use crate::modules::entity;
                return entity::player_get_health(args, executor, funcs);
            }

            // player::take_damage(id, amount) - Recibir daño
            if func_name == "player::take_damage" && args.len() == 2 {
                use crate::modules::entity;
                return entity::player_take_damage(args, executor, funcs);
            }

            // enemy::set_ai_type(id, ai_type) - Tipo de IA
            if func_name == "enemy::set_ai_type" && args.len() == 2 {
                use crate::modules::entity;
                return entity::enemy_set_ai_type(args, executor, funcs);
            }

            // enemy::set_patrol_points(id, points) - Puntos de patrulla
            if func_name == "enemy::set_patrol_points" && args.len() == 2 {
                use crate::modules::entity;
                return entity::enemy_set_patrol_points(args, executor, funcs);
            }

            // enemy::set_detection_range(id, range) - Rango de detección
            if func_name == "enemy::set_detection_range" && args.len() == 2 {
                use crate::modules::entity;
                return entity::enemy_set_detection_range(args, executor, funcs);
            }

            // enemy::update_ai(id, player_id) - Actualizar IA
            if func_name == "enemy::update_ai" && args.len() == 2 {
                use crate::modules::entity;
                return entity::enemy_update_ai(args, executor, funcs);
            }

            // enemy::is_alerted(id) - Verificar alerta
            if func_name == "enemy::is_alerted" && args.len() == 1 {
                use crate::modules::entity;
                return entity::enemy_is_alerted(args, executor, funcs);
            }

            // enemy::set_health(id, health) - Vida
            if func_name == "enemy::set_health" && args.len() == 2 {
                use crate::modules::entity;
                return entity::enemy_set_health(args, executor, funcs);
            }

            // enemy::set_damage(id, damage) - Daño
            if func_name == "enemy::set_damage" && args.len() == 2 {
                use crate::modules::entity;
                return entity::enemy_set_damage(args, executor, funcs);
            }

            // enemy::set_reward(id, coins) - Recompensa
            if func_name == "enemy::set_reward" && args.len() == 2 {
                use crate::modules::entity;
                return entity::enemy_set_reward(args, executor, funcs);
            }

            // boss::set_phases(id, phases) - Establecer fases
            if func_name == "boss::set_phases" && args.len() == 2 {
                use crate::modules::entity;
                return entity::boss_set_phases(args, executor, funcs);
            }

            // boss::get_current_phase(id) - Fase actual
            if func_name == "boss::get_current_phase" && args.len() == 1 {
                use crate::modules::entity;
                return entity::boss_get_current_phase(args, executor, funcs);
            }

            // boss::transition_to_phase(id, phase) - Transición de fase
            if func_name == "boss::transition_to_phase" && args.len() == 2 {
                use crate::modules::entity;
                return entity::boss_transition_to_phase(args, executor, funcs);
            }

            // boss::set_arena_bounds(id, min_x, min_y, max_x, max_y) - Arena
            if func_name == "boss::set_arena_bounds" && args.len() == 5 {
                use crate::modules::entity;
                return entity::boss_set_arena_bounds(args, executor, funcs);
            }

            // ========================================================================
            // COLLISION SYSTEM (FASE 2D)
            // ========================================================================

            // collision::check_rect_rect(...) - Colisión rectángulo
            if func_name == "collision::check_rect_rect" && args.len() == 8 {
                use crate::modules::entity;
                return entity::collision_check_rect_rect(args, executor, funcs);
            }

            // collision::check_circle_circle(...) - Colisión círculo
            if func_name == "collision::check_circle_circle" && args.len() == 6 {
                use crate::modules::entity;
                return entity::collision_check_circle_circle(args, executor, funcs);
            }

            // collision::check_rect_circle(...) - Colisión rect-círculo
            if func_name == "collision::check_rect_circle" && args.len() == 7 {
                use crate::modules::entity;
                return entity::collision_check_rect_circle(args, executor, funcs);
            }

            // collision::check_point_rect(...) - Punto en rectángulo
            if func_name == "collision::check_point_rect" && args.len() == 6 {
                use crate::modules::entity;
                return entity::collision_check_point_rect(args, executor, funcs);
            }

            // collision::check(id1, id2) - Colisión entre entidades
            if func_name == "collision::check" && args.len() == 2 {
                use crate::modules::entity;
                return entity::collision_check(args, executor, funcs);
            }

            // area2d::create(x, y, w, h) - Crear área 2D
            if func_name == "area2d::create" && args.len() == 4 {
                use crate::modules::entity;
                return entity::area2d_create(args, executor, funcs);
            }

            // area2d::set_position(id, x, y) - Mover área
            if func_name == "area2d::set_position" && args.len() == 3 {
                use crate::modules::entity;
                return entity::area2d_set_position(args, executor, funcs);
            }

            // area2d::get_position(id) - Obtener posición
            if func_name == "area2d::get_position" && args.len() == 1 {
                use crate::modules::entity;
                return entity::area2d_get_position(args, executor, funcs);
            }

            // area2d::check(id1, id2) - Verificar colisión
            if func_name == "area2d::check" && args.len() == 2 {
                use crate::modules::entity;
                return entity::area2d_check(args, executor, funcs);
            }

            // area2d::get_overlapping(id) - Obtener áreas superpuestas
            if func_name == "area2d::get_overlapping" && args.len() == 1 {
                use crate::modules::entity;
                return entity::area2d_get_overlapping(args, executor, funcs);
            }

            // area2d::destroy(id) - Destruir área
            if func_name == "area2d::destroy" && args.len() == 1 {
                use crate::modules::entity;
                return entity::area2d_destroy(args, executor, funcs);
            }

            // ========================================================================
            // TRAP COMPONENT (FASE 2C)
            // ========================================================================

            // trap::set_type(id, type) - Tipo de trampa
            if func_name == "trap::set_type" && args.len() == 2 {
                use crate::modules::entity;
                return entity::trap_set_type(args, executor, funcs);
            }

            // trap::set_damage(id, damage) - Daño
            if func_name == "trap::set_damage" && args.len() == 2 {
                use crate::modules::entity;
                return entity::trap_set_damage(args, executor, funcs);
            }

            // trap::set_trigger_range(id, range) - Rango
            if func_name == "trap::set_trigger_range" && args.len() == 2 {
                use crate::modules::entity;
                return entity::trap_set_trigger_range(args, executor, funcs);
            }

            // trap::set_visible(id, visible) - Visible/invisible
            if func_name == "trap::set_visible" && args.len() == 2 {
                use crate::modules::entity;
                return entity::trap_set_visible(args, executor, funcs);
            }

            // trap::is_triggered(id) - Verificar activación
            if func_name == "trap::is_triggered" && args.len() == 1 {
                use crate::modules::entity;
                return entity::trap_is_triggered(args, executor, funcs);
            }

            // trap::activate(id) - Activar
            if func_name == "trap::activate" && args.len() == 1 {
                use crate::modules::entity;
                return entity::trap_activate(args, executor, funcs);
            }

            // trap::reset(id) - Resetear
            if func_name == "trap::reset" && args.len() == 1 {
                use crate::modules::entity;
                return entity::trap_reset(args, executor, funcs);
            }

            // ========================================================================
            // COIN COMPONENT (FASE 2C)
            // ========================================================================

            // coin::set_value(id, value) - Valor de moneda
            if func_name == "coin::set_value" && args.len() == 2 {
                use crate::modules::entity;
                return entity::coin_set_value(args, executor, funcs);
            }

            // coin::set_type(id, type) - Tipo de moneda
            if func_name == "coin::set_type" && args.len() == 2 {
                use crate::modules::entity;
                return entity::coin_set_type(args, executor, funcs);
            }

            // coin::get_value(id) - Obtener valor
            if func_name == "coin::get_value" && args.len() == 1 {
                use crate::modules::entity;
                return entity::coin_get_value(args, executor, funcs);
            }

            // coin::collect(id, player_id) - Recolectar
            if func_name == "coin::collect" && args.len() == 2 {
                use crate::modules::entity;
                return entity::coin_collect(args, executor, funcs);
            }

            // coin::is_collected(id) - Verificar recolección
            if func_name == "coin::is_collected" && args.len() == 1 {
                use crate::modules::entity;
                return entity::coin_is_collected(args, executor, funcs);
            }

            // --- INPUT MAP (v0.5.1) ---
            // input_map::register(combo, action) - Registrar combinación
            if func_name == "input_map::register" && args.len() == 2 {
                use crate::modules::input_map;
                return input_map::input_map_register(args, executor, funcs);
            }

            // input_map::list() - Listar combinaciones
            if func_name == "input_map::list" {
                use crate::modules::input_map;
                return input_map::input_map_list(args, executor, funcs);
            }

            // input_map::clear() - Limpiar combinaciones
            if func_name == "input_map::clear" {
                use crate::modules::input_map;
                return input_map::input_map_clear(args, executor, funcs);
            }

            // input_map::count() - Cantidad de combinaciones
            if func_name == "input_map::count" {
                use crate::modules::input_map;
                return input_map::input_map_count(args, executor, funcs);
            }

            // input_map::press(key) - Registrar tecla presionada
            if func_name == "input_map::press" && args.len() == 1 {
                use crate::modules::input_map;
                return input_map::input_map_press(args, executor, funcs);
            }

            // input_map::release(key) - Registrar tecla soltada
            if func_name == "input_map::release" && args.len() == 1 {
                use crate::modules::input_map;
                return input_map::input_map_release(args, executor, funcs);
            }

            // input_map::is_pressed(action) - Verificar si acción está presionada
            if func_name == "input_map::is_pressed" && args.len() == 1 {
                use crate::modules::input_map;
                return input_map::input_map_is_pressed(args, executor, funcs);
            }

            // input_map::get_active() - Obtener acciones activas
            if func_name == "input_map::get_active" {
                use crate::modules::input_map;
                return input_map::input_map_get_active(args, executor, funcs);
            }

            // ========================================================================
            // GAMEPAD INPUT (v0.9.5)
            // ========================================================================

            // gamepad::is_connected() - Verificar gamepad
            if func_name == "gamepad::is_connected" {
                use crate::modules::input_map;
                return input_map::gamepad_is_connected(args, executor, funcs);
            }

            // gamepad::press_button(button) - Presionar botón
            if func_name == "gamepad::press_button" && args.len() == 1 {
                use crate::modules::input_map;
                return input_map::gamepad_press_button(args, executor, funcs);
            }

            // gamepad::release_button(button) - Soltar botón
            if func_name == "gamepad::release_button" && args.len() == 1 {
                use crate::modules::input_map;
                return input_map::gamepad_release_button(args, executor, funcs);
            }

            // gamepad::is_pressed(button) - Verificar botón presionado
            if func_name == "gamepad::is_pressed" && args.len() == 1 {
                use crate::modules::input_map;
                return input_map::gamepad_is_pressed(args, executor, funcs);
            }

            // gamepad::get_axis(stick) - Obtener eje analógico
            if func_name == "gamepad::get_axis" && args.len() == 1 {
                use crate::modules::input_map;
                return input_map::gamepad_get_axis(args, executor, funcs);
            }

            // --- INPUT IME (v0.9.2) ---
            // input::show_keyboard() - Mostrar teclado virtual
            if func_name == "input::show_keyboard" {
                use crate::modules::input_ime;
                return input_ime::input_show_keyboard(args, executor, funcs);
            }

            // input::hide_keyboard() - Ocultar teclado virtual
            if func_name == "input::hide_keyboard" {
                use crate::modules::input_ime;
                return input_ime::input_hide_keyboard(args, executor, funcs);
            }

            // input::get_text() - Obtener texto ingresado
            if func_name == "input::get_text" {
                use crate::modules::input_ime;
                return input_ime::input_get_text(args, executor, funcs);
            }

            // input::has_text() - Verificar si hay texto nuevo
            if func_name == "input::has_text" {
                use crate::modules::input_ime;
                return input_ime::input_has_text(args, executor, funcs);
            }

            // input::clear_text() - Limpiar texto
            if func_name == "input::clear_text" {
                use crate::modules::input_ime;
                return input_ime::input_clear_text(args, executor, funcs);
            }

            // input::text(prompt) - Mostrar teclado y esperar input
            if func_name == "input::text" {
                use crate::modules::input_ime;
                return input_ime::input_text(args, executor, funcs);
            }

            // input::is_keyboard_visible() - Verificar si teclado está visible
            if func_name == "input::is_keyboard_visible" {
                use crate::modules::input_ime;
                return input_ime::input_is_keyboard_visible(args, executor, funcs);
            }

            // input::simulate_text(text) - Simular input (para demos)
            if func_name == "input::simulate_text" {
                use crate::modules::input_ime;
                return input_ime::input_simulate_text(args, executor, funcs);
            }

            // --- PHYSICS (v0.9.3) ---
            // physics::create_body(id, x, y, w, h) - Crear cuerpo físico
            if func_name == "physics::create_body" && args.len() == 5 {
                use crate::modules::physics;
                return physics::physics_create_body(args, executor, funcs);
            }

            // physics::update(dt) - Actualizar mundo físico
            if func_name == "physics::update" {
                use crate::modules::physics;
                return physics::physics_update(args, executor, funcs);
            }

            // physics::get_position(id) - Obtener posición
            if func_name == "physics::get_position" && args.len() == 1 {
                use crate::modules::physics;
                return physics::physics_get_position(args, executor, funcs);
            }

            // physics::set_position(id, x, y) - Establecer posición
            if func_name == "physics::set_position" && args.len() == 3 {
                use crate::modules::physics;
                return physics::physics_set_position(args, executor, funcs);
            }

            // physics::set_velocity(id, vx, vy) - Establecer velocidad
            if func_name == "physics::set_velocity" && args.len() == 3 {
                use crate::modules::physics;
                return physics::physics_set_velocity(args, executor, funcs);
            }

            // physics::apply_gravity(id) - Aplicar gravedad
            if func_name == "physics::apply_gravity" && args.len() == 1 {
                use crate::modules::physics;
                return physics::physics_apply_gravity(args, executor, funcs);
            }

            // physics::set_bounds(x, y, w, h) - Establecer límites
            if func_name == "physics::set_bounds" && args.len() == 4 {
                use crate::modules::physics;
                return physics::physics_set_bounds(args, executor, funcs);
            }

            // physics::check_collision(id_a, id_b) - Verificar colisión
            if func_name == "physics::check_collision" && args.len() == 2 {
                use crate::modules::physics;
                return physics::physics_check_collision(args, executor, funcs);
            }

            // --- STATISTICS: MEAN ---
            if func_name == "stats::mean" && args.len() == 1 {
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
            if func_name == "stats::median" && args.len() == 1 {
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
            if func_name == "stats::std_dev" && args.len() == 1 {
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
            if func_name == "stats::min" && args.len() == 1 {
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
            if func_name == "stats::max" && args.len() == 1 {
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
            if func_name == "plot::ascii_chart" && args.len() == 2 {
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
            if func_name == "plot::svg_chart" && args.len() == 3 {
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
                    let first_y = h
                        - padding
                        - ((nums[0] - min_val) / range * (h - 2 * padding) as f64) as i32;
                    let mut path = format!(
                        "<polyline points='{},{}' fill='none' stroke='blue' stroke-width='2'/>",
                        padding, first_y
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
            if func_name == "bezier::linear" && args.len() == 5 {
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
            if func_name == "bezier::quadratic" && args.len() == 7 {
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
            if func_name == "bezier::cubic" && args.len() == 9 {
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
            if func_name == "bezier::cubic_derivative" && args.len() == 9 {
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
            if func_name == "bezier::generate_points" && args.len() == 2 {
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

            // ========================================================================
            // MÓDULOS RYDIT - v0.10.4: Reconexión Total
            // ========================================================================

            // ASSETS::load(id, path) - Cargar textura
            if func_name == "assets::load" || func_name == "assets::sprite" {
                return assets_load(args, executor, funcs);
            }

            // ASSETS::draw(id, x, y) - Dibujar sprite
            if func_name == "assets::draw" {
                return assets_draw(args, executor, funcs);
            }

            // CAMERA::follow(entity_id) - Cámara sigue entidad
            if func_name == "camera::follow" {
                return camera_follow(args, executor, funcs);
            }

            // CAMERA::set_zoom(zoom) - Set zoom de cámara
            if func_name == "camera::set_zoom" {
                return camera_set_zoom(args, executor, funcs);
            }

            // CAMERA::set_position(x, y) - Set posición de cámara
            if func_name == "camera::set_position" {
                return camera_set_position(args, executor, funcs);
            }

            // AUDIO::play(sound_id) - Reproducir sonido
            if func_name == "audio::play" {
                return audio_play(args, executor, funcs);
            }

            // AUDIO::stop(sound_id) - Detener sonido
            if func_name == "audio::stop" {
                return audio_stop(args, executor, funcs);
            }

            // AUDIO::set_volume(sound_id, volume) - Set volumen
            // ⚠️ Pendiente: implementar en audio.rs
            // if func_name == "audio::set_volume" {
            //     return audio_set_volume(args, executor, funcs);
            // }

            // PHYSICS::apply_gravity(entity_id) - Aplicar gravedad
            if func_name == "physics::apply_gravity" {
                return physics_apply_gravity(args, executor, funcs);
            }

            // PHYSICS::resolve_collision(entity_a, entity_b) - Resolver colisión
            // ⚠️ Pendiente: implementar en physics.rs
            // if func_name == "physics::resolve_collision" {
            //     return physics_resolve_collision(args, executor, funcs);
            // }

            // INPUT_MAP::bind(key, action) - Mapear tecla a acción
            if func_name == "input_map::bind" || func_name == "input_map::register" {
                return input_map_register(args, executor, funcs);
            }

            // INPUT_MAP::is_pressed(action) - Verificar si acción está presionada
            if func_name == "input_map::is_pressed" {
                return input_map_is_pressed(args, executor, funcs);
            }

            // ENTITY::create(id, type, x, y) - Crear entidad
            if func_name == "entity::create" {
                return entity_create(args, executor, funcs);
            }

            // ENTITY::get_position(id) - Obtener posición de entidad
            if func_name == "entity::get_position" {
                return entity_get_position(args, executor, funcs);
            }

            Valor::Error(format!("Función '{}' no soportada en expresiones", func_name))
        }
        Expr::Binary { left, op, right } => {
            let left_val = evaluar_expr(left, executor, funcs);
            let right_val = evaluar_expr(right, executor, funcs);

            // Operadores lógicos (usan referencias, no mueven valores)
            match op {
                BinaryOp::And => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool && r_bool);
                }
                BinaryOp::Or => {
                    let l_bool = valor_a_bool(&left_val);
                    let r_bool = valor_a_bool(&right_val);
                    return Valor::Bool(l_bool || r_bool);
                }
                _ => {}
            }

            // Concatenación de strings con + (con coerción automática de números)
            if matches!(op, BinaryOp::Suma) {
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
                    BinaryOp::Suma => Valor::Num(l + r),
                    BinaryOp::Resta => Valor::Num(l - r),
                    BinaryOp::Mult => Valor::Num(l * r),
                    BinaryOp::Div => {
                        if r != 0.0 {
                            Valor::Num(l / r)
                        } else {
                            Valor::Error("División por cero".to_string())
                        }
                    }
                    BinaryOp::Mayor => Valor::Bool(l > r),
                    BinaryOp::Menor => Valor::Bool(l < r),
                    BinaryOp::Igual => Valor::Bool((l - r).abs() < 0.0001),
                    BinaryOp::MayorIgual => Valor::Bool(l >= r),
                    BinaryOp::MenorIgual => Valor::Bool(l <= r),
                    _ => Valor::Error("Operador no soportado".to_string()),
                };
            }

            Valor::Error("Operación inválida".to_string())
        }
        Expr::Unary { op, expr } => {
            let val = evaluar_expr(expr, executor, funcs);
            match op {
                UnaryOp::Not => {
                    let b = valor_a_bool(&val);
                    Valor::Bool(!b)
                }
                UnaryOp::Neg => {
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
