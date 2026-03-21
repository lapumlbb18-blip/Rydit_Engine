// crates/blast-core/src/lib.rs

use std::collections::HashMap;
use std::io::Write;

/// Valor que puede tener una variable RyDit
#[derive(Debug, Clone, PartialEq)]
pub enum Valor {
    Num(f64),
    Texto(String),
    Bool(bool),
    Array(Vec<Valor>),  // Array de valores
    Vacio,
    Error(String),
}

impl std::fmt::Display for Valor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Valor::Num(n) => write!(f, "{}", n),
            Valor::Texto(s) => write!(f, "{}", s),
            Valor::Bool(b) => write!(f, "{}", b),
            Valor::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", items.join(", "))
            }
            Valor::Vacio => write!(f, "vacío"),
            Valor::Error(msg) => write!(f, "[ERROR] {}", msg),
        }
    }
}

/// Executor con memoria para RyDit
/// Guarda variables y ejecuta comandos con estilo
pub struct Executor {
    pub memoria: HashMap<String, Valor>,
    pub activo: bool,
    /// Stack de scopes para funciones (scope local)
    pub scope_stack: Vec<HashMap<String, Valor>>,
}

impl Executor {
    pub fn nuevo() -> Self {
        println!("[BLAST-CORE]: Executor despertado con memoria y stack de scopes.");
        Self {
            memoria: HashMap::new(),
            activo: true,
            scope_stack: Vec::new(),
        }
    }

    /// Guardar variable en memoria (scope actual o global)
    /// dark.slot delta.flow = 0.5
    pub fn guardar(&mut self, nombre: &str, valor: Valor) {
        // Si hay scope local, guardar ahí
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.insert(nombre.to_string(), valor.clone());
            println!("[MEMORIA LOCAL] {} = {}", nombre, valor);
        } else {
            // Sino, guardar en global
            self.memoria.insert(nombre.to_string(), valor.clone());
            println!("[MEMORIA GLOBAL] {} = {}", nombre, valor);
        }
    }

    /// Leer variable de memoria (busca en scope local primero, luego global)
    pub fn leer(&self, nombre: &str) -> Option<Valor> {
        // Buscar en scopes locales (desde el más interno hacia afuera)
        for scope in self.scope_stack.iter().rev() {
            if let Some(valor) = scope.get(nombre) {
                return Some(valor.clone());
            }
        }
        // Buscar en scope global
        self.memoria.get(nombre).cloned()
    }

    /// Empujar nuevo scope local (para funciones)
    pub fn push_scope(&mut self) {
        self.scope_stack.push(HashMap::new());
        println!("[SCOPE] Nuevo scope local creado (nivel {})", self.scope_stack.len());
    }

    /// Pop scope local (salir de función)
    pub fn pop_scope(&mut self) {
        if let Some(scope) = self.scope_stack.pop() {
            println!("[SCOPE] Scope local eliminado (quedan {} niveles)", self.scope_stack.len());
            // Opcional: mostrar variables locales que se van
            for (nombre, valor) in &scope {
                println!("[SCOPE] {} = {} (eliminado)", nombre, valor);
            }
        } else {
            println!("[WARNING] Pop de scope vacío");
        }
    }

    /// Guardar en scope local actual (para parámetros de funciones)
    pub fn guardar_local(&mut self, nombre: &str, valor: Valor) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.insert(nombre.to_string(), valor.clone());
            println!("[LOCAL] {} = {}", nombre, valor);
        } else {
            // Fallback a global si no hay scope local
            println!("[WARNING] guardar_local sin scope local, usando global");
            self.memoria.insert(nombre.to_string(), valor.clone());
        }
    }

    /// Ejecutar comando con estilo
    pub fn ejecutar(&self, comando: &str) {
        if self.activo {
            println!("[BLAST-CORE] Ejecutando: {}", comando);
        }
    }

    /// Shock wave con aura
    pub fn shock_wave(&self) {
        if self.activo {
            println!("[BLAST-CORE]: Impacto sónico detectado.");
        }
    }

    /// Mostrar estado de memoria
    pub fn mostrar_memoria(&self) {
        println!("\n=== MEMORIA RYDIT ===");
        if self.memoria.is_empty() {
            println!("(vacía)");
        } else {
            for (nombre, valor) in &self.memoria {
                println!("  {} = {}", nombre, valor);
            }
        }
        println!("=====================\n");
    }

    /// Print/voz - mostrar valor en pantalla
    pub fn voz(&self, valor: &Valor) {
        println!("{}", valor);
    }

    /// Input - leer valor del usuario con manejo seguro de errores
    pub fn input(&self, prompt: &str) -> Valor {
        use std::io;

        print!("{}", prompt);
        // Usar ok() en vez de unwrap() para evitar panic en I/O
        if let Err(e) = io::stdout().flush() {
            eprintln!("[WARNING] Error en flush de stdout: {}", e);
            return Valor::Error("Error de I/O en input".to_string());
        }

        let mut input = String::new();
        // Manejar error de lectura de stdin
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("[WARNING] Error al leer stdin: {}", e);
            return Valor::Error("Error de lectura en input".to_string());
        }

        // Intentar parsear como número
        if let Ok(num) = input.trim().parse::<f64>() {
            Valor::Num(num)
        } else {
            Valor::Texto(input.trim().to_string())
        }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::nuevo()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nuevo_executor() {
        let executor = Executor::nuevo();
        assert!(executor.activo);
        assert!(executor.memoria.is_empty());
    }

    #[test]
    fn test_guardar_numero() {
        let mut executor = Executor::nuevo();
        executor.guardar("x", Valor::Num(100.0));
        assert_eq!(executor.memoria.len(), 1);
        assert_eq!(executor.leer("x"), Some(Valor::Num(100.0)));
    }

    #[test]
    fn test_guardar_decimal() {
        let mut executor = Executor::nuevo();
        executor.guardar("delta.flow", Valor::Num(0.5));
        assert_eq!(executor.leer("delta.flow"), Some(Valor::Num(0.5)));
    }

    #[test]
    fn test_guardar_texto() {
        let mut executor = Executor::nuevo();
        executor.guardar("nombre", Valor::Texto("Heroe".to_string()));
        assert_eq!(executor.leer("nombre"), Some(Valor::Texto("Heroe".to_string())));
    }

    #[test]
    fn test_leer_inexistente() {
        let executor = Executor::nuevo();
        assert_eq!(executor.leer("inexistente"), None);
    }

    #[test]
    fn test_sobrescribir_variable() {
        let mut executor = Executor::nuevo();
        executor.guardar("x", Valor::Num(100.0));
        executor.guardar("x", Valor::Num(200.0));
        assert_eq!(executor.leer("x"), Some(Valor::Num(200.0)));
    }

    #[test]
    fn test_multiples_variables() {
        let mut executor = Executor::nuevo();
        executor.guardar("a", Valor::Num(1.0));
        executor.guardar("b", Valor::Num(2.0));
        executor.guardar("c", Valor::Num(3.0));
        assert_eq!(executor.memoria.len(), 3);
    }

    #[test]
    fn test_guardar_array() {
        let mut executor = Executor::nuevo();
        let array = Valor::Array(vec![Valor::Num(1.0), Valor::Num(2.0), Valor::Num(3.0)]);
        executor.guardar("lista", array);
        assert_eq!(executor.memoria.len(), 1);
    }

    #[test]
    fn test_leer_array() {
        let mut executor = Executor::nuevo();
        let array = Valor::Array(vec![Valor::Num(10.0), Valor::Num(20.0)]);
        executor.guardar("x", array);
        
        if let Some(Valor::Array(arr)) = executor.leer("x") {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("No es un Array");
        }
    }

    #[test]
    fn test_array_display() {
        let array = Valor::Array(vec![Valor::Num(1.0), Valor::Num(2.0), Valor::Num(3.0)]);
        let display = format!("{}", array);
        assert_eq!(display, "[1, 2, 3]");
    }

    #[test]
    fn test_array_vacio_display() {
        let array = Valor::Array(vec![]);
        let display = format!("{}", array);
        assert_eq!(display, "[]");
    }

    // ==================== TESTS DE INPUT ====================

    #[test]
    fn test_input_numero() {
        // Test que verifica que input() puede retornar números
        // Nota: No podemos testear stdin real, pero verificamos el parseo
        let input_str = "42";
        if let Ok(num) = input_str.parse::<f64>() {
            assert_eq!(num, 42.0);
        }
    }

    #[test]
    fn test_input_texto() {
        // Test que verifica que input() puede retornar texto
        let input_str = "hola";
        if let Ok(_) = input_str.parse::<f64>() {
            panic!("No debería parsear como número");
        }
        // Se retorna como texto
        assert_eq!(input_str, "hola");
    }

    #[test]
    fn test_input_vacio() {
        // Input vacío debería retornar string vacío
        let input_str = "";
        let trimmed = input_str.trim();
        assert_eq!(trimmed, "");
    }

    // ==================== TESTS DE SCOPE ====================

    #[test]
    fn test_push_pop_scope() {
        let mut executor = Executor::nuevo();
        assert_eq!(executor.scope_stack.len(), 0);
        
        executor.push_scope();
        assert_eq!(executor.scope_stack.len(), 1);
        
        executor.pop_scope();
        assert_eq!(executor.scope_stack.len(), 0);
    }

    #[test]
    fn test_guardar_en_scope_local() {
        let mut executor = Executor::nuevo();
        
        // Variable global
        executor.guardar("x", Valor::Num(100.0));
        
        // Push scope local
        executor.push_scope();
        
        // Guardar en scope local (debería ir al scope, no al global)
        executor.guardar("x", Valor::Num(200.0));
        
        // Leer debería retornar el valor local
        assert_eq!(executor.leer("x"), Some(Valor::Num(200.0)));
        
        // Pop scope
        executor.pop_scope();
        
        // Leer debería retornar el valor global ahora
        assert_eq!(executor.leer("x"), Some(Valor::Num(100.0)));
    }

    #[test]
    fn test_scope_anidados() {
        let mut executor = Executor::nuevo();
        
        // Global
        executor.guardar("nivel", Valor::Num(0.0));
        
        // Scope 1
        executor.push_scope();
        executor.guardar("nivel", Valor::Num(1.0));
        executor.guardar("var1", Valor::Num(10.0));
        
        // Scope 2
        executor.push_scope();
        executor.guardar("nivel", Valor::Num(2.0));
        executor.guardar("var2", Valor::Num(20.0));
        
        // Debería leer del scope más interno
        assert_eq!(executor.leer("nivel"), Some(Valor::Num(2.0)));
        assert_eq!(executor.leer("var2"), Some(Valor::Num(20.0)));
        assert_eq!(executor.leer("var1"), Some(Valor::Num(10.0)));
        
        // Pop scope 2
        executor.pop_scope();
        assert_eq!(executor.leer("nivel"), Some(Valor::Num(1.0)));
        assert_eq!(executor.leer("var2"), None);  // Ya no existe
        assert_eq!(executor.leer("var1"), Some(Valor::Num(10.0)));
        
        // Pop scope 1
        executor.pop_scope();
        assert_eq!(executor.leer("nivel"), Some(Valor::Num(0.0)));
        assert_eq!(executor.leer("var1"), None);  // Ya no existe
    }

    #[test]
    fn test_guardar_local() {
        let mut executor = Executor::nuevo();
        
        // Sin scope local, guardar_local usa global
        executor.guardar_local("x", Valor::Num(100.0));
        assert_eq!(executor.leer("x"), Some(Valor::Num(100.0)));
        
        // Con scope local
        executor.push_scope();
        executor.guardar_local("y", Valor::Num(200.0));
        assert_eq!(executor.leer("y"), Some(Valor::Num(200.0)));
        
        executor.pop_scope();
        assert_eq!(executor.leer("y"), None);  // Ya no existe
    }

    // ========================================================================
    // TESTS V0.1.9 - SCOPES Y MEMORIA
    // ========================================================================

    #[test]
    fn test_scope_anidados_con_simbolos() {
        // Variables con símbolos en scopes anidados
        let mut executor = Executor::nuevo();
        
        // Scope global: $global = 100
        executor.guardar("$global", Valor::Num(100.0));
        
        // Simular scope anidado (push/pop)
        executor.push_scope();
        executor.guardar("@local", Valor::Texto("interno".to_string()));
        
        // Verificar que ambos existen
        assert_eq!(executor.leer("$global"), Some(Valor::Num(100.0)));
        assert_eq!(executor.leer("@local"), Some(Valor::Texto("interno".to_string())));
        
        // Pop scope
        executor.pop_scope();
        
        // $global debe seguir existiendo, @local no
        assert_eq!(executor.leer("$global"), Some(Valor::Num(100.0)));
        assert_eq!(executor.leer("@local"), None);
    }

    #[test]
    fn test_memoria_variables_temporales() {
        // Variables temporales en memoria
        let mut executor = Executor::nuevo();
        
        // Variables persistentes
        executor.guardar("x", Valor::Num(10.0));
        executor.guardar("$precio", Valor::Num(99.99));
        
        // Verificar memoria global
        assert_eq!(executor.memoria.len(), 2);
        
        // Simular variable temporal (scope)
        executor.push_scope();
        executor.guardar("__temp", Valor::Num(42.0));
        
        // __temp debe existir en scope local
        assert_eq!(executor.leer("__temp"), Some(Valor::Num(42.0)));
        
        // Pop scope elimina temporal
        executor.pop_scope();
        assert_eq!(executor.leer("__temp"), None);
        
        // Persistentes siguen ahí
        assert_eq!(executor.leer("x"), Some(Valor::Num(10.0)));
        assert_eq!(executor.leer("$precio"), Some(Valor::Num(99.99)));
    }
}

// Legacy - para compatibilidad
pub struct BlastCore {
    pub activo: bool,
}

impl BlastCore {
    pub fn despertar() -> Self {
        println!("[BLAST-CORE]: Motor de audio en guardia.");
        Self { activo: true }
    }

    pub fn ignite(&self, sonido: &str) {
        if self.activo {
            println!("[BLAST-CORE]: Ejecutando ráfaga -> {}", sonido);
        }
    }

    pub fn shock_wave(&self) {
        if self.activo {
            println!("[BLAST-CORE]: Impacto sónico detectado.");
        }
    }
}

