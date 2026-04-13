// demo_war_spacio.rs
// 🚀 War Spacio (Galaga) — SDL2 + Gravitación + Color Velocidad + Partículas
//
// cargo run --bin demo_war_spacio --release

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;
use ry_gfx::particles::ParticleSystem;
use ry_gfx::sdl2_helpers::*;

const W: u32 = 900;
const H: u32 = 600;
const PSPEED: f32 = 300.0;
const BSPEED: f32 = 500.0;
const COLS: usize = 8;
const ROWS: usize = 4;
const SP: f32 = 70.0;

#[derive(Clone)]
struct Player { x: f32, y: f32, lives: u32, score: u32 }
#[derive(Clone)]
struct Enemy { x: f32, y: f32, vx: f32, vy: f32, alive: bool, mass: f32, row: usize, dt_: f32, diving: bool }
#[derive(Clone)]
struct Bullet { x: f32, y: f32, vy: f32, active: bool }

fn rc(a:f32,b:f32,c:f32,d:f32,e:f32,f:f32,g:f32,h:f32) -> bool {
    a<e+g && a+c>e && b<f+h && b+d>f
}

fn rf() -> f32 {
    use std::time::{SystemTime,UNIX_EPOCH};
    ((SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as f32).sin()*10000.0).fract()
}

fn spawn_enemies() -> Vec<Enemy> {
    let mut v = Vec::new();
    let sx = (W as f32 - COLS as f32 * SP)/2.0 + 20.0;
    for r in 0..ROWS { for c in 0..COLS {
        v.push(Enemy { x:sx+c as f32*SP, y:60.0+r as f32*50.0, vx:(rf()-0.5)*30.0, vy:(rf()-0.5)*20.0,
            alive:true, mass:20.0+r as f32*10.0, row:r, dt_:0.0, diving:false });
    }}
    v
}

fn main() -> Result<(), String> {
    println!("🚀 War Spacio — SDL2 + Gravitación + Color Velocidad + Partículas");
    println!("   WASD: Mover | ESPACIO: Disparar | R: Reiniciar | ESC: Salir");

    let sdl = sdl2::init().map_err(|e|e.to_string())?;
    let video = sdl.video().map_err(|e|e.to_string())?;
    let mut ep = sdl.event_pump().map_err(|e|e.to_string())?;
    let win = video.window("🚀 War Spacio", W, H).position_centered().opengl().build().map_err(|e|e.to_string())?;
    let mut canvas = win.into_canvas().accelerated().present_vsync().build().map_err(|e|e.to_string())?;

    let mut ps = ParticleSystem::new(); ps.global_gravity = 100.0;
    let mut p = Player { x:W as f32/2.0-15.0, y:H as f32-60.0, lives:3, score:0 };
    let mut enemies = spawn_enemies();
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut energy = 0.0f64;
    let mut hits = 0u32;
    let mut over = false;
    let mut frame = 0u64;
    let mut ku = false; let mut kd = false; let mut kl = false; let mut kr = false;

    'run: loop {
        let dt = 1.0/60.0;

        for ev in ep.poll_iter() {
            match ev {
                Event::Quit{..}|Event::KeyDown{keycode:Some(Keycode::Escape),..}=>break 'run,
                Event::KeyDown{keycode:Some(Keycode::R),..}=>{
                    enemies=spawn_enemies(); bullets.clear(); ps.clear();
                    p.lives=3;p.score=0;energy=0.0;hits=0;over=false;
                }
                Event::KeyDown{keycode:Some(Keycode::Space),repeat:false,..} if !over=>{
                    bullets.push(Bullet{x:p.x+13.0,y:p.y,vy:-BSPEED,active:true});
                }
                Event::KeyDown{keycode:Some(Keycode::W),..}=>{ku=true;}
                Event::KeyUp{keycode:Some(Keycode::W),..}=>{ku=false;}
                Event::KeyDown{keycode:Some(Keycode::S),..}=>{kd=true;}
                Event::KeyUp{keycode:Some(Keycode::S),..}=>{kd=false;}
                Event::KeyDown{keycode:Some(Keycode::A),..}=>{kl=true;}
                Event::KeyUp{keycode:Some(Keycode::A),..}=>{kl=false;}
                Event::KeyDown{keycode:Some(Keycode::D),..}=>{kr=true;}
                Event::KeyUp{keycode:Some(Keycode::D),..}=>{kr=false;}
                _=>{}
            }
        }

        if over {
            canvas.set_draw_color(Color::RGB(5,5,15));let _=canvas.clear();
            canvas.set_draw_color(Color::RED);let _=canvas.fill_rect(Rect::new(W as i32/2-100,H as i32/2-30,200,60));
            canvas.set_draw_color(Color::WHITE);let _=canvas.fill_rect(Rect::new(W as i32/2-80,H as i32/2-10,160,40));
            canvas.present();std::thread::sleep(std::time::Duration::from_millis(16));continue;
        }

        if ku{p.y-=PSPEED*dt;}if kd{p.y+=PSPEED*dt;}if kl{p.x-=PSPEED*dt;}if kr{p.x+=PSPEED*dt;}
        p.x=p.x.max(0.0).min(W as f32-30.0);p.y=p.y.max(0.0).min(H as f32-30.0);

        // Gravedad entre enemigos
        let en_clone: Vec<Enemy> = enemies.clone();
        for i in 0..en_clone.len() {
            if !en_clone[i].alive { continue; }
            for j in (i+1)..en_clone.len() {
                if !en_clone[j].alive { continue; }
                let dx=en_clone[j].x-en_clone[i].x; let dy=en_clone[j].y-en_clone[i].y;
                let d2=dx*dx+dy*dy; let d=d2.sqrt(); if d<10.0{continue;}
                let g=50.0; let f=g*en_clone[i].mass*en_clone[j].mass/d2;
                let ax=f*dx/(d*en_clone[i].mass); let ay=f*dy/(d*en_clone[i].mass);
                enemies[i].vx+=ax*dt; enemies[i].vy+=ay*dt;
                enemies[j].vx-=ax*dt; enemies[j].vy-=ay*dt;
            }
        }

        for e in enemies.iter_mut() {
            if !e.alive{continue;}
            e.x+=e.vx*dt;e.y+=e.vy*dt;
            e.vy+=(frame as f32*0.02).sin()*10.0*dt;
            if e.x<0.0||e.x>W as f32-30.0{e.vx=-e.vx;e.x=e.x.max(0.0).min(W as f32-30.0);}
            e.dt_+=dt;
            if !e.diving&&e.dt_>5.0+rf()*5.0{e.diving=true;e.vy=150.0;e.vx=(rf()-0.5)*200.0;e.dt_=0.0;}
            if e.diving&&e.y>H as f32-100.0{e.diving=false;e.vy=-50.0;}
        }

        for b in bullets.iter_mut() {
            if !b.active{continue;}
            b.y+=b.vy*dt;
            if b.y < -10.0||b.y>H as f32+10.0{b.active=false;continue;}
            for e in enemies.iter_mut() {
                if !e.alive{continue;}
                if rc(b.x,b.y,4.0,10.0,e.x,e.y,30.0,20.0) {
                    b.active=false;e.alive=false;p.score+=100*(e.row as u32+1);hits+=1;
                    energy+=0.5*e.mass as f64*(e.vx*e.vx+e.vy*e.vy) as f64;break;
                }
            }
        }
        bullets.retain(|b|b.active);

        if enemies.iter().all(|e|!e.alive){enemies=spawn_enemies();}

        ps.update(dt);

        // RENDER
        canvas.set_draw_color(Color::RGB(5,5,15));let _=canvas.clear();
        canvas.set_draw_color(Color::RGB(80,80,120));
        for i in 0..80u32{let _=canvas.draw_point(Point::new(((i*7919+13)%W) as i32,((i*6271+37)%H) as i32));}

        for e in enemies.iter() {
            if !e.alive{continue;}
            let spd=(e.vx*e.vx+e.vy*e.vy).sqrt();
            let c=velocity_color_sdl2(spd,300.0);
            canvas.set_draw_color(Color::RGBA(c.r/3,c.g/3,c.b/3,60));
            let _=canvas.fill_rect(Rect::new(e.x as i32-4,e.y as i32-4,38,28));
            canvas.set_draw_color(c);
            let _=canvas.fill_rect(Rect::new(e.x as i32,e.y as i32,30,20));
            canvas.set_draw_color(Color::RGB(255,255,255));
            let _=canvas.fill_rect(Rect::new(e.x as i32+6,e.y as i32+6,4,4));
            let _=canvas.fill_rect(Rect::new(e.x as i32+20,e.y as i32+6,4,4));
        }

        canvas.set_draw_color(Color::RGB(255,255,100));
        for b in bullets.iter(){if b.active{let _=canvas.fill_rect(Rect::new(b.x as i32,b.y as i32,4,10));}}

        canvas.set_draw_color(Color::RGB(0,200,255));
        let _=canvas.fill_rect(Rect::new(p.x as i32+12,p.y as i32,6,30));
        let _=canvas.fill_rect(Rect::new(p.x as i32+2,p.y as i32+15,26,15));
        canvas.set_draw_color(Color::RGB(0,150,200));
        let _=canvas.fill_rect(Rect::new(p.x as i32,p.y as i32+22,6,8));
        let _=canvas.fill_rect(Rect::new(p.x as i32+24,p.y as i32+22,6,8));

        if ps.emitters.iter().any(|(_,e)|!e.particles.is_empty()) {
            set_blend_additive(&mut canvas);
            draw_particles_sdl2(&mut canvas,&ps,400.0);
            set_blend_normal(&mut canvas);
        }

        canvas.set_draw_color(Color::RGBA(0,0,0,150));
        let _=canvas.fill_rect(Rect::new(0,0,W,28));
        canvas.set_draw_color(Color::WHITE);
        let _=canvas.fill_rect(Rect::new(10,8,120,12));
        let _=canvas.fill_rect(Rect::new(150,8,180,12));
        let _=canvas.fill_rect(Rect::new(500,8,280,12));

        canvas.present();frame+=1;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    println!("\n✅ War Spacio cerrado — Score: {} | Hits: {}",p.score,hits);
    Ok(())
}
