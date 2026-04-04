// Fragment Shader para GPU Instancing - RyDit v0.10.1
// Renderiza círculos suaves con anti-aliasing

#version 330 core

// Color interpolado desde el vertex shader
in vec4 vColor;

// Salida final
out vec4 fragColor;

void main() {
    // Calcular distancia desde el centro del quad
    vec2 coord = gl_PointCoord * 2.0 - 1.0;
    float dist = length(coord);
    
    // Crear círculo suave (anti-aliasing)
    if (dist > 1.0) {
        discard;
    }
    
    // Aplicar color con alpha
    fragColor = vColor;
}
