// crates/rydit-rs/src/tests/mod.rs
// Tests del núcleo de RyDit

#[cfg(test)]
mod warning_tests {
    use crate::*;

    #[test]
    fn test_division_por_cero() {
        // Verificar que división por cero retorna Error, no panic
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let left = Expr::Num(10.0);
        let right = Expr::Num(0.0);
        let expr = Expr::BinOp {
            left: Box::new(left),
            op: lizer::BinOp::Div,
            right: Box::new(right),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert!(matches!(result, Valor::Error(_)));
    }

    #[test]
    fn test_division_normal() {
        // Verificar que división normal funciona
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let left = Expr::Num(10.0);
        let right = Expr::Num(2.0);
        let expr = Expr::BinOp {
            left: Box::new(left),
            op: lizer::BinOp::Div,
            right: Box::new(right),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert!(matches!(result, Valor::Num(5.0)));
    }

    // ========================================================================
    // TESTS V0.1.9 - CONCATENACIÓN Y SÍMBOLOS
    // ========================================================================

    #[test]
    fn test_concatenacion_string_numero() {
        // "x=" + 42 -> "x=42"
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let left = Expr::Texto("x=".to_string());
        let right = Expr::Num(42.0);
        let expr = Expr::BinOp {
            left: Box::new(left),
            op: lizer::BinOp::Suma,
            right: Box::new(right),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Texto("x=42".to_string()));
    }

    #[test]
    fn test_concatenacion_numero_string() {
        // 42 + "x" -> "42x"
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let left = Expr::Num(42.0);
        let right = Expr::Texto("x".to_string());
        let expr = Expr::BinOp {
            left: Box::new(left),
            op: lizer::BinOp::Suma,
            right: Box::new(right),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Texto("42x".to_string()));
    }

    #[test]
    fn test_concatenacion_multiple() {
        // "a"+1+"b"+2 -> "a1b2"
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        // "a" + 1 -> "a1"
        let expr1 = Expr::BinOp {
            left: Box::new(Expr::Texto("a".to_string())),
            op: lizer::BinOp::Suma,
            right: Box::new(Expr::Num(1.0)),
        };
        let result1 = evaluar_expr(&expr1, &mut executor, &mut funcs);
        assert_eq!(result1, Valor::Texto("a1".to_string()));

        // "a1" + "b" -> "a1b"
        let expr2 = Expr::BinOp {
            left: Box::new(Expr::Texto("a1".to_string())),
            op: lizer::BinOp::Suma,
            right: Box::new(Expr::Texto("b".to_string())),
        };
        let result2 = evaluar_expr(&expr2, &mut executor, &mut funcs);
        assert_eq!(result2, Valor::Texto("a1b".to_string()));

        // "a1b" + 2 -> "a1b2"
        let expr3 = Expr::BinOp {
            left: Box::new(Expr::Texto("a1b".to_string())),
            op: lizer::BinOp::Suma,
            right: Box::new(Expr::Num(2.0)),
        };
        let result3 = evaluar_expr(&expr3, &mut executor, &mut funcs);
        assert_eq!(result3, Valor::Texto("a1b2".to_string()));
    }

    #[test]
    fn test_concatenacion_con_expresion() {
        // "total: " + (2+3)*4 -> "total: 20"
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        // (2+3)*4 = 20
        let inner = Expr::BinOp {
            left: Box::new(Expr::Num(2.0)),
            op: lizer::BinOp::Suma,
            right: Box::new(Expr::Num(3.0)),
        };
        let expr_mult = Expr::BinOp {
            left: Box::new(inner),
            op: lizer::BinOp::Mult,
            right: Box::new(Expr::Num(4.0)),
        };

        // "total: " + 20
        let expr = Expr::BinOp {
            left: Box::new(Expr::Texto("total: ".to_string())),
            op: lizer::BinOp::Suma,
            right: Box::new(expr_mult),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Texto("total: 20".to_string()));
    }

    #[test]
    fn test_variable_dolar_asignacion() {
        // $x = 10 debe guardarse correctamente
        let mut executor = Executor::nuevo();
        executor.guardar("$x", Valor::Num(10.0));
        let result = executor.leer("$x");
        assert_eq!(result, Some(Valor::Num(10.0)));
    }

    #[test]
    fn test_variable_arroba_lectura() {
        // @user debe leerse correctamente
        let mut executor = Executor::nuevo();
        executor.guardar("@user", Valor::Texto("alucard18".to_string()));
        let result = executor.leer("@user");
        assert_eq!(result, Some(Valor::Texto("alucard18".to_string())));
    }

    #[test]
    fn test_variable_porcentaje_expresion() {
        // %p = 50 + 25 -> 75
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let expr = Expr::BinOp {
            left: Box::new(Expr::Num(50.0)),
            op: lizer::BinOp::Suma,
            right: Box::new(Expr::Num(25.0)),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        executor.guardar("%p", result);

        assert_eq!(executor.leer("%p"), Some(Valor::Num(75.0)));
    }

    #[test]
    fn test_simbolos_en_array() {
        // [$a, $b] debe evaluarse como array
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        executor.guardar("$a", Valor::Num(1.0));
        executor.guardar("$b", Valor::Num(2.0));

        let expr = Expr::Array(vec![
            Expr::Var("$a".to_string()),
            Expr::Var("$b".to_string()),
        ]);

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 2);
            assert_eq!(arr[0], Valor::Num(1.0));
            assert_eq!(arr[1], Valor::Num(2.0));
        } else {
            panic!("Expected Array, got {:?}", result);
        }
    }

    #[test]
    fn test_concatenacion_string_string() {
        // "hello" + "world" -> "helloworld"
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let left = Expr::Texto("hello".to_string());
        let right = Expr::Texto("world".to_string());
        let expr = Expr::BinOp {
            left: Box::new(left),
            op: lizer::BinOp::Suma,
            right: Box::new(right),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Texto("helloworld".to_string()));
    }

    #[test]
    fn test_suma_aritmetica_no_se_afecta() {
        // 2 + 3 debe seguir siendo 5 (no concatenación)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let left = Expr::Num(2.0);
        let right = Expr::Num(3.0);
        let expr = Expr::BinOp {
            left: Box::new(left),
            op: lizer::BinOp::Suma,
            right: Box::new(right),
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Num(5.0));
    }

    // ========================================================================
    // TESTS V0.6.2 - MÓDULO REGEX
    // ========================================================================

    #[test]
    fn test_regex_match_valido() {
        // regex::match("[a-z]+", "hola") -> true
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto("[a-z]+".to_string()),
            Expr::Texto("hola".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::match".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Bool(true));
    }

    #[test]
    fn test_regex_match_invalido() {
        // regex::match("\\d+", "abc") -> false
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto("\\d+".to_string()),
            Expr::Texto("abc".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::match".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Bool(false));
    }

    #[test]
    fn test_regex_replace() {
        // regex::replace("[aeiou]", "*", "hola") -> "h*l*"
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto("[aeiou]".to_string()),
            Expr::Texto("*".to_string()),
            Expr::Texto("hola".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::replace".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Texto("h*l*".to_string()));
    }

    #[test]
    fn test_regex_split() {
        // regex::split(",", "uno,dos,tres") -> ["uno", "dos", "tres"]
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto(",".to_string()),
            Expr::Texto("uno,dos,tres".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::split".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Valor::Texto("uno".to_string()));
            assert_eq!(arr[1], Valor::Texto("dos".to_string()));
            assert_eq!(arr[2], Valor::Texto("tres".to_string()));
        } else {
            panic!("Expected Array, got {:?}", result);
        }
    }

    #[test]
    fn test_regex_find_all() {
        // regex::find_all("\\d+", "a1b23c456") -> ["1", "23", "456"]
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto("\\d+".to_string()),
            Expr::Texto("a1b23c456".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::find_all".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Valor::Texto("1".to_string()));
            assert_eq!(arr[1], Valor::Texto("23".to_string()));
            assert_eq!(arr[2], Valor::Texto("456".to_string()));
        } else {
            panic!("Expected Array, got {:?}", result);
        }
    }

    #[test]
    fn test_regex_capture() {
        // regex::capture("([a-z]+):(\\d+)", "edad:25") -> ["edad:25", "edad", "25"]
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto("([a-z]+):(\\d+)".to_string()),
            Expr::Texto("edad:25".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::capture".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr[0], Valor::Texto("edad:25".to_string()));
            assert_eq!(arr[1], Valor::Texto("edad".to_string()));
            assert_eq!(arr[2], Valor::Texto("25".to_string()));
        } else {
            panic!("Expected Array, got {:?}", result);
        }
    }

    #[test]
    fn test_regex_email_validation() {
        // Validar email real
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();
        let args = vec![
            Expr::Texto("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}".to_string()),
            Expr::Texto("usuario@ejemplo.com".to_string()),
        ];
        let expr = Expr::Call {
            name: "regex::match".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);
        assert_eq!(result, Valor::Bool(true));
    }

    // ========================================================================
    // TESTS V0.6.3 - MÓDULO FILES
    // ========================================================================

    #[test]
    fn test_files_write_and_read() {
        // Escribir y leer archivo
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        // Write
        let write_args = vec![
            Expr::Texto("test_rydit.txt".to_string()),
            Expr::Texto("Hola RyDit".to_string()),
        ];
        let write_expr = Expr::Call {
            name: "files::write".to_string(),
            args: write_args,
        };
        let write_result = evaluar_expr(&write_expr, &mut executor, &mut funcs);
        assert_eq!(write_result, Valor::Bool(true));

        // Read
        let read_args = vec![Expr::Texto("test_rydit.txt".to_string())];
        let read_expr = Expr::Call {
            name: "files::read".to_string(),
            args: read_args,
        };
        let read_result = evaluar_expr(&read_expr, &mut executor, &mut funcs);
        assert_eq!(read_result, Valor::Texto("Hola RyDit".to_string()));

        // Cleanup
        std::fs::remove_file("test_rydit.txt").ok();
    }

    #[test]
    fn test_files_append() {
        // Append a archivo
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        // Write inicial
        std::fs::write("test_append.txt", "Linea 1");

        // Append
        let append_args = vec![
            Expr::Texto("test_append.txt".to_string()),
            Expr::Texto("\nLinea 2".to_string()),
        ];
        let append_expr = Expr::Call {
            name: "files::append".to_string(),
            args: append_args,
        };
        let append_result = evaluar_expr(&append_expr, &mut executor, &mut funcs);
        assert_eq!(append_result, Valor::Bool(true));

        // Verify
        let content = std::fs::read_to_string("test_append.txt").unwrap();
        assert_eq!(content, "Linea 1\nLinea 2");

        // Cleanup
        std::fs::remove_file("test_append.txt").ok();
    }

    #[test]
    fn test_files_exists() {
        // Verificar existencia
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        // Crear archivo
        std::fs::write("test_exists.txt", "test");

        // Exists - true
        let exists_args = vec![Expr::Texto("test_exists.txt".to_string())];
        let exists_expr = Expr::Call {
            name: "files::exists".to_string(),
            args: exists_args,
        };
        let exists_result = evaluar_expr(&exists_expr, &mut executor, &mut funcs);
        assert_eq!(exists_result, Valor::Bool(true));

        // Exists - false
        let not_exists_args = vec![Expr::Texto("no_existe.txt".to_string())];
        let not_exists_expr = Expr::Call {
            name: "files::exists".to_string(),
            args: not_exists_args,
        };
        let not_exists_result = evaluar_expr(&not_exists_expr, &mut executor, &mut funcs);
        assert_eq!(not_exists_result, Valor::Bool(false));

        // Cleanup
        std::fs::remove_file("test_exists.txt").ok();
    }

    #[test]
    fn test_files_delete() {
        // Eliminar archivo
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        // Crear archivo
        std::fs::write("test_delete.txt", "para eliminar");

        // Delete
        let delete_args = vec![Expr::Texto("test_delete.txt".to_string())];
        let delete_expr = Expr::Call {
            name: "files::delete".to_string(),
            args: delete_args,
        };
        let delete_result = evaluar_expr(&delete_expr, &mut executor, &mut funcs);
        assert_eq!(delete_result, Valor::Bool(true));

        // Verify deleted
        assert!(!std::path::Path::new("test_delete.txt").exists());
    }

    // ========================================================================
    // TESTS V0.7.1.1 - ANIMACIÓN 2D
    // ========

    #[test]
    fn test_anim_ease_in() {
        // anim::ease_in(0.5) = 0.25 (quadratic)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![Expr::Num(0.5)];
        let expr = Expr::Call {
            name: "anim::ease_in".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Num(val) = result {
            assert!(
                (val - 0.25).abs() < 0.001,
                "ease_in(0.5) debería ser 0.25, fue {}",
                val
            );
        } else {
            panic!("ease_in debería retornar Num, fue {:?}", result);
        }
    }

    #[test]
    fn test_anim_ease_out() {
        // anim::ease_out(0.5) = 0.75 (quadratic)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![Expr::Num(0.5)];
        let expr = Expr::Call {
            name: "anim::ease_out".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Num(val) = result {
            assert!(
                (val - 0.75).abs() < 0.001,
                "ease_out(0.5) debería ser 0.75, fue {}",
                val
            );
        } else {
            panic!("ease_out debería retornar Num, fue {:?}", result);
        }
    }

    #[test]
    fn test_anim_ease_in_out() {
        // anim::ease_in_out(0.5) = 0.5 (punto medio)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![Expr::Num(0.5)];
        let expr = Expr::Call {
            name: "anim::ease_in_out".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Num(val) = result {
            assert!(
                (val - 0.5).abs() < 0.001,
                "ease_in_out(0.5) debería ser 0.5, fue {}",
                val
            );
        } else {
            panic!("ease_in_out debería retornar Num, fue {:?}", result);
        }
    }

    #[test]
    fn test_anim_squash() {
        // anim::squash(2.0) = [2.0, 0.5] (mantiene área)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![Expr::Num(2.0)];
        let expr = Expr::Call {
            name: "anim::squash".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 2, "squash debería retornar array de 2 elementos");
            if let (Valor::Num(x), Valor::Num(y)) = (&arr[0], &arr[1]) {
                assert!((x - 2.0).abs() < 0.001, "squash X debería ser 2.0");
                assert!((y - 0.5).abs() < 0.001, "squash Y debería ser 0.5");
            }
        } else {
            panic!("squash debería retornar Array, fue {:?}", result);
        }
    }

    #[test]
    fn test_anim_stretch() {
        // anim::stretch(2.0) = [0.5, 2.0] (mantiene área)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![Expr::Num(2.0)];
        let expr = Expr::Call {
            name: "anim::stretch".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(
                arr.len(),
                2,
                "stretch debería retornar array de 2 elementos"
            );
            if let (Valor::Num(x), Valor::Num(y)) = (&arr[0], &arr[1]) {
                assert!((x - 0.5).abs() < 0.001, "stretch X debería ser 0.5");
                assert!((y - 2.0).abs() < 0.001, "stretch Y debería ser 2.0");
            }
        } else {
            panic!("stretch debería retornar Array, fue {:?}", result);
        }
    }

    #[test]
    fn test_anim_anticipate() {
        // anim::anticipate(100, 200, 20) = 80 (retrocede antes de avanzar)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![Expr::Num(100.0), Expr::Num(200.0), Expr::Num(20.0)];
        let expr = Expr::Call {
            name: "anim::anticipate".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Num(val) = result {
            assert!(
                (val - 80.0).abs() < 0.001,
                "anticipate(100, 200, 20) debería ser 80, fue {}",
                val
            );
        } else {
            panic!("anticipate debería retornar Num, fue {:?}", result);
        }
    }

    #[test]
    fn test_illusion_muller_lyer() {
        // illusion::muller_lyer(100, 200, 200, true)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![
            Expr::Num(100.0),
            Expr::Num(200.0),
            Expr::Num(200.0),
            Expr::Bool(true),
        ];
        let expr = Expr::Call {
            name: "illusion::muller_lyer".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 4, "muller_lyer debería retornar 4 elementos");
        } else {
            panic!("muller_lyer debería retornar Array, fue {:?}", result);
        }
    }

    #[test]
    fn test_illusion_phi_effect() {
        // illusion::phi_effect con movimiento
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![
            Expr::Num(100.0), // x1
            Expr::Num(300.0), // y1
            Expr::Num(700.0), // x2
            Expr::Num(300.0), // y2
            Expr::Num(3.0),   // speed
            Expr::Num(50.0),  // frame
        ];
        let expr = Expr::Call {
            name: "illusion::phi_effect".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(
                arr.len(),
                3,
                "phi_effect debería retornar 3 elementos [x, y, direction]"
            );
        } else {
            panic!("phi_effect debería retornar Array, fue {:?}", result);
        }
    }

    // ========================================================================
    // TESTS V0.7.1.2 - FÍSICA
    // ========================================================================

    #[test]
    fn test_physics_projectile() {
        // physics::projectile(0, 0, 50, 45)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![
            Expr::Num(0.0),  // x0
            Expr::Num(0.0),  // y0
            Expr::Num(50.0), // v0
            Expr::Num(45.0), // angle
        ];
        let expr = Expr::Call {
            name: "physics::projectile".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 5, "projectile debería retornar 5 elementos");
            // Verificar que todos sean números
            for val in &arr {
                assert!(
                    matches!(val, Valor::Num(_)),
                    "Todos los elementos deberían ser números"
                );
            }
        } else {
            panic!("projectile debería retornar Array, fue {:?}", result);
        }
    }

    #[test]
    fn test_physics_projectile_at() {
        // physics::projectile_at(0, 0, 50, 45, 2)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![
            Expr::Num(0.0),  // x0
            Expr::Num(0.0),  // y0
            Expr::Num(50.0), // v0
            Expr::Num(45.0), // angle
            Expr::Num(2.0),  // t
        ];
        let expr = Expr::Call {
            name: "physics::projectile_at".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(
                arr.len(),
                4,
                "projectile_at debería retornar 4 elementos [x, y, vx, vy]"
            );
        } else {
            panic!("projectile_at debería retornar Array, fue {:?}", result);
        }
    }

    #[test]
    fn test_physics_nbody_2() {
        // physics::nbody_2(m1, m2, x1, y1, x2, y2, G)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![
            Expr::Num(5.97e24),   // m1 (Tierra)
            Expr::Num(7.35e22),   // m2 (Luna)
            Expr::Num(0.0),       // x1
            Expr::Num(0.0),       // y1
            Expr::Num(3.844e8),   // x2
            Expr::Num(0.0),       // y2
            Expr::Num(6.674e-11), // G
        ];
        let expr = Expr::Call {
            name: "physics::nbody_2".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(
                arr.len(),
                5,
                "nbody_2 debería retornar 5 elementos [fx1, fy1, fx2, fy2, dist]"
            );
            // La fuerza debería ser positiva (atracción)
            if let Valor::Num(fx) = arr[0] {
                assert!(fx > 0.0, "La fuerza en x debería ser positiva");
            }
        } else {
            panic!("nbody_2 debería retornar Array, fue {:?}", result);
        }
    }

    #[test]
    fn test_physics_wave_1d() {
        // physics::wave_1d(x, t, lambda, freq)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![
            Expr::Num(1.0), // x
            Expr::Num(0.5), // t
            Expr::Num(2.0), // lambda
            Expr::Num(1.0), // freq
        ];
        let expr = Expr::Call {
            name: "physics::wave_1d".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Num(amp) = result {
            // La amplitud debería estar entre -1 y 1
            assert!(
                amp >= -1.0 && amp <= 1.0,
                "La amplitud debería estar entre -1 y 1, fue {}",
                amp
            );
        } else {
            panic!("wave_1d debería retornar Num, fue {:?}", result);
        }
    }

    #[test]
    fn test_physics_pendulum() {
        // physics::pendulum(length, angle0, t)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![
            Expr::Num(1.0),  // longitud
            Expr::Num(10.0), // ang0
            Expr::Num(1.0),  // t
        ];
        let expr = Expr::Call {
            name: "physics::pendulum".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(
                arr.len(),
                3,
                "pendulum debería retornar 3 elementos [angle, angular_vel, period]"
            );
        } else {
            panic!("pendulum debería retornar Array, fue {:?}", result);
        }
    }

    // ========================================================================
    // TESTS V0.7.1.3 - CIENCIA DE DATOS
    // ========================================================================

    #[test]
    fn test_stats_mean() {
        // stats::mean([1, 2, 3, 4, 5])
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![Expr::Array(vec![
            Expr::Num(1.0),
            Expr::Num(2.0),
            Expr::Num(3.0),
            Expr::Num(4.0),
            Expr::Num(5.0),
        ])];
        let expr = Expr::Call {
            name: "stats::mean".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Num(mean) = result {
            assert!(
                (mean - 3.0).abs() < 0.001,
                "mean([1,2,3,4,5]) debería ser 3.0, fue {}",
                mean
            );
        } else {
            panic!("mean debería retornar Num, fue {:?}", result);
        }
    }

    #[test]
    fn test_stats_median_odd() {
        // stats::median([1, 2, 3, 4, 5])
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![Expr::Array(vec![
            Expr::Num(1.0),
            Expr::Num(2.0),
            Expr::Num(3.0),
            Expr::Num(4.0),
            Expr::Num(5.0),
        ])];
        let expr = Expr::Call {
            name: "stats::median".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Num(median) = result {
            assert!(
                (median - 3.0).abs() < 0.001,
                "median([1,2,3,4,5]) debería ser 3.0, fue {}",
                median
            );
        } else {
            panic!("median debería retornar Num, fue {:?}", result);
        }
    }

    #[test]
    fn test_stats_median_even() {
        // stats::median([1, 2, 3, 4])
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![Expr::Array(vec![
            Expr::Num(1.0),
            Expr::Num(2.0),
            Expr::Num(3.0),
            Expr::Num(4.0),
        ])];
        let expr = Expr::Call {
            name: "stats::median".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Num(median) = result {
            assert!(
                (median - 2.5).abs() < 0.001,
                "median([1,2,3,4]) debería ser 2.5, fue {}",
                median
            );
        } else {
            panic!("median debería retornar Num, fue {:?}", result);
        }
    }

    #[test]
    fn test_stats_min_max() {
        // stats::min y stats::max
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let arr = Expr::Array(vec![
            Expr::Num(5.0),
            Expr::Num(2.0),
            Expr::Num(8.0),
            Expr::Num(1.0),
            Expr::Num(9.0),
        ]);

        // Test min
        let expr_min = Expr::Call {
            name: "stats::min".to_string(),
            args: vec![arr.clone()],
        };
        let result_min = evaluar_expr(&expr_min, &mut executor, &mut funcs);
        if let Valor::Num(min) = result_min {
            assert!(
                (min - 1.0).abs() < 0.001,
                "min debería ser 1.0, fue {}",
                min
            );
        } else {
            panic!("min debería retornar Num, fue {:?}", result_min);
        }

        // Test max
        let expr_max = Expr::Call {
            name: "stats::max".to_string(),
            args: vec![arr],
        };
        let result_max = evaluar_expr(&expr_max, &mut executor, &mut funcs);
        if let Valor::Num(max) = result_max {
            assert!(
                (max - 9.0).abs() < 0.001,
                "max debería ser 9.0, fue {}",
                max
            );
        } else {
            panic!("max debería retornar Num, fue {:?}", result_max);
        }
    }

    #[test]
    fn test_csv_parse() {
        // csv::parse("a,b\n1,2")
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![Expr::Texto("nombre,edad\nJuan,25\nMaria,30".to_string())];
        let expr = Expr::Call {
            name: "csv::parse".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(rows) = result {
            assert_eq!(rows.len(), 2, "Debería parsear 2 filas (sin headers)");
            if let Valor::Array(first_row) = &rows[0] {
                assert_eq!(first_row.len(), 2, "Cada fila debería tener 2 columnas");
            }
        } else {
            panic!("csv::parse debería retornar Array, fue {:?}", result);
        }
    }

    #[test]
    fn test_plot_ascii_chart() {
        // plot::ascii_chart([1, 2, 3, 4, 5], 20)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![
            Expr::Array(vec![
                Expr::Num(1.0),
                Expr::Num(2.0),
                Expr::Num(3.0),
                Expr::Num(4.0),
                Expr::Num(5.0),
            ]),
            Expr::Num(20.0),
        ];
        let expr = Expr::Call {
            name: "plot::ascii_chart".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Texto(chart) = result {
            assert!(chart.contains('*'), "El gráfico debería contener '*'");
            assert!(chart.len() > 10, "El gráfico debería tener contenido");
        } else {
            panic!("ascii_chart debería retornar Texto, fue {:?}", result);
        }
    }

    // ========================================================================
    // TESTS V0.7.1.4 - BEZIER
    // ========================================================================

    #[test]
    fn test_bezier_linear() {
        // bezier::linear(p0_x, p0_y, p1_x, p1_y, t)
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![
            Expr::Num(0.0),   // p0_x
            Expr::Num(0.0),   // p0_y
            Expr::Num(100.0), // p1_x
            Expr::Num(100.0), // p1_y
            Expr::Num(0.5),   // t
        ];
        let expr = Expr::Call {
            name: "bezier::linear".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 2, "linear debería retornar [x, y]");
            if let (Valor::Num(x), Valor::Num(y)) = (&arr[0], &arr[1]) {
                assert!((x - 50.0).abs() < 0.01, "x debería ser 50, fue {}", x);
                assert!((y - 50.0).abs() < 0.01, "y debería ser 50, fue {}", y);
            }
        } else {
            panic!("linear debería retornar Array, fue {:?}", result);
        }
    }

    #[test]
    fn test_bezier_quadratic() {
        // bezier::quadratic con 3 puntos de control
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![
            Expr::Num(0.0),   // p0_x
            Expr::Num(0.0),   // p0_y
            Expr::Num(50.0),  // p1_x (control)
            Expr::Num(100.0), // p1_y (control)
            Expr::Num(100.0), // p2_x
            Expr::Num(0.0),   // p2_y
            Expr::Num(0.5),   // t
        ];
        let expr = Expr::Call {
            name: "bezier::quadratic".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 2, "quadratic debería retornar [x, y]");
        } else {
            panic!("quadratic debería retornar Array, fue {:?}", result);
        }
    }

    #[test]
    fn test_bezier_cubic() {
        // bezier::cubic con 4 puntos de control
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![
            Expr::Num(0.0),   // p0_x
            Expr::Num(0.0),   // p0_y
            Expr::Num(30.0),  // p1_x
            Expr::Num(100.0), // p1_y
            Expr::Num(70.0),  // p2_x
            Expr::Num(100.0), // p2_y
            Expr::Num(100.0), // p3_x
            Expr::Num(0.0),   // p3_y
            Expr::Num(0.5),   // t
        ];
        let expr = Expr::Call {
            name: "bezier::cubic".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 2, "cubic debería retornar [x, y]");
        } else {
            panic!("cubic debería retornar Array, fue {:?}", result);
        }
    }

    #[test]
    fn test_bezier_cubic_derivative() {
        // bezier::cubic_derivative - tangente en t=0.5
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        let args = vec![
            Expr::Num(0.0),   // p0_x
            Expr::Num(0.0),   // p0_y
            Expr::Num(30.0),  // p1_x
            Expr::Num(100.0), // p1_y
            Expr::Num(70.0),  // p2_x
            Expr::Num(100.0), // p2_y
            Expr::Num(100.0), // p3_x
            Expr::Num(0.0),   // p3_y
            Expr::Num(0.5),   // t
        ];
        let expr = Expr::Call {
            name: "bezier::cubic_derivative".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(arr) = result {
            assert_eq!(arr.len(), 2, "cubic_derivative debería retornar [dx, dy]");
        } else {
            panic!("cubic_derivative debería retornar Array, fue {:?}", result);
        }
    }

    #[test]
    fn test_bezier_generate_points() {
        // bezier::generate_points - generar 5 puntos de una curva
        let mut executor = Executor::nuevo();
        let mut funcs: HashMap<String, (Vec<String>, Vec<Stmt>)> = HashMap::new();

        // Puntos de control: [[0,0], [30,100], [70,100], [100,0]]
        let control_points = Expr::Array(vec![
            Expr::Array(vec![Expr::Num(0.0), Expr::Num(0.0)]),
            Expr::Array(vec![Expr::Num(30.0), Expr::Num(100.0)]),
            Expr::Array(vec![Expr::Num(70.0), Expr::Num(100.0)]),
            Expr::Array(vec![Expr::Num(100.0), Expr::Num(0.0)]),
        ]);

        let args = vec![control_points, Expr::Num(5.0)];
        let expr = Expr::Call {
            name: "bezier::generate_points".to_string(),
            args,
        };

        let result = evaluar_expr(&expr, &mut executor, &mut funcs);

        if let Valor::Array(points) = result {
            assert_eq!(points.len(), 5, "Debería generar 5 puntos");
            // Primer punto debería ser (0, 0)
            if let Valor::Array(first) = &points[0] {
                if let (Valor::Num(x), Valor::Num(y)) = (&first[0], &first[1]) {
                    assert!((x - 0.0).abs() < 0.01, "Primer punto x debería ser 0");
                    assert!((y - 0.0).abs() < 0.01, "Primer punto y debería ser 0");
                }
            }
            // Último punto debería ser (100, 0)
            if let Valor::Array(last) = &points[4] {
                if let (Valor::Num(x), Valor::Num(y)) = (&last[0], &last[1]) {
                    assert!((x - 100.0).abs() < 0.01, "Último punto x debería ser 100");
                    assert!((y - 0.0).abs() < 0.01, "Último punto y debería ser 0");
                }
            }
        } else {
            panic!("generate_points debería retornar Array, fue {:?}", result);
        }
    }
}
