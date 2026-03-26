// crates/rydit-rs/src/module.rs
// RyditModule trait para módulos extensibles

use blast_core::{Executor, Valor};
use lizer::{Expr, Stmt};
use std::collections::HashMap;

/// Tipo de función registrable en un módulo
pub type ModuleFunction =
    fn(&[Expr], &mut Executor, &mut HashMap<String, (Vec<String>, Vec<Stmt>)>) -> Valor;

/// Registro de funciones de un módulo
pub struct ModuleRegistry {
    functions: HashMap<String, ModuleFunction>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }

    /// Registrar una función en el módulo
    pub fn register_fn(&mut self, name: &str, func: ModuleFunction) {
        self.functions.insert(name.to_string(), func);
    }

    /// Obtener una función por nombre
    pub fn get_fn(&self, name: &str) -> Option<&ModuleFunction> {
        self.functions.get(name)
    }

    /// Verificar si una función existe
    pub fn has_fn(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    /// Obtener todas las funciones registradas
    pub fn list_functions(&self) -> Vec<&String> {
        self.functions.keys().collect()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Contexto compartido entre módulos
pub struct ModuleContext {
    pub frame: u32,
    pub dt: f32,
    pub width: f32,
    pub height: f32,
}

impl ModuleContext {
    pub fn new() -> Self {
        Self {
            frame: 0,
            dt: 0.0,
            width: 800.0,
            height: 600.0,
        }
    }

    pub fn update(&mut self, frame: u32, dt: f32) {
        self.frame = frame;
        self.dt = dt;
    }
}

impl Default for ModuleContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait base para todos los módulos de RyDit
pub trait RyditModule {
    /// Nombre único del módulo (ej: "math", "physics", "anim")
    fn name(&self) -> &'static str;

    /// Versión semántica (ej: "0.1.0")
    fn version(&self) -> &'static str;

    /// Registrar funciones en el registry
    fn register(&self, registry: &mut ModuleRegistry);

    /// Hook opcional para update (física, animación)
    fn update(&self, _ctx: &mut ModuleContext) {}

    /// Hook opcional para render (gráficos 2D/3D)
    fn render(&self, _ctx: &mut ModuleContext) {}

    /// Hook opcional para inicialización
    fn init(&self) {}

    /// Hook opcional para cleanup
    fn shutdown(&self) {}
}

/// Macro para registrar módulos automáticamente
#[macro_export]
macro_rules! rydit_module {
    ($name:ident, version: $version:literal, {
        $(fn $fn_name:ident = $fn_path:path,)*
    }) => {
        pub struct $name;

        impl $crate::module::RyditModule for $name {
            fn name(&self) -> &'static str {
                stringify!($name)
            }

            fn version(&self) -> &'static str {
                $version
            }

            fn register(&self, registry: &mut $crate::module::ModuleRegistry) {
                $(
                    registry.register_fn(stringify!($fn_name), $fn_path);
                )*
            }
        }
    };
}

// ============================================================================
// MÓDULO PILOTO: MATH (para pruebas)
// ============================================================================

/// Función math::sqrt para registrar
pub fn math_sqrt(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("math::sqrt() requiere 1 argumento".to_string());
    }

    // Evaluar el argumento
    use crate::eval::evaluar_expr;
    let val = evaluar_expr(&args[0], executor, _funcs);

    if let Valor::Num(x) = val {
        if x >= 0.0 {
            Valor::Num(x.sqrt())
        } else {
            Valor::Error("math::sqrt() requiere número >= 0".to_string())
        }
    } else {
        Valor::Error("math::sqrt() requiere número".to_string())
    }
}

/// Función math::sin para registrar
pub fn math_sin(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("math::sin() requiere 1 argumento".to_string());
    }

    use crate::eval::evaluar_expr;
    let val = evaluar_expr(&args[0], executor, _funcs);

    if let Valor::Num(x) = val {
        Valor::Num(x.sin())
    } else {
        Valor::Error("math::sin() requiere número".to_string())
    }
}

/// Función math::cos para registrar
pub fn math_cos(
    args: &[Expr],
    executor: &mut Executor,
    _funcs: &mut HashMap<String, (Vec<String>, Vec<Stmt>)>,
) -> Valor {
    if args.len() != 1 {
        return Valor::Error("math::cos() requiere 1 argumento".to_string());
    }

    use crate::eval::evaluar_expr;
    let val = evaluar_expr(&args[0], executor, _funcs);

    if let Valor::Num(x) = val {
        Valor::Num(x.cos())
    } else {
        Valor::Error("math::cos() requiere número".to_string())
    }
}

// Registrar módulo Math usando la macro
rydit_module!(MathModule, version: "0.1.0", {
    fn sqrt = math_sqrt,
    fn sin = math_sin,
    fn cos = math_cos,
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_registry() {
        let mut registry = ModuleRegistry::new();
        registry.register_fn("test_fn", math_sqrt);

        assert!(registry.has_fn("test_fn"));
        assert!(!registry.has_fn("nonexistent"));
    }

    #[test]
    fn test_math_module() {
        let module = MathModule;
        assert_eq!(module.name(), "MathModule");
        assert_eq!(module.version(), "0.1.0");

        let mut registry = ModuleRegistry::new();
        module.register(&mut registry);

        assert!(registry.has_fn("sqrt"));
        assert!(registry.has_fn("sin"));
        assert!(registry.has_fn("cos"));
    }
}
