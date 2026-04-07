// Fragment Shader para GPU Instancing - RyDit v0.15.0
// Renderiza círculos suaves con anti-aliasing
// FIX v0.15.0: gl_PointCoord solo funciona con gl_POINTS.
// Para QUADS instanciados, calculamos la distancia desde las coordenadas del vértice.

#version 330 core

// Posición interpolada del vértice (viene del vertex shader)
in vec4 vColor;

// Coordenada local del quad interpolada (-0.5 a 0.5 en X e Y)
// La reconstruimos desde gl_FragCoord y el centro de la partícula
// Pero como no tenemos uniform del centro, usamos un truco:
// el vertex shader puede pasar la coordenada local como varying.

// FIX: agregamos varying para coordenada local del quad
in vec2 vLocalPos;

// Salida final
out vec4 fragColor;

void main() {
    // CUAD SÓLIDO — igual que SDL2 fill_rect (sin descarte de círculo)
    fragColor = vColor;
}
