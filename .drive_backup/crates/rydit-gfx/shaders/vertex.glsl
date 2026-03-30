// Vertex Shader para GPU Instancing - RyDit v0.10.1
// Renderiza 100K+ partículas en un solo draw call

#version 330 core

// Atributos por vértice (posición del quad)
layout(location = 0) in vec2 aPosition;  // Posición del vértice en el quad (-0.5 a 0.5)

// Atributos por instancia (datos de cada partícula)
layout(location = 1) in vec2 aOffset;    // Posición de la partícula
layout(location = 2) in float aSize;     // Tamaño de la partícula
layout(location = 3) in vec4 aColor;     // Color de la partícula

// Salida al fragment shader
out vec4 vColor;

// Uniforms
uniform mat4 uProjection;
uniform vec2 uCamera;

void main() {
    // Escalar y trasladar el quad
    vec2 scaledPos = aPosition * aSize;
    vec2 worldPos = scaledPos + aOffset - uCamera;
    
    // Transformar a coordenadas de clip
    gl_Position = uProjection * vec4(worldPos, 0.0, 1.0);
    
    // Pasar color al fragment shader
    vColor = aColor;
}
