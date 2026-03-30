import os
import sdl2
import moderngl
import numpy as np
import ctypes
import math
import time

# 1. Parche ModernGL para Python 3.13
os.environ['DISPLAY'] = ':0'
os.environ['MESA_LOADER_DRIVER_OVERRIDE'] = 'zink'

import _moderngl
class FixedLoader:
    def __init__(self):
        self.load_opengl_function = lambda name: sdl2.video.SDL_GL_GetProcAddress(name.encode())
_moderngl.DefaultLoader = FixedLoader

# 2. Inicializar SDL2 y ModernGL
sdl2.SDL_Init(sdl2.SDL_INIT_VIDEO)
width, height = 1280, 720
window = sdl2.SDL_CreateWindow(b"Tanke Auto-Follow - MOUSE: Mover/Apuntar/Fuego", 
                              sdl2.SDL_WINDOWPOS_CENTERED, sdl2.SDL_WINDOWPOS_CENTERED, 
                              width, height, sdl2.SDL_WINDOW_OPENGL)
gl_context = sdl2.SDL_GL_CreateContext(window)
ctx = moderngl.create_context()

# 3. Shaders
prog = ctx.program(
    vertex_shader='''
        #version 330
        uniform mat4 model;
        uniform mat4 projection;
        in vec2 in_vert;
        void main() {
            gl_Position = projection * model * vec4(in_vert, 0.0, 1.0);
        }
    ''',
    fragment_shader='''
        #version 330
        uniform vec3 color;
        uniform float alpha;
        out vec4 f_color;
        void main() {
            f_color = vec4(color, alpha);
        }
    '''
)

rect_verts = np.array([-0.5, -0.5, 0.5, -0.5, -0.5, 0.5, 0.5, 0.5], dtype='f4')
vbo = ctx.buffer(rect_verts)
vao = ctx.vertex_array(prog, [(vbo, '2f', 'in_vert')])

def get_ortho(l, r, b, t):
    m = np.eye(4, dtype='f4')
    m[0, 0], m[1, 1], m[2, 2] = 2/(r-l), 2/(t-b), -1
    m[0, 3], m[1, 3] = -(r+l)/(r-l), -(t+b)/(t-b)
    return m.T.copy(order='C')

def get_model(x, y, ang, sx, sy):
    rad = math.radians(ang)
    c, s = math.cos(rad), math.sin(rad)
    m = np.eye(4, dtype='f4')
    m[0:2, 0:2] = [[c*sx, -s*sy], [s*sx, c*sy]]
    m[0:2, 3] = [x, y]
    return m.T.copy(order='C')

prog['projection'].write(get_ortho(0, width, height, 0))

def draw_rect(x, y, ang, sx, sy, color, alpha=1.0):
    prog['color'].value = color
    prog['alpha'].value = alpha
    prog['model'].write(get_model(x, y, ang, sx, sy))
    vao.render(moderngl.TRIANGLE_STRIP)

# --- CLASES DE JUEGO ---
class Tank:
    def __init__(self, x, y, color):
        self.pos = np.array([float(x), float(y)])
        self.angle = 0.0
        self.turret_angle = 0.0
        self.color = color
        self.radius = 35.0
        self.health = 100
        self.alive = True

    def draw(self):
        if not self.alive: return
        draw_rect(self.pos[0]+4, self.pos[1]+4, self.angle, 90, 65, (0,0,0), 0.3)
        draw_rect(self.pos[0], self.pos[1], self.angle, 80, 55, self.color)
        draw_rect(self.pos[0], self.pos[1], self.turret_angle, 45, 45, tuple(c*1.2 for c in self.color))
        cx = self.pos[0] + math.cos(math.radians(self.turret_angle)) * 25
        cy = self.pos[1] + math.sin(math.radians(self.turret_angle)) * 25
        draw_rect(cx, cy, self.turret_angle, 55, 12, (0.1, 0.1, 0.1))

class Bullet:
    def __init__(self, x, y, angle, owner):
        self.pos = np.array([float(x), float(y)])
        self.angle = angle
        self.owner = owner
        self.speed = 15.0
        self.life = 0
        self.active = True

    def update(self):
        self.pos[0] += math.cos(math.radians(self.angle)) * self.speed
        self.pos[1] += math.sin(math.radians(self.angle)) * self.speed
        self.life += 1
        if self.life > 80: self.active = False
        if self.pos[0] < 0 or self.pos[0] > width or self.pos[1] < 0 or self.pos[1] > height:
            self.active = False

    def draw(self):
        draw_rect(self.pos[0], self.pos[1], self.angle, 18, 7, (1.0, 0.9, 0.2))

# Inicializar
player = Tank(200, 360, (0.2, 0.4, 0.2))
enemy = Tank(1000, 360, (0.6, 0.2, 0.2))
bullets = []

running = True
event = sdl2.SDL_Event()
mouse_pos = [0, 0]

print("¡Tanke Auto-Follow Activado! Mueve el ratón para que el tanke te siga.")

while running:
    while sdl2.SDL_PollEvent(ctypes.byref(event)) != 0:
        if event.type == sdl2.SDL_QUIT: running = False
        if event.type == sdl2.SDL_MOUSEMOTION:
            mouse_pos = [event.motion.x, event.motion.y]
        if event.type == sdl2.SDL_MOUSEBUTTONDOWN:
            if event.button.button == sdl2.SDL_BUTTON_LEFT and player.alive:
                bullets.append(Bullet(player.pos[0], player.pos[1], player.turret_angle, "player"))

    # --- LÓGICA JUGADOR (MOVIMIENTO AUTOMÁTICO) ---
    if player.alive:
        dx = mouse_pos[0] - player.pos[0]
        dy = mouse_pos[1] - player.pos[1]
        dist = math.sqrt(dx**2 + dy**2)
        target_ang = math.degrees(math.atan2(dy, dx))
        
        # Rotar torreta instantánea al ratón
        player.turret_angle = target_ang
        
        # Movimiento automático del chasis
        if dist > 60: # Solo se mueve si el ratón está algo lejos
            # Suavizar rotación del chasis hacia el ratón
            angle_diff = (target_ang - player.angle + 180) % 360 - 180
            player.angle += angle_diff * 0.1
            
            # Avanzar
            player.pos[0] += math.cos(math.radians(player.angle)) * 3.5
            player.pos[1] += math.sin(math.radians(player.angle)) * 3.5

    # --- LÓGICA ENEMIGO (IA) ---
    if enemy.alive:
        edx = player.pos[0] - enemy.pos[0]
        edy = player.pos[1] - enemy.pos[1]
        dist_e = math.sqrt(edx**2 + edy**2)
        target_ang_e = math.degrees(math.atan2(edy, edx))
        
        if dist_e > 250:
            enemy.angle = target_ang_e
            enemy.pos[0] += math.cos(math.radians(enemy.angle)) * 2.2
            enemy.pos[1] += math.sin(math.radians(enemy.angle)) * 2.2
        
        enemy.turret_angle = target_ang_e
        if np.random.random() < 0.02 and player.alive:
            bullets.append(Bullet(enemy.pos[0], enemy.pos[1], enemy.turret_angle, "enemy"))

    # --- COLISIONES Y BALAS ---
    new_bullets = []
    for b in bullets:
        b.update()
        if not b.active: continue
        hit = False
        if b.owner == "player" and enemy.alive:
            if np.linalg.norm(b.pos - enemy.pos) < enemy.radius:
                enemy.health -= 20; hit = True
                if enemy.health <= 0: enemy.alive = False
        if b.owner == "enemy" and player.alive:
            if np.linalg.norm(b.pos - player.pos) < player.radius:
                player.health -= 15; hit = True
                if player.health <= 0: player.alive = False
        if not hit: new_bullets.append(b)
    bullets = new_bullets

    # Colisión entre tanques
    if player.alive and enemy.alive:
        d_vec = player.pos - enemy.pos
        d = np.linalg.norm(d_vec)
        if d < 70:
            player.pos += (d_vec / d) * (70 - d)

    # --- RENDER ---
    ctx.clear(0.12, 0.13, 0.11)
    ctx.enable(moderngl.BLEND)
    
    # Grid de fondo
    for i in range(0, 1300, 100):
        draw_rect(i, 360, 0, 1, 720, (1,1,1), 0.08)
        draw_rect(640, i, 0, 1280, 1, (1,1,1), 0.08)

    player.draw()
    enemy.draw()
    for b in bullets: b.draw()
    
    # UI Salud
    if player.alive:
        draw_rect(player.pos[0], player.pos[1]-65, 0, 80, 10, (0.1, 0.1, 0.1))
        draw_rect(player.pos[0]-40+player.health*0.4, player.pos[1]-65, 0, player.health*0.8, 8, (0.3, 0.9, 0.3))
    if enemy.alive:
        draw_rect(enemy.pos[0], enemy.pos[1]-65, 0, 80, 10, (0.1, 0.1, 0.1))
        draw_rect(enemy.pos[0]-40+enemy.health*0.4, enemy.pos[1]-65, 0, enemy.health*0.8, 8, (0.9, 0.3, 0.3))

    sdl2.SDL_GL_SwapWindow(window)
    sdl2.SDL_Delay(16)

sdl2.SDL_Quit()
