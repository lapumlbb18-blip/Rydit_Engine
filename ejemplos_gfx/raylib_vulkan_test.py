import pyray as rl
import numpy as np
import os

# Forzar GPU Adreno
os.environ['DISPLAY'] = ':0'
os.environ['MESA_LOADER_DRIVER_OVERRIDE'] = 'zink'

NUM_PARTICLES = 1200 # Un poco menos para asegurar fluidez en el loop
G = 0.05

rl.init_window(1280, 720, "Raylib Stress Test: Vulkan Active")
rl.set_target_fps(60)

# Datos NumPy
pos = np.random.uniform(-15, 15, (NUM_PARTICLES, 3)).astype('f4')
vel = np.random.uniform(-0.1, 0.1, (NUM_PARTICLES, 3)).astype('f4')

print("¡GPU Adreno 610 (Turnip/Vulkan) detectada y activa!")

while not rl.window_should_close():
    # Física NumPy
    dist = np.linalg.norm(pos, axis=1, keepdims=True)
    vel -= (pos / (dist**3 + 0.1)) * G
    pos += vel

    # Renderizado
    rl.begin_drawing()
    rl.clear_background(rl.BLACK)
    
    # Cámara estática simple
    camera = rl.Camera3D(
        rl.Vector3(30, 30, 30),
        rl.Vector3(0, 0, 0),
        rl.Vector3(0, 1, 0),
        45, 0 # Perspective
    )
    
    rl.begin_mode_3d(camera)
    # Dibujamos cubos muy pequeños como partículas
    size = rl.Vector3(0.1, 0.1, 0.1)
    for i in range(NUM_PARTICLES):
        # Usamos draw_cube que es muy compatible
        p = rl.Vector3(pos[i,0], pos[i,1], pos[i,2])
        rl.draw_cube(p, 0.15, 0.15, 0.15, rl.SKYBLUE)
    rl.end_mode_3d()
    
    rl.draw_fps(10, 10)
    rl.draw_text("Renderer: Zink (Vulkan/Turnip)", 10, 40, 20, rl.GREEN)
    rl.end_drawing()

rl.close_window()
