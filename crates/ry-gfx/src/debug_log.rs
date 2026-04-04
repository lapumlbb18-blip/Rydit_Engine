// crates/rydit-gfx/src/debug_log.rs
// Sistema de Debug Log para game loop

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

static DEBUG_LOG: Mutex<Option<DebugLogger>> = Mutex::new(None);

pub struct DebugLogger {
    file: std::fs::File,
    frame: u32,
}

impl DebugLogger {
    pub fn init(path: &str) -> Result<(), String> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|e| format!("No se pudo abrir log: {}", e))?;

        let mut logger = DEBUG_LOG.lock().unwrap();
        *logger = Some(DebugLogger { file, frame: 0 });

        if let Some(ref mut log) = *logger {
            writeln!(log.file, "=== RYDIT DEBUG LOG v0.8.5 ===").ok();
            writeln!(log.file, "Game Loop Analysis").ok();
            writeln!(log.file).ok();
        }

        Ok(())
    }

    pub fn log_frame(&mut self, msg: &str) {
        writeln!(self.file, "[FRAME {}] {}", self.frame, msg).ok();
        self.frame += 1;
    }

    pub fn log(&mut self, msg: &str) {
        writeln!(self.file, "{}", msg).ok();
    }

    pub fn flush(&mut self) {
        self.file.flush().ok();
    }
}

// Funciones públicas para usar desde el game loop
pub fn debug_init(path: &str) {
    if let Err(e) = DebugLogger::init(path) {
        eprintln!("[DEBUG] Error inicializando log: {}", e);
    } else {
        eprintln!("[DEBUG] Log iniciado en: {}", path);
    }
}

pub fn debug_log(msg: &str) {
    let mut logger = DEBUG_LOG.lock().unwrap();
    if let Some(ref mut log) = *logger {
        log.log(msg);
        log.flush();
    }
}

pub fn debug_log_frame(msg: &str) {
    let mut logger = DEBUG_LOG.lock().unwrap();
    if let Some(ref mut log) = *logger {
        log.log_frame(msg);
        log.flush();
    }
}

pub fn debug_error(context: &str, error: &str) {
    let msg = format!("🔴 ERROR en {}: {}", context, error);
    debug_log(&msg);
    eprintln!("{}", msg);
}

pub fn debug_warn(context: &str, warning: &str) {
    let msg = format!("⚠️  WARNING en {}: {}", context, warning);
    debug_log(&msg);
    eprintln!("{}", msg);
}

pub fn debug_info(msg: &str) {
    debug_log(&format!("ℹ️  INFO: {}", msg));
}
