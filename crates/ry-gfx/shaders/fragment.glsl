// Fragment Shader GPU Instancing - RyDit v0.16.0
// Renderiza círculos SUAVES con anti-aliasing + alpha blending
//
// v0.16.0: smoothstep anti-aliasing + alpha por partícula
// v0.15.0: FIX gl_PointCoord solo funciona con gl_POINTS

#version 330 core

in vec4 vColor;
in vec2 vLocalPos;

out vec4 fragColor;

// Uniform para controlar suavizado (0.0 = borde duro, 1.0 = borde suave)
uniform float uSmoothness;

void main() {
    // Distancia desde el centro del quad (0 = centro, 0.707 = esquina)
    float dist = length(vLocalPos);

    // Radio del círculo (0.5 = ocupa todo el quad)
    float radius = 0.5;

    if (uSmoothness > 0.01) {
        // CÍRCULO SUAVE con anti-aliasing
        // smoothstep crea transición gradual entre borde interior y exterior
        float alpha = smoothstep(radius, radius - uSmoothness, dist);
        fragColor = vec4(vColor.rgb, vColor.a * alpha);

        // Descartar fragmentos completamente transparentes (optimización)
        if (fragColor.a < 0.01) discard;
    } else {
        // CUAD SÓLIDO — igual que SDL2 fill_rect (sin suavizado)
        fragColor = vColor;
    }
}
