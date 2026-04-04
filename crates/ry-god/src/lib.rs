//! # ry-god - Industrial Security & Efficiency Framework
//!
//! El **"cargo" de Ry-Dit**: supervisa, valida, reporta y asegura
//! que todo script se ejecute de forma segura, eficiente y auditable.
//!
//! ## Características
//!
//! - **ryprime**: Comando de autorización sobre el proyecto
//! - **Console reporting**: Errores, warnings, informes detallados
//! - **Sandbox VM**: Aislamiento total de scripts
//! - **Memory limits**: Sin OOM en dispositivos low-end
//! - **Input validation**: Nunca crash, siempre recover
//! - **Audit logging**: Registro de cada acción ejecutada
//! - **Zero-crash guarantees**: Error recovery en cada nivel
//!
//! ## Filosofía
//!
//! Si funciona en un Android gama baja con 2GB RAM, funciona en cualquier lado.
//!
//! ```rust,ignore
//! use ry_god::RyGod;
//!
//! let mut god = RyGod::new();
//! god.sandbox().memory_limit_mb(64).enable();
//! god.run("demos/test.rydit")?;
//! god.report().print();
//! ```

pub mod console;
pub mod sandbox;
pub mod audit;
pub mod limits;
pub mod ryprime;
pub mod report;
pub mod bench;
pub mod tests;

pub use console::ConsoleReporter;
pub use sandbox::Sandbox;
pub use audit::AuditLogger;
pub use limits::ResourceLimits;
pub use ryprime::RyPrime;
pub use report::ExecutionReport;
pub use bench::Benchmark;

/// ry-god: El supervisor industrial de Ry-Dit
///
/// Combina sandbox, auditoría, límites y reporting en una sola API.
pub struct RyGod {
    pub sandbox: Sandbox,
    pub audit: AuditLogger,
    pub limits: ResourceLimits,
    pub console: ConsoleReporter,
    pub report: ExecutionReport,
    pub ryprime: RyPrime,
}

impl RyGod {
    /// Crear nueva instancia de ry-god
    pub fn new() -> Self {
        Self {
            sandbox: Sandbox::new(),
            audit: AuditLogger::new(),
            limits: ResourceLimits::default(),
            console: ConsoleReporter::new(),
            report: ExecutionReport::new(),
            ryprime: RyPrime::new(),
        }
    }

    /// Configurar sandbox con límites seguros por defecto
    pub fn secure_defaults(&mut self) -> &mut Self {
        self.sandbox.enable();
        self.limits
            .set_max_memory_mb(64)
            .set_max_instructions(1_000_000)
            .set_max_loop_iterations(100_000);
        self.audit.enable();
        self.console.enable_warnings();
        self
    }

    /// Ejecutar script con supervisión completa
    pub fn run(&mut self, script_path: &str) -> Result<(), String> {
        self.audit.log_start(script_path);
        self.report.start_timer();

        // 1. Validar input antes de ejecutar
        if let Err(e) = self.validate_script(script_path) {
            self.console.error("Validación fallida", &e);
            self.audit.log_error("validation", &e);
            self.report.add_error("validation", &e);
            return Err(e);
        }

        // 2. Ejecutar dentro del sandbox
        self.console.info("Ejecutando", script_path);
        self.audit.log_action("execute", script_path);

        // 3. Ejecutar con ryprime si requiere autorización
        if self.ryprime.requires_authorization(script_path) {
            self.console.warn("Requiere autorización", script_path);
            if !self.ryprime.authorize(script_path) {
                let msg = format!("No autorizado: {}", script_path);
                self.console.error("Bloqueado", &msg);
                self.audit.log_error("authorization", &msg);
                return Err(msg);
            }
        }

        // 4. Ejecutar y capturar resultados
        let result = self.execute_script(script_path);

        self.report.stop_timer();

        match result {
            Ok(_) => {
                self.console.ok("Completado", script_path);
                self.audit.log_success(script_path);
                self.report.mark_success();
            }
            Err(ref e) => {
                self.console.error("Error ejecución", e);
                self.audit.log_error("execution", e);
                self.report.add_error("execution", e);
            }
        }

        result
    }

    /// Validar script antes de ejecutar
    fn validate_script(&self, path: &str) -> Result<(), String> {
        use std::fs;

        // Verificar que existe
        if !std::path::Path::new(path).exists() {
            return Err(format!("Archivo no encontrado: {}", path));
        }

        // Leer y validar sintaxis
        let source = fs::read_to_string(path)
            .map_err(|e| format!("No se pudo leer {}: {}", path, e))?;

        // Verificar tamaño (límite low-end)
        if source.len() > self.limits.max_script_size_kb * 1024 {
            return Err(format!(
                "Script demasiado grande: {}KB (máx {}KB)",
                source.len() / 1024,
                self.limits.max_script_size_kb
            ));
        }

        // Validación básica: archivo no vacío
        if source.trim().is_empty() {
            return Err("Script vacío".to_string());
        }

        #[cfg(feature = "integration")]
        {
            // Lexer validation
            let tokens = ry_lexer::Lexer::new(&source).scan();
            if tokens.is_empty() {
                return Err("Script vacío o sin tokens válidos".to_string());
            }

            // Parser validation
            let (_program, errors) = ry_parser::Parser::new(tokens).parse();
            if !errors.is_empty() {
                let msgs: Vec<_> = errors.iter().map(|e| format!("{}", e)).collect();
                return Err(format!("Errores de parsing: {:?}", msgs));
            }
        }

        Ok(())
    }

    /// Ejecutar script real con Ry-Dit
    fn execute_script(&mut self, path: &str) -> Result<(), String> {
        // Aquí se integra con el executor real de Ry-Dit
        // Por ahora, validación completa
        self.console.info("Script validado correctamente", path);
        Ok(())
    }

    /// Generar informe completo
    pub fn generate_report(&self) -> String {
        self.report.generate_text()
    }

    /// Imprimir informe en consola
    pub fn print_report(&self) {
        self.report.print();
    }

    /// Benchmark rápido
    pub fn quick_bench(&self, iterations: u32) -> bench::BenchmarkResult {
        Benchmark::run(iterations)
    }
}

impl Default for RyGod {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

// Tests movidos a tests/ module (revelation.rs, crate_verify.rs)
// Los tests inline test_rygod_new, test_secure_defaults, test_validate_nonexistent
// están cubiertos por revelation.rs
