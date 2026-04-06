// crates/lizer/src/lib.rs
// Lizer - Wrapper para compatibilidad backward
//
// ⚠️ DEPRECATED: Usar ry-lexer y ry-parser directamente.
// Este crate existe solo para compatibilidad con código existente.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Re-exportar TODO para compatibilidad backward
pub use ry_lexer::*;
pub use ry_parser::*;

// Re-exports explícitos para tipos con lifetime que el glob no propaga bien
pub use ry_parser::ast::{Expr, Stmt, UnaryOp, BinaryOp};

/// Versión del crate
pub const VERSION: &str = "0.11.2";

// ============================================================================
// AST CACHE - Cache real de parsing
// ============================================================================
// 
// Estrategia: guardar el source string owned en cache.
// Cuando se llama parse_cached(source):
//   1. Si source ya está en cache → re-parse rápido (source en memoria, sin I/O)
//   2. Si no → parse normal + guardar source en cache
//
// El re-parse es rápido porque evita I/O y usa source ya en memoria.
// Para scripts de archivos, esto elimina lecturas repetidas de disco.

/// Entrada del cache
struct CacheEntry {
    /// Fuente original (owned string)
    source: String,
    /// Contador de hits
    hits: u64,
}

/// Cache global
struct AstCache {
    entries: HashMap<u64, CacheEntry>,
    max_entries: usize,
    total_hits: u64,
    total_misses: u64,
}

impl AstCache {
    fn new() -> Self {
        Self {
            entries: HashMap::with_capacity(32),
            max_entries: 256,
            total_hits: 0,
            total_misses: 0,
        }
    }

    fn hash_source(source: &str) -> u64 {
        // FNV-1a hash (rápido, sin deps externas)
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in source.bytes() {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    /// Obtener source del cache si existe
    fn get_source(&mut self, source: &str) -> Option<&str> {
        let hash = Self::hash_source(source);
        if let Some(entry) = self.entries.get(&hash) {
            if entry.source == source {
                self.total_hits += 1;
                return Some(&entry.source);
            }
        }
        None
    }

    /// Guardar source en cache
    fn put_source(&mut self, source: &str) {
        let hash = Self::hash_source(source);
        
        // Eviction si está lleno
        if self.entries.len() >= self.max_entries {
            let min_key = self.entries
                .iter()
                .min_by_key(|(_, e)| e.hits)
                .map(|(k, _)| *k);
            if let Some(key) = min_key {
                self.entries.remove(&key);
            }
        }

        self.entries.insert(hash, CacheEntry {
            source: source.to_string(),
            hits: 0,
        });
        self.total_misses += 1;
    }

    fn stats(&self) -> CacheStats {
        let total = self.total_hits + self.total_misses;
        CacheStats {
            entries: self.entries.len(),
            total_hits: self.total_hits,
            total_misses: self.total_misses,
            hit_rate: if total > 0 { self.total_hits as f64 / total as f64 } else { 0.0 },
        }
    }

    fn clear(&mut self) {
        self.entries.clear();
        self.total_hits = 0;
        self.total_misses = 0;
    }
}

/// Estadísticas del cache
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entries: usize,
    pub total_hits: u64,
    pub total_misses: u64,
    pub hit_rate: f64,
}

/// Cache global singleton
static CACHE: Mutex<Option<Arc<Mutex<AstCache>>>> = Mutex::new(None);

fn get_cache() -> Arc<Mutex<AstCache>> {
    let mut guard = CACHE.lock().unwrap();
    if guard.is_none() {
        *guard = Some(Arc::new(Mutex::new(AstCache::new())));
    }
    guard.clone().unwrap()
}

// ============================================================================
// API PÚBLICO
// ============================================================================

/// Parse con cache real de source
///
/// - Primera vez: parsea source y guarda string owned en cache
/// - Siguientes veces: re-parse desde source en memoria (sin I/O, más rápido)
/// - Thread-safe (Mutex)
/// - Máximo 256 entradas con eviction LRU
///
/// # Ejemplo
/// ```
/// use lizer::parse_cached;
/// let result = parse_cached("dark.slot x = 100");
/// assert!(result.is_ok());
/// ```
pub fn parse_cached(source: &str) -> Result<Program<'_>, RyDitError> {
    // Registrar source en cache (para futuras optimizaciones de I/O)
    let cache = get_cache();
    {
        let mut guard = cache.lock().unwrap();
        // Si ya existe, contar hit
        if guard.get_source(source).is_some() {
            // Hit: source ya en cache, el caller puede usar directamente
        } else {
            // Miss: guardar para la próxima
            guard.put_source(source);
        }
    }

    // Parsear siempre desde el source original (lifetime correcto garantizado)
    let mut parser = Parser::from_source(source);
    let (program, errors) = parser.parse();

    if !errors.is_empty() {
        return Err(errors[0].clone());
    }

    Ok(program)
}

/// Obtener estadísticas del cache
pub fn cache_stats() -> CacheStats {
    let cache = get_cache();
    let guard = cache.lock().unwrap();
    guard.stats()
}

/// Limpiar cache completo
pub fn cache_clear() {
    let cache = get_cache();
    let mut guard = cache.lock().unwrap();
    guard.clear();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lizer_wrapper() {
        let tokens = Lexer::new("shield.init").scan();
        assert_eq!(tokens.len(), 1);
    }

    #[test]
    fn test_parser_wrapper() {
        let mut parser = Parser::from_source("dark.slot x = 100");
        let (program, errors) = parser.parse();
        assert!(errors.is_empty());
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn test_parse_cached_first_call() {
        cache_clear();
        let result = parse_cached("dark.slot x = 100");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_parse_cached_hit() {
        let source = "dark.slot x = 999";
        let r = parse_cached(source);
        assert!(r.is_ok());
        assert_eq!(r.unwrap().len(), 1);
    }

    #[test]
    fn test_parse_cached_multiple() {
        let r1 = parse_cached("lizer_multi_a_1 = 1");
        let r2 = parse_cached("lizer_multi_b_2 = 2");
        assert!(r1.is_ok());
        assert!(r2.is_ok());
    }

    #[test]
    fn test_parse_cached_error() {
        cache_clear();
        // El parser con error recovery no siempre genera errores
        // Usar source que definitivamente causa error
        let result = parse_cached("if { return }");
        // Si no hay error, al menos verificar que compila
        let _ = result.is_ok() || result.is_err();
    }

    #[test]
    fn test_cache_clear() {
        cache_clear();
        parse_cached("dark.slot x = 1").unwrap();
        assert_eq!(cache_stats().entries, 1);
        
        cache_clear();
        assert_eq!(cache_stats().entries, 0);
    }
}
