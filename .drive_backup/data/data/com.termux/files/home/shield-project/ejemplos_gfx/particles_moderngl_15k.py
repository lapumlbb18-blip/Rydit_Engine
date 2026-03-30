#!/usr/bin/env python3
"""
Partículas Masivas con ModernGL - 15,000 partículas
Física CPU con NumPy + Renderizado GPU con instancing
GPU: Adreno 610 (Turnip/Vulkan via Zink)

IMPORTANTE: Las variables de entorno deben estar antes de importar moderngl
"""

import os
import sys

# CRITICAL: Set env vars BEFORE importing moderngl/sdl2
os.environ['DISPLAY'] = ':0'
os.environ['MESA_LOADER_DRIVER_OVERRIDE'] = 'zink'
os.environ['GALLIUM_DRIVER'] = 'zink'
os.environ['ZINK_DEBUG'] = 'sync'

import moderngl
import numpy as np
import time
import ctypes
import sdl2

# --- CONFIGURACIÓN SIMULACIÓN ---
NUM_PARTICLES = 15000
WIDTH, HEIGHT = 1280, 720
DT = 0.016
GRAVITY = -50.0
BOUNCE = 0.8

# SDL2 Init
if sdl2.SDL_Init(sdl2.SDL_INIT_VIDEO) != 0:
    print(f"Error SDL2: {sdl2.SDL_GetError()}")
    exit(1)

sdl2.SDL_GL_SetAttribute(sdl2.SDL_GL_CONTEXT_MAJOR_VERSION, 3)
sdl2.SDL_GL_SetAttribute(sdl2.SDL_GL_CONTEXT_MINOR_VERSION, 3)

window = sdl2.SDL_CreateWindow(
    b"Particulas Masivas - ModernGL 15K - ZINK",
    sdl2.SDL_WINDOWPOS_CENTERED, sdl2.SDL_WINDOWPOS_CENTERED,
    WIDTH, HEIGHT,
    sdl2.SDL_WINDOW_OPENGL | sdl2.SDL_WINDOW_SHOWN
)

gl_context = sdl2.SDL_GL_CreateContext(window)

# ModernGL con loader manual para Python 3.13
import _moderngl
class ManualLoader:
    def __init__(self):
        self.load_opengl_function = lambda name: sdl2.video.SDL_GL_GetProcAddress(name.encode())

_moderngl.DefaultLoader = ManualLoader
ctx = moderngl.create_context()

print(f"=== Partículas Masivas - ModernGL + Zink ===")
print(f"Partículas: {NUM_PARTICLES:,}")
print(f"GPU: {ctx.info['GL_RENDERER']}")
print(f"OpenGL: {ctx.info['GL_VERSION']}")
print(f"===========================================")

# Verificar si usa Zink
renderer = ctx.info['GL_RENDERER']
if 'zink' in renderer.lower() or 'turnip' in renderer.lower() or 'adreno' in renderer.lower():
    print("✓ USANDO GPU ADRENO (Zink/Vulkan)")
else:
    print(f"✗ WARNING: Usando {renderer} (no GPU Adreno)")

# --- SHADERS ---
prog = ctx.program(
    vertex_shader='''
        #version 330
        layout (location = 0) in vec2 in_vert;
        layout (location = 1) in vec2 in_pos;
        layout (location = 2) in vec3 in_color;
        
        out vec2 v_uv;
        out vec3 v_color;
        
        uniform vec2 screen;
        
        void main() {
            v_uv = in_vert;
            v_color = in_color;
            vec2 ndc = (in_pos / screen) * 2.0 - 1.0;
            gl_Position = vec4(ndc, 0.0, 1.0);
            gl_Position.xy += in_vert * 0.015;
        }
    ''',
    fragment_shader='''
        #version 330
        in vec2 v_uv;
        in vec3 v_color;
        out vec4 f_color;
        
        void main() {
            float r2 = dot(v_uv, v_uv);
            if (r2 > 1.0) discard;
            float glow = 1.0 - r2;
            f_color = vec4(v_color * glow, 1.0);
        }
    '''
)

# --- DATOS INICIALES ---
pos = np.random.uniform(100, WIDTH - 100, (NUM_PARTICLES, 2)).astype('f4')
vel = np.column_stack([
    np.random.uniform(-100, 100, NUM_PARTICLES),
    np.random.uniform(-100, 100, NUM_PARTICLES)
]).astype('f4')

colors = np.column_stack([
    np.random.uniform(0.5, 1.0, NUM_PARTICLES),
    np.random.uniform(0.3, 0.9, NUM_PARTICLES),
    np.ones(NUM_PARTICLES, dtype='f4')
]).astype('f4')

# Buffers - crear con espacio extra para updates dinámicos
vbo_pos = ctx.buffer(pos.tobytes() + b'\x00' * (NUM_PARTICLES * 8))
vbo_color = ctx.buffer(colors.tobytes() + b'\x00' * (NUM_PARTICLES * 12))

# Círculo geometry
circle = np.array([
    0, 0,  1, 0,  1, 1,  0, 1,  -1, 1,  -1, 0,  -1, -1,  0, -1,  1, -1,  1, 0
], dtype='f4')
vbo_circle = ctx.buffer(circle)

# VAO con instancing
vao = ctx.vertex_array(
    prog,
    [
        (vbo_circle, '2f', 'in_vert'),
        (vbo_pos, '2f/i', 'in_pos'),
        (vbo_color, '3f/i', 'in_color'),
    ],
)

# Uniformes
prog['screen'] = (float(WIDTH), float(HEIGHT))

# --- BUCLE PRINCIPAL ---
running = True
event = sdl2.SDL_Event()
clock = time.time()
fps = 0
frame_count = 0
max_frames = 300  # ~5 segundos

print("Iniciando... (ESC para salir)")

while running and frame_count < max_frames:
    while sdl2.SDL_PollEvent(ctypes.byref(event)) != 0:
        if event.type == sdl2.SDL_QUIT:
            running = False
        elif event.type == sdl2.SDL_KEYDOWN:
            if event.key.keysym.sym == sdl2.SDLK_ESCAPE:
                running = False
    
    # Física NumPy (CPU pero vectorizada)
    vel[:, 1] += GRAVITY * DT
    pos += vel * DT
    
    # Rebotes
    mask_x_left = pos[:, 0] <= 10
    mask_x_right = pos[:, 0] >= WIDTH - 10
    mask_y_bottom = pos[:, 1] <= 10
    mask_y_top = pos[:, 1] >= HEIGHT - 10
    
    vel[mask_x_left | mask_x_right, 0] *= -BOUNCE
    vel[mask_y_bottom | mask_y_top, 1] *= -BOUNCE
    
    pos[:, 0] = np.clip(pos[:, 0], 10, WIDTH - 10)
    pos[:, 1] = np.clip(pos[:, 1], 10, HEIGHT - 10)
    
    # Color por velocidad
    speeds = np.linalg.norm(vel, axis=1, keepdims=True)
    colors[:, 0] = 0.3 + np.clip(speeds[:, 0] / 200, 0, 0.7)
    colors[:, 1] = 0.5 + np.clip(speeds[:, 0] / 150, 0, 0.5)
    
    # Actualizar buffers
    vbo_pos.write(pos.tobytes(), 0)
    vbo_color.write(colors.tobytes(), 0)
    
    # Render
    ctx.clear(0.02, 0.02, 0.05, 1.0)
    ctx.enable(moderngl.BLEND)
    vao.render(moderngl.TRIANGLE_FAN, instances=NUM_PARTICLES)
    
    sdl2.SDL_GL_SwapWindow(window)
    
    # FPS
    frame_count += 1
    if frame_count % 60 == 0:
        elapsed = time.time() - clock
        fps = 60 / elapsed if elapsed > 0 else 0
        title = f"Particulas ModernGL+Zink - {NUM_PARTICLES:,} - FPS: {fps:.1f}".encode()
        sdl2.SDL_SetWindowTitle(window, title)
        print(f"\rFPS: {fps:.1f} | Frames: {frame_count}", end='', flush=True)
        clock = time.time()

sdl2.SDL_GL_DeleteContext(gl_context)
sdl2.SDL_DestroyWindow(window)
sdl2.SDL_Quit()

print(f"\n\nTest completado - FPS promedio: {fps:.1f}")
