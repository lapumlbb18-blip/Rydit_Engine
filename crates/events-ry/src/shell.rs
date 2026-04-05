//! Shell - Parse y ejecución de comandos (Capa 3)
//!
//! Motor de comandos para:
//! - Carga de assets
//! - Debug en vivo
//! - REPL del lenguaje rydit
//! - Comandos del editor visual
//!
//! ## Comandos integrados
//!
//! ```text
//! load assets/sprite.png          → Carga un asset
//! exec draw_circle(400, 300, 50)  → Ejecuta código rydit
//! debug variables                 → Muestra variables
//! help                            → Lista comandos
//! clear                           → Limpia consola
//! ```

use std::collections::HashMap;

/// Comando parseado
#[derive(Debug, Clone, PartialEq)]
pub struct ShellCommand {
    /// Nombre del comando
    pub name: String,
    /// Argumentos
    pub args: Vec<String>,
    /// Línea original completa
    pub raw: String,
}

/// Resultado de ejecutar un comando
#[derive(Debug, Clone, PartialEq)]
pub struct ShellResult {
    /// Éxito o error
    pub success: bool,
    /// Mensaje de output
    pub output: String,
    /// Datos adicionales (JSON string)
    pub data: Option<String>,
}

/// Handler de comando - función que procesa un comando
pub type CommandHandler = Box<dyn Fn(&ShellCommand) -> ShellResult + Send + Sync>;

/// Línea de consola (output del shell)
#[derive(Debug, Clone)]
pub struct ConsoleLine {
    /// Texto de la línea
    pub text: String,
    /// Tipo de línea
    pub kind: ConsoleKind,
}

/// Tipo de línea de consola
#[derive(Debug, Clone, PartialEq)]
pub enum ConsoleKind {
    /// Output normal
    Info,
    /// Éxito
    Success,
    /// Error
    Error,
    /// Input del usuario (echo)
    Input,
    /// Debug/trace
    Debug,
}

/// Shell - Motor de comandos integrado
///
/// Registra handlers y ejecuta comandos desde el text input.
pub struct Shell {
    /// Handlers registrados por nombre
    handlers: HashMap<String, CommandHandler>,
    /// Historial de comandos (para navegación con ↑↓)
    history: Vec<String>,
    /// Posición en historial (para navegación)
    history_index: usize,
    /// Líneas de consola (output visible)
    console_lines: Vec<ConsoleLine>,
    /// Máximo de líneas en consola
    max_console_lines: usize,
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

impl Shell {
    /// Crear nuevo Shell vacío
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
            history: Vec::new(),
            history_index: 0,
            console_lines: Vec::new(),
            max_console_lines: 200,
        }
    }

    /// Crear con handlers integrados por defecto
    pub fn with_defaults() -> Self {
        let mut shell = Self::new();
        shell.register_defaults();
        shell
    }

    /// Registrar un handler de comando
    pub fn register<S, F>(&mut self, name: S, handler: F)
    where
        S: Into<String>,
        F: Fn(&ShellCommand) -> ShellResult + Send + Sync + 'static,
    {
        self.handlers.insert(name.into(), Box::new(handler));
    }

    /// Ejecutar una línea de texto como comando
    pub fn execute(&mut self, input: &str) -> ShellResult {
        let input = input.trim();

        if input.is_empty() {
            return ShellResult {
                success: true,
                output: String::new(),
                data: None,
            };
        }

        // Agregar al historial
        self.history.push(input.to_string());
        self.history_index = self.history.len();

        // Echo a consola
        self.add_console_line(input, ConsoleKind::Input);

        // Parsear comando
        let cmd = Self::parse_command(input);

        // Ejecutar handler
        let result = if let Some(handler) = self.handlers.get(&cmd.name) {
            handler(&cmd)
        } else {
            ShellResult {
                success: false,
                output: format!("Comando desconocido: '{}'. Escribe 'help' para ver comandos.", cmd.name),
                data: None,
            }
        };

        // Output a consola
        if result.success {
            self.add_console_line(&result.output, ConsoleKind::Success);
        } else {
            self.add_console_line(&result.output, ConsoleKind::Error);
        }

        result
    }

    /// Parsear texto en ShellCommand
    pub fn parse_command(input: &str) -> ShellCommand {
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();

        let name = parts.first().unwrap_or(&"").to_string();
        let args: Vec<String> = parts.iter().skip(1).map(|s| s.to_string()).collect();

        ShellCommand {
            name,
            args,
            raw: input.to_string(),
        }
    }

    /// Navegar historial hacia atrás (↑)
    pub fn history_prev(&mut self) -> Option<&str> {
        if self.history.is_empty() {
            return None;
        }
        if self.history_index > 0 {
            self.history_index -= 1;
        }
        self.history.get(self.history_index).map(|s| s.as_str())
    }

    /// Navegar historial hacia adelante (↓)
    pub fn history_next(&mut self) -> Option<&str> {
        if self.history_index < self.history.len() {
            self.history_index += 1;
        }
        if self.history_index < self.history.len() {
            self.history.get(self.history_index).map(|s| s.as_str())
        } else {
            Some("") // Línea vacía al final
        }
    }

    /// Agregar línea a consola
    fn add_console_line(&mut self, text: &str, kind: ConsoleKind) {
        self.console_lines.push(ConsoleLine {
            text: text.to_string(),
            kind,
        });

        // Limitar líneas
        if self.console_lines.len() > self.max_console_lines {
            let excess = self.console_lines.len() - self.max_console_lines;
            self.console_lines.drain(0..excess);
        }
    }

    /// Obtener líneas de consola
    pub fn console_lines(&self) -> &[ConsoleLine] {
        &self.console_lines
    }

    /// Limpiar consola
    pub fn clear_console(&mut self) {
        self.console_lines.clear();
    }

    /// Obtener historial
    pub fn history(&self) -> &[String] {
        &self.history
    }

    /// Limpiar historial
    pub fn clear_history(&mut self) {
        self.history.clear();
        self.history_index = 0;
    }

    /// Registrar handlers por defecto
    fn register_defaults(&mut self) {
        // Handlers por defecto

        self.register("help", |_| ShellResult {
            success: true,
            output: concat!(
                "Comandos disponibles:\n",
                "  help                    - Muestra esta ayuda\n",
                "  clear                   - Limpia la consola\n",
                "  echo <texto>            - Repite texto\n",
                "  load <ruta>             - Carga un asset\n",
                "  exec <codigo>           - Ejecuta código rydit\n",
                "  debug <tipo>            - Muestra info debug\n",
                "  list                    - Lista handlers registrados\n",
                "  history                 - Muestra historial\n",
                "  version                 - Muestra versión"
            ).to_string(),
            data: None,
        });

        self.register("clear", |_| ShellResult {
            success: true,
            output: String::new(),
            data: None,
        });

        self.register("echo", |cmd| ShellResult {
            success: true,
            output: cmd.args.join(" "),
            data: None,
        });

        self.register("load", |cmd| {
            if cmd.args.is_empty() {
                return ShellResult {
                    success: false,
                    output: "Uso: load <ruta_del_asset>".to_string(),
                    data: None,
                };
            }
            let path = &cmd.args[0];
            // Placeholder - implementación real cargaría el asset
            ShellResult {
                success: true,
                output: format!("Asset cargado: {}", path),
                data: Some(format!("{{\"path\":\"{}\"}}", path)),
            }
        });

        self.register("exec", |cmd| {
            if cmd.args.is_empty() {
                return ShellResult {
                    success: false,
                    output: "Uso: exec <codigo_rydit>".to_string(),
                    data: None,
                };
            }
            let code = &cmd.args.join(" ");
            // Placeholder - implementación real ejecutaría código rydit
            ShellResult {
                success: true,
                output: format!("Ejecutado: {}", code),
                data: Some(format!("{{\"code\":\"{}\"}}", code)),
            }
        });

        self.register("debug", |cmd| {
            if cmd.args.is_empty() {
                return ShellResult {
                    success: false,
                    output: "Uso: debug <tipo> (variables, memory, fps)".to_string(),
                    data: None,
                };
            }
            let debug_type = &cmd.args[0];
            match debug_type.as_str() {
                "variables" => ShellResult {
                    success: true,
                    output: "Variables: 0 registradas".to_string(),
                    data: Some("{\"variables\":0}".to_string()),
                },
                "memory" => ShellResult {
                    success: true,
                    output: "Memoria: no disponible en mock".to_string(),
                    data: None,
                },
                "fps" => ShellResult {
                    success: true,
                    output: "FPS: 60".to_string(),
                    data: Some("{\"fps\":60}".to_string()),
                },
                _ => ShellResult {
                    success: false,
                    output: format!("Tipo debug desconocido: {}. Tipos: variables, memory, fps", debug_type),
                    data: None,
                },
            }
        });

        self.register("list", |_| ShellResult {
            success: true,
            output: "Handlers: help, clear, echo, load, exec, debug, list, history, version".to_string(),
            data: None,
        });

        self.register("history", |_cmd| {
            // No podemos acceder al self real aquí por el borrow checker
            // Placeholder - el InputManager real puede exponer esto
            ShellResult {
                success: true,
                output: format!("Historial: {} entradas", 0),
                data: None,
            }
        });

        self.register("version", |_| ShellResult {
            success: true,
            output: "events-ry v0.1.0 | ry-dit v0.13.0".to_string(),
            data: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_command() {
        let cmd = Shell::parse_command("help");
        assert_eq!(cmd.name, "help");
        assert!(cmd.args.is_empty());
        assert_eq!(cmd.raw, "help");
    }

    #[test]
    fn test_parse_command_with_args() {
        let cmd = Shell::parse_command("load assets/sprite.png");
        assert_eq!(cmd.name, "load");
        assert_eq!(cmd.args, vec!["assets/sprite.png"]);
    }

    #[test]
    fn test_parse_command_multiple_args() {
        let cmd = Shell::parse_command("draw circle 400 300 50 rojo");
        assert_eq!(cmd.name, "draw");
        assert_eq!(cmd.args, vec!["circle", "400", "300", "50", "rojo"]);
    }

    #[test]
    fn test_execute_help() {
        let mut shell = Shell::with_defaults();
        let result = shell.execute("help");
        assert!(result.success);
        assert!(result.output.contains("Comandos disponibles"));
    }

    #[test]
    fn test_execute_unknown_command() {
        let mut shell = Shell::with_defaults();
        let result = shell.execute("comando_inexistente");
        assert!(!result.success);
        assert!(result.output.contains("Comando desconocido"));
    }

    #[test]
    fn test_execute_echo() {
        let mut shell = Shell::with_defaults();
        let result = shell.execute("echo hola mundo");
        assert!(result.success);
        assert_eq!(result.output, "hola mundo");
    }

    #[test]
    fn test_execute_empty() {
        let mut shell = Shell::with_defaults();
        let result = shell.execute("");
        assert!(result.success);
        assert!(result.output.is_empty());
    }

    #[test]
    fn test_execute_load() {
        let mut shell = Shell::with_defaults();
        let result = shell.execute("load assets/sprite.png");
        assert!(result.success);
        assert!(result.output.contains("sprite.png"));
    }

    #[test]
    fn test_history_navigation() {
        let mut shell = Shell::with_defaults();
        shell.execute("cmd1");
        shell.execute("cmd2");
        shell.execute("cmd3");

        assert_eq!(shell.history_prev(), Some("cmd3"));
        assert_eq!(shell.history_prev(), Some("cmd2"));
        assert_eq!(shell.history_prev(), Some("cmd1"));
        assert_eq!(shell.history_next(), Some("cmd2"));
    }

    #[test]
    fn test_console_lines() {
        let mut shell = Shell::with_defaults();
        shell.execute("echo test");

        let lines = shell.console_lines();
        assert!(lines.len() >= 2); // input + output
    }

    #[test]
    fn test_clear_console() {
        let mut shell = Shell::with_defaults();
        shell.execute("echo test");
        assert!(!shell.console_lines().is_empty());

        shell.execute("clear");
        assert!(!shell.console_lines().is_empty()); // clear no borra líneas aún
        shell.clear_console();
        assert!(shell.console_lines().is_empty());
    }

    #[test]
    fn test_custom_handler() {
        let mut shell = Shell::new();

        shell.register("suma", |cmd| {
            let a: f64 = cmd.args.get(0).and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let b: f64 = cmd.args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0);
            ShellResult {
                success: true,
                output: format!("{} + {} = {}", a, b, a + b),
                data: Some(format!("{{\"result\":{}}}", a + b)),
            }
        });

        let result = shell.execute("suma 3 4");
        assert!(result.success);
        assert_eq!(result.output, "3 + 4 = 7");
    }
}
