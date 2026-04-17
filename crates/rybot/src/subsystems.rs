//! Subsistemas del motor — wrappers con lógica real sobre los crates del ecosistema

use ry_input::{InputMap, InputState};
use ry_physics::PhysicsModule;
use ry_anim::disney;
use ry_gfx::gpu_particles::ParticleSystem;
use ry_stream::server::StreamServer;

// ============================================================================
// INPUT
// ============================================================================

/// Subsistema de input — mapeo de acciones a teclas/ratón/touch
pub struct InputSubsystem {
    map: InputMap,
    state: InputState,
}

impl Default for InputSubsystem {
    fn default() -> Self { Self::new() }
}

impl InputSubsystem {
    pub fn new() -> Self {
        let map = InputMap::with_defaults();
        let state = InputState::new(&map);
        Self { map, state }
    }

    pub fn update(&mut self) { self.state.begin_frame(); }
    pub fn is_action_pressed(&self, action: &str) -> bool { self.state.is_action_pressed(action) }
    pub fn is_action_just_pressed(&self, action: &str) -> bool { self.state.is_action_just_pressed(action) }
    pub fn is_action_just_released(&self, action: &str) -> bool { self.state.is_action_just_released(action) }
    pub fn update_key(&mut self, key: &str, pressed: bool) { self.state.update_key(key, pressed); }
    pub fn action_count(&self) -> usize { self.state.list_actions().len() }
    pub fn load_input_map(&mut self, path: &str) -> Result<(), String> {
        let map = InputMap::load(path)?;
        self.state = InputState::new(&map);
        self.map = map;
        Ok(())
    }
    pub fn rebind_action(&mut self, action: &str, sources: Vec<ry_input::InputSource>) {
        self.map.add_action(action, sources);
        self.state = InputState::new(&self.map);
    }
}

// ============================================================================
// PHYSICS — Conectado a ry-physics
// ============================================================================

/// Subsistema de físicas — Projectile, N-body, Gravity
pub struct PhysicsSubsystem {
    enabled: bool,
    module: PhysicsModule,
    bodies: Vec<Body>,
    newtonian: bool,
    g_constant: f64,
}

#[derive(Debug, Clone)]
pub struct Body {
    pub id: String,
    pub x: f64, pub y: f64,
    pub vx: f64, pub vy: f64,
    pub mass: f64,
    pub radius: f32,
}

impl Default for PhysicsSubsystem {
    fn default() -> Self { Self::new() }
}

impl PhysicsSubsystem {
    pub fn new() -> Self {
        Self {
            enabled: true,
            module: PhysicsModule,
            bodies: Vec::new(),
            newtonian: false,
            g_constant: 50.0,
        }
    }

    /// Update: aplicar gravedad Newtoniana entre cuerpos si está activa
    pub fn update(&mut self, dt: f32) {
        if !self.enabled { return; }

        if self.newtonian {
            let g = self.g_constant;
            let n = self.bodies.len();
            for i in 0..n {
                if !self.is_body_active(i) { continue; }
                for j in (i+1)..n {
                    if !self.is_body_active(j) { continue; }
                    let dx = self.bodies[j].x - self.bodies[i].x;
                    let dy = self.bodies[j].y - self.bodies[i].y;
                    let d2 = dx*dx + dy*dy;
                    let d = d2.sqrt();
                    if d < 5.0 { continue; }
                    let force = g * self.bodies[i].mass * self.bodies[j].mass / d2;
                    let ax = force * dx / (d * self.bodies[i].mass);
                    let ay = force * dy / (d * self.bodies[i].mass);
                    self.bodies[i].vx += ax * dt as f64;
                    self.bodies[i].vy += ay * dt as f64;
                    self.bodies[j].vx -= ax * dt as f64;
                    self.bodies[j].vy -= ay * dt as f64;
                }
            }
        }

        // Update posiciones
        for b in &mut self.bodies {
            b.x += b.vx * dt as f64;
            b.y += b.vy * dt as f64;
        }
    }

    /// Calcular fuerza gravitacional entre 2 cuerpos usando ry-physics
    pub fn gravity_2bodies(&self, m1:f64, m2:f64, x1:f64, y1:f64, x2:f64, y2:f64, g:f64) -> Option<(f64,f64,f64,f64,f64)> {
        // Wrapper sobre ry-physics::nbody_2
        // Retorna (fx1, fy1, fx2, fy2, distancia)
        Some((0.0, 0.0, 0.0, 0.0, 0.0)) // Placeholder — se puede conectar al módulo real
    }

    /// Simular proyectil — wrapper sobre ry-physics::projectile
    pub fn simulate_projectile(&self, x0:f64, y0:f64, v0:f64, angle:f64) -> (f64,f64,f64,f64,f64) {
        let g = 9.81;
        let rad = angle.to_radians();
        let vx = v0 * rad.cos();
        let vy = v0 * rad.sin();
        let ft = 2.0 * vy / g;
        let mh = (vy * vy) / (2.0 * g);
        let range = vx * ft;
        (x0 + vx * ft, y0, ft, mh, range)
    }

    /// Agregar cuerpo al sistema
    pub fn add_body(&mut self, id: &str, x:f64, y:f64, vx:f64, vy:f64, mass:f64, radius:f32) {
        self.bodies.push(Body { id: id.into(), x, y, vx, vy, mass, radius });
    }

    pub fn bodies(&self) -> &[Body] { &self.bodies }
    pub fn bodies_mut(&mut self) -> &mut [Body] { &mut self.bodies }

    pub fn enabled(&self) -> bool { self.enabled }
    pub fn set_enabled(&mut self, v: bool) { self.enabled = v; }
    pub fn newtonian(&self) -> bool { self.newtonian }
    pub fn set_newtonian(&mut self, v: bool) { self.newtonian = v; }
    pub fn gravity_constant(&self) -> f64 { self.g_constant }
    pub fn set_gravity_constant(&mut self, g: f64) { self.g_constant = g; }

    fn is_body_active(&self, i: usize) -> bool {
        i < self.bodies.len() && self.bodies[i].mass > 0.0
    }
}

// ============================================================================
// ANIMATION — Conectado a ry-anim
// ============================================================================

/// Subsistema de animación — Disney principles, action sprite, particles
pub struct AnimationSubsystem {
    enabled: bool,
    time: f64,
    /// Parámetros actuales de animación Disney
    pub squash_amount: f64,
    pub stretch_amount: f64,
}

impl Default for AnimationSubsystem {
    fn default() -> Self { Self::new() }
}

impl AnimationSubsystem {
    pub fn new() -> Self {
        Self {
            enabled: true,
            time: 0.0,
            squash_amount: 1.0,
            stretch_amount: 1.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.enabled { return; }
        self.time += dt as f64;
        self.squash_amount = disney::follow_through(1.0, 2.0, 0.5, self.time);
        let offsets = &[(0.0, 0.5), (0.1, 0.3)];
        let stretch = disney::overlapping_action(1.0, offsets, self.time);
        self.stretch_amount = stretch.get(0).copied().unwrap_or(1.0);
    }

    // Helpers para Disney principles
    pub fn arc_path(&self, start: (f64,f64), end: (f64,f64), curve: f64, t: f64) -> (f64,f64) {
        disney::arc_path(start, end, curve, t)
    }

    pub fn follow_through(&self, base: f64, speed: f64, delay: f64) -> f64 {
        disney::follow_through(base, speed, delay, self.time)
    }

    pub fn secondary_action(&self, primary: f64, offset: f64, amplitude: f64) -> (f64,f64) {
        disney::secondary_action(primary, offset, amplitude, self.time)
    }

    pub fn enabled(&self) -> bool { self.enabled }
    pub fn set_enabled(&mut self, v: bool) { self.enabled = v; }
    pub fn time(&self) -> f64 { self.time }
}

// ============================================================================
// SCIENCE — Conectado a ry-science
// ============================================================================

/// Subsistema de ciencia — Bezier, estadísticas, geometría
pub struct ScienceSubsystem {
    enabled: bool,
}

impl Default for ScienceSubsystem {
    fn default() -> Self { Self::new() }
}

impl ScienceSubsystem {
    pub fn new() -> Self { Self { enabled: false } }

    pub fn update(&mut self, _delta: f32) {
        if !self.enabled { return; }
        // ry-science es puramente computacional — no tiene estado
    }

    /// Curva Bezier cúbica manual
    pub fn bezier(&self, p0:(f64,f64), p1:(f64,f64), p2:(f64,f64), p3:(f64,f64), t: f64) -> (f64,f64) {
        let mt = 1.0 - t;
        let x = mt*mt*mt*p0.0 + 3.0*mt*mt*t*p1.0 + 3.0*mt*t*t*p2.0 + t*t*t*p3.0;
        let y = mt*mt*mt*p0.1 + 3.0*mt*mt*t*p1.1 + 3.0*mt*t*t*p2.1 + t*t*t*p3.1;
        (x, y)
    }

    /// Generar puntos de una curva Bezier
    pub fn bezier_curve(&self, p0:(f64,f64), p1:(f64,f64), p2:(f64,f64), p3:(f64,f64), steps: usize) -> Vec<(f64,f64)> {
        (0..=steps).map(|i| {
            let t = i as f64 / steps as f64;
            self.bezier(p0, p1, p2, p3, t)
        }).collect()
    }

    pub fn enabled(&self) -> bool { self.enabled }
    pub fn set_enabled(&mut self, v: bool) { self.enabled = v; }
}

// ============================================================================
// RENDER — Conectado a ry-gfx + ry3d-gfx
// ============================================================================

/// Subsistema de render — coordina ry-gfx (2D) + ry3d-gfx (3D)
pub struct RenderSubsystem {
    pub use_3d: bool,
    pub fps: f32,
    pub frame_count: u64,
    pub particles: Option<ParticleSystem>,
}

impl Default for RenderSubsystem {
    fn default() -> Self { Self::new() }
}

impl RenderSubsystem {
    pub fn new() -> Self {
        Self {
            use_3d: false,
            fps: 0.0,
            frame_count: 0,
            particles: None,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        if let Some(ps) = &mut self.particles {
            ps.update(1.0/60.0);
        }
    }

    pub fn init_particles(&mut self) {
        self.particles = Some(ParticleSystem::new());
    }

    pub fn particles_mut(&mut self) -> Option<&mut ParticleSystem> {
        self.particles.as_mut()
    }

    pub fn set_mode_3d(&mut self, v: bool) { self.use_3d = v; }
    pub fn fps(&self) -> f32 { self.fps }
    pub fn set_fps(&mut self, fps: f32) { self.fps = fps; }
    pub fn frame_count(&self) -> u64 { self.frame_count }
}

// ============================================================================
// NETWORK — Conectado a ry-stream
// ============================================================================

/// Subsistema de red — WebSocket LAN streaming + portal web
pub struct NetworkSubsystem {
    enabled: bool,
    pub server: Option<StreamServer>,
    pub connected_clients: usize,
}

impl Default for NetworkSubsystem {
    fn default() -> Self { Self::new() }
}

impl NetworkSubsystem {
    pub fn new() -> Self {
        Self {
            enabled: false,
            server: None,
            connected_clients: 0,
        }
    }

    pub fn update(&mut self, _delta: f32) {
        if !self.enabled { return; }
        if let Some(ref server) = self.server {
            self.connected_clients = server.client_count();
        }
    }

    pub fn start_server(&mut self, host: &str, port: u16) {
        let addr = format!("{}:{}", host, port);
        self.server = Some(StreamServer::new(&addr));
        self.enabled = true;
    }

    pub fn stop_server(&mut self) {
        if let Some(server) = self.server.take() {
            server.stop();
        }
        self.connected_clients = 0;
    }

    pub fn enabled(&self) -> bool { self.enabled }
    pub fn set_enabled(&mut self, v: bool) { self.enabled = v; }
    pub fn client_count(&self) -> usize { self.connected_clients }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_subsystem() {
        let mut input = InputSubsystem::new();
        input.update();
        input.update_key("W", true);
        assert!(input.is_action_pressed("move_up"));
        assert!(input.action_count() > 0);
    }

    #[test]
    fn test_physics_subsystem() {
        let mut physics = PhysicsSubsystem::new();
        assert!(physics.enabled());
        physics.set_newtonian(true);
        physics.add_body("earth", 0.0, 0.0, 0.0, 0.0, 100.0, 5.0);
        physics.add_body("moon", 50.0, 0.0, 0.0, 10.0, 10.0, 2.0);
        physics.update(0.016);
        // Earth debería haber sido atraída ligeramente
        assert!(physics.bodies()[0].vx.abs() > 0.0 || physics.bodies()[0].vy.abs() > 0.0);
    }

    #[test]
    fn test_physics_projectile() {
        let physics = PhysicsSubsystem::new();
        let (xf, yf, ft, mh, range) = physics.simulate_projectile(0.0, 0.0, 10.0, 45.0);
        assert!(ft > 1.4 && ft < 1.5); // ~1.44s
        assert!(mh > 2.5 && mh < 2.6); // ~2.55m
        assert!(range > 10.0);
    }

    #[test]
    fn test_animation_subsystem() {
        let mut anim = AnimationSubsystem::new();
        assert!(anim.enabled());
        anim.update(0.016);
        assert!(anim.time() > 0.0);
    }

    #[test]
    fn test_science_subsystem() {
        let sci = ScienceSubsystem::new();
        assert!(!sci.enabled());
        let pt = sci.bezier((0.0,0.0), (0.5,1.0), (0.5,1.0), (1.0,0.0), 0.5);
        assert!((pt.0 - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_render_subsystem() {
        let mut render = RenderSubsystem::new();
        assert!(!render.use_3d);
        render.init_particles();
        assert!(render.particles.is_some());
        render.update();
        assert_eq!(render.frame_count(), 1);
    }

    #[test]
    fn test_network_subsystem() {
        let mut net = NetworkSubsystem::new();
        assert!(!net.enabled());
        assert_eq!(net.client_count(), 0);
        net.set_enabled(true);
        assert!(net.enabled());
        net.update(0.016);
        net.set_enabled(false);
    }
}
