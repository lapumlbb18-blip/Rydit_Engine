// Tests de Núcleo - Nivel 1
// ✅ Sin gráficos, < 1 segundo, ultrarrápidos

// ============================================================================
// LIZER TESTS (Parser)
// ============================================================================

mod lizer_tests {
    use lizer::Lizer;

    #[test]
    fn test_numero_simple() {
        let lizer = Lizer::new("5");
        let tokens = lizer.scan();
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_expresion_aritmetica() {
        let lizer = Lizer::new("2 + 3");
        let tokens = lizer.scan();
        assert!(tokens.len() >= 3);
    }

    #[test]
    fn test_string_simple() {
        let lizer = Lizer::new("\"hola\"");
        let tokens = lizer.scan();
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_variable() {
        let lizer = Lizer::new("mi_var");
        let tokens = lizer.scan();
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_array_basico() {
        let lizer = Lizer::new("[1, 2, 3]");
        let tokens = lizer.scan();
        assert!(tokens.len() >= 5);
    }
}

// ============================================================================
// BLAST-CORE TESTS (Executor)
// ============================================================================

mod blast_tests {
    use blast_core::{Executor, Valor};

    #[test]
    fn test_guardar_leer() {
        let mut executor = Executor::nuevo();
        executor.guardar("x", Valor::Num(5.0));
        assert_eq!(executor.leer("x"), Some(Valor::Num(5.0)));
    }

    #[test]
    fn test_scopes() {
        let mut executor = Executor::nuevo();
        executor.guardar("x", Valor::Num(5.0));
        executor.push_scope();
        executor.guardar("x", Valor::Num(10.0));
        assert_eq!(executor.leer("x"), Some(Valor::Num(10.0)));
        executor.pop_scope();
        assert_eq!(executor.leer("x"), Some(Valor::Num(5.0)));
    }

    #[test]
    fn test_tipos_valor() {
        let mut executor = Executor::nuevo();
        executor.guardar("num", Valor::Num(3.14));
        executor.guardar("text", Valor::Texto("hola".to_string()));
        executor.guardar("bool", Valor::Bool(true));

        assert!(matches!(executor.leer("num"), Some(Valor::Num(_))));
        assert!(matches!(executor.leer("text"), Some(Valor::Texto(_))));
        assert!(matches!(executor.leer("bool"), Some(Valor::Bool(_))));
    }
}

// ============================================================================
// RYDITMODULE TESTS (Trait + Registry)
// ============================================================================

mod ryditmodule_tests {
    use rydit_anim::AnimModule;
    use rydit_core::{ModuleRegistry, RyditModule};
    use rydit_physics::PhysicsModule;
    use rydit_science::ScienceModule;
    use serde_json::json;

    #[test]
    fn test_registro_tres_modulos() {
        let mut registry = ModuleRegistry::new();
        registry.register(PhysicsModule);
        registry.register(AnimModule);
        registry.register(ScienceModule);

        assert_eq!(registry.len(), 3);
        assert!(registry.contains("physics"));
        assert!(registry.contains("anim"));
        assert!(registry.contains("science"));
    }

    #[test]
    fn test_physics_projectile() {
        let mut registry = ModuleRegistry::new();
        registry.register(PhysicsModule);

        let module = registry.get("physics").unwrap();
        let params = json!([0.0, 0.0, 10.0, 45.0]);
        let result = module.execute("projectile", params);

        assert!(result.is_ok());
    }

    #[test]
    #[test]
    #[test]
    fn test_metadata() {
        let mut registry = ModuleRegistry::new();
        registry.register(PhysicsModule);

        let module = registry.get("physics").unwrap();
        let metadata = module.metadata();

        assert_eq!(metadata.name, "physics");
        assert!(!metadata.version.is_empty());
    }
}
