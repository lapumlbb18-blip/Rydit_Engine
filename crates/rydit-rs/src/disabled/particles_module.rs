// crates/rydit-rs/src/modules/particles.rs
// ✅ v0.9.2 - Sistema de partículas para scripts .rydit
//
// Uso en .rydit:
//   particles::create_emitter("fuego", x, y, rate)
//   particles::set_emitter_type("fuego", "fire")
//   particles::update(dt)
//   particles::draw()
//   particles::set_gravity(x, y)

use blast_core::{Executor, Valor};
use lizer::Expr;
use rydit_gfx::particles::{ParticleEmitter, ParticleSystem};
use std::cell::RefCell;

// Sistema de partículas global (accesde desde .rydit)
thread_local! {
    static PARTICLES: RefCell<ParticleSystem> = RefCell::new(ParticleSystem::new());
}

/// Ejecutar función de partículas desde .rydit
pub fn ejecutar_funcion(
    name: &str,
    args: &[Expr],
    executor: &mut Executor,
    input: &crate::InputEstado,
    funcs: &mut std::collections::HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Option<Valor> {
    // particles::create_emitter(nombre, x, y, rate)
    if name == "particles::create_emitter" && args.len() >= 4 {
        let nombre_val = crate::evaluar_expr_gfx(&args[0], executor, input, funcs);
        let x_val = crate::evaluar_expr_gfx(&args[1], executor, input, funcs);
        let y_val = crate::evaluar_expr_gfx(&args[2], executor, input, funcs);
        let rate_val = crate::evaluar_expr_gfx(&args[3], executor, input, funcs);

        if let (Valor::Texto(nombre), Valor::Num(x), Valor::Num(y), Valor::Num(rate)) =
            (nombre_val, x_val, y_val, rate_val)
        {
            PARTICLES.with(|p| {
                let mut system = p.borrow_mut();
                system.create_emitter(&nombre, x as f32, y as f32, rate as f32);
            });
            return Some(Valor::Texto(format!("Emisor '{}' creado", nombre)));
        } else {
            return Some(Valor::Error(
                "particles::create_emitter() requiere (texto, num, num, num)".to_string(),
            ));
        }
    }

    // particles::set_emitter_type(nombre, tipo)
    // Tipos: "fire", "smoke", "sparks", "explosion"
    if name == "particles::set_emitter_type" && args.len() == 2 {
        let nombre_val = crate::evaluar_expr_gfx(&args[0], executor, input, funcs);
        let tipo_val = crate::evaluar_expr_gfx(&args[1], executor, input, funcs);

        if let (Valor::Texto(nombre), Valor::Texto(tipo)) = (nombre_val, tipo_val) {
            PARTICLES.with(|p| {
                let mut system = p.borrow_mut();
                if let Some(emitter) = system.get_emitter_mut(&nombre) {
                    *emitter = match tipo.as_str() {
                        "fire" | "fuego" => ParticleEmitter::fire(emitter.x, emitter.y),
                        "smoke" | "humo" => ParticleEmitter::smoke(emitter.x, emitter.y),
                        "sparks" | "chispas" => ParticleEmitter::sparks(emitter.x, emitter.y),
                        "explosion" | "explosión" => {
                            ParticleEmitter::explosion(emitter.x, emitter.y)
                        }
                        _ => {
                            eprintln!("[WARN] Tipo de emisor desconocido: {}", tipo);
                            return;
                        }
                    };
                }
            });
            return Some(Valor::Texto(format!("Emisor '{}' tipo '{}'", nombre, tipo)));
        } else {
            return Some(Valor::Error(
                "particles::set_emitter_type() requiere (texto, texto)".to_string(),
            ));
        }
    }

    // particles::remove_emitter(nombre)
    if name == "particles::remove_emitter" && args.len() == 1 {
        let nombre_val = crate::evaluar_expr_gfx(&args[0], executor, input, funcs);

        if let Valor::Texto(nombre) = nombre_val {
            PARTICLES.with(|p| {
                let mut system = p.borrow_mut();
                system.remove_emitter(&nombre);
            });
            return Some(Valor::Texto(format!("Emisor '{}' removido", nombre)));
        } else {
            return Some(Valor::Error(
                "particles::remove_emitter() requiere texto".to_string(),
            ));
        }
    }

    // particles::update(dt)
    if name == "particles::update" && args.len() == 1 {
        let dt_val = crate::evaluar_expr_gfx(&args[0], executor, input, funcs);

        if let Valor::Num(dt) = dt_val {
            PARTICLES.with(|p| {
                let mut system = p.borrow_mut();
                system.update(dt as f32);
            });
            return Some(Valor::Num(0.0)); // Success
        } else {
            return Some(Valor::Error(
                "particles::update() requiere num (delta time)".to_string(),
            ));
        }
    }

    // particles::draw()
    if name == "particles::draw" && args.len() == 0 {
        // El draw se maneja especial en ejecutar_stmt_gfx
        // porque necesita acceso al DrawHandle
        return Some(Valor::Texto("particles::draw() - listo para dibujar".to_string()));
    }

    // particles::set_gravity(gravity)
    if name == "particles::set_gravity" && args.len() == 1 {
        let gravity_val = crate::evaluar_expr_gfx(&args[0], executor, input, funcs);

        if let Valor::Num(gravity) = gravity_val {
            PARTICLES.with(|p| {
                let mut system = p.borrow_mut();
                system.set_gravity(gravity as f32);
            });
            return Some(Valor::Texto(format!("Gravedad: {}", gravity)));
        } else {
            return Some(Valor::Error(
                "particles::set_gravity() requiere (num)".to_string(),
            ));
        }
    }

    // particles::particle_count() - Retorna número de partículas activas
    if name == "particles::particle_count" && args.len() == 0 {
        let count = PARTICLES.with(|p| {
            let system = p.borrow();
            system.particle_count()
        });
        return Some(Valor::Num(count as f64));
    }

    // particles::clear() - Remover todos los emisores
    if name == "particles::clear" && args.len() == 0 {
        PARTICLES.with(|p| {
            let mut system = p.borrow_mut();
            system.clear();
        });
        return Some(Valor::Texto("Partículas limpiadas".to_string()));
    }

    None // Función no reconocida
}

/// Dibujar partículas - se llama desde el game loop
pub fn draw_particles(gfx: &mut rydit_gfx::RyditGfx) {
    PARTICLES.with(|p| {
        let system = p.borrow();
        let mut d = gfx.begin_draw();
        system.draw(&mut d.draw);
        drop(d);
    });
}

/// Limpiar partículas al iniciar un nuevo script
#[allow(dead_code)] // Para futura limpieza manual de partículas
pub fn clear_particles() {
    PARTICLES.with(|p| {
        let mut system = p.borrow_mut();
        system.clear();
    });
}
