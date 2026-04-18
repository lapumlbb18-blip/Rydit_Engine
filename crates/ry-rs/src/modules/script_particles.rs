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
use ry_gfx::gpu_particles::{ParticleEmitter, ParticleSystem};
use std::cell::RefCell;

// Sistema de partículas global (accesde desde .rydit)
thread_local! {
    static PARTICLES: RefCell<ParticleSystem> = RefCell::new(ParticleSystem::new());
}

/// Ejecutar función de partículas desde .rydit
pub fn ejecutar_funcion<'a>(
    name: &str,
    args: &[Expr<'a>],
    executor: &mut Executor,
    input: &crate::interpreter::InputEstado,
    funcs: &mut std::collections::HashMap<String, (Vec<String>, Vec<lizer::Stmt>)>,
) -> Option<Valor> {
    // particles::create_emitter(nombre, x, y, rate)
    if name == "particles::create_emitter" && args.len() >= 4 {
        let nombre_val = crate::interpreter::evaluar_expr_gfx(&args[0], executor, input, funcs);
        let x_val = crate::interpreter::evaluar_expr_gfx(&args[1], executor, input, funcs);
        let y_val = crate::interpreter::evaluar_expr_gfx(&args[2], executor, input, funcs);
        let rate_val = crate::interpreter::evaluar_expr_gfx(&args[3], executor, input, funcs);

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
        let nombre_val = crate::interpreter::evaluar_expr_gfx(&args[0], executor, input, funcs);
        let tipo_val = crate::interpreter::evaluar_expr_gfx(&args[1], executor, input, funcs);

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
        let nombre_val = crate::interpreter::evaluar_expr_gfx(&args[0], executor, input, funcs);

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
        let dt_val = crate::interpreter::evaluar_expr_gfx(&args[0], executor, input, funcs);

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
        return Some(Valor::Texto(
            "particles::draw() - listo para dibujar".to_string(),
        ));
    }

    // 🆕 v0.19.2: particles::enable_velocity_color(max_speed) — activar color por velocidad
    if name == "particles::enable_velocity_color" && args.len() == 1 {
        let max_speed_val = crate::interpreter::evaluar_expr_gfx(&args[0], executor, input, funcs);
        if let Valor::Num(max_speed) = max_speed_val {
            executor.guardar("__PARTICLE_VELOCITY_COLOR__", Valor::Bool(true));
            executor.guardar("__PARTICLE_MAX_SPEED__", Valor::Num(max_speed));
            return Some(Valor::Texto(format!("Color por velocidad activado (max_speed={})", max_speed)));
        } else {
            return Some(Valor::Error(
                "particles::enable_velocity_color() requiere num (max_speed)".to_string(),
            ));
        }
    }

    // 🆕 v0.19.2: particles::enable_additive_blend() — activar blend aditivo (explosiones brillantes)
    if name == "particles::enable_additive_blend" && args.len() == 0 {
        PARTICLES.with(|p| {
            let mut system = p.borrow_mut();
            system.additive_blend = true;
        });
        return Some(Valor::Texto("Blend aditivo activado — explosiones brillantes".to_string()));
    }

    // particles::set_gravity(gravity)
    if name == "particles::set_gravity" && args.len() == 1 {
        let gravity_val = crate::interpreter::evaluar_expr_gfx(&args[0], executor, input, funcs);

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
#[allow(dead_code)]
pub fn draw_particles<'a>(gfx: &mut ry_gfx::RyditGfx) {
    PARTICLES.with(|p| {
        let system = p.borrow();
        let mut d = gfx.begin_draw();
        system.draw(&mut d.draw);
        drop(d);
    });
}

/// ✅ v0.10.4: Dibujar partículas con DrawHandle existente (para integrar con RenderQueue)
pub fn draw_particles_with_handle<'a>(d: &mut ry_gfx::DrawHandle) {
    PARTICLES.with(|p| {
        let system = p.borrow();
        system.draw(&mut d.draw);
    });
}

/// 🆕 v0.19.2: Dibujar partículas con color por velocidad
pub fn draw_particles_with_handle_velocity<'a>(d: &mut ry_gfx::DrawHandle, max_speed: f32) {
    PARTICLES.with(|p| {
        let system = p.borrow();
        system.draw_with_velocity(&mut d.draw, max_speed);
    });
}

/// Limpiar partículas al iniciar un nuevo script
#[allow(dead_code)] // Para futura limpieza manual de partículas
pub fn clear_particles<'a>() {
    PARTICLES.with(|p| {
        let mut system = p.borrow_mut();
        system.clear();
    });
}
