// Vertex Shader GPU Instancing - RyDit v0.15.0 FIX
// Coordenadas de pantalla directas (0-w, 0-h) — compatible con Termux-X11

#version 330 core

layout(location = 0) in vec2 aPosition;  // Quad local (-0.5 a 0.5)
layout(location = 1) in vec2 aOffset;    // Posición partícula (coordenadas de pantalla)
layout(location = 2) in float aSize;
layout(location = 3) in vec4 aColor;

out vec4 vColor;
out vec2 vLocalPos;

uniform vec2 uResolution; // Ancho y alto de pantalla

void main() {
    // Escalar quad local
    vec2 localQuad = aPosition * aSize;

    // Posición final en coordenadas de pantalla
    vec2 screenPos = localQuad + aOffset;

    // Convertir a NDC (-1 a 1)
    vec2 ndc = (screenPos / uResolution) * 2.0 - 1.0;

    gl_Position = vec4(ndc.x, -ndc.y, 0.0, 1.0);

    vColor = aColor;
    vLocalPos = aPosition;
}
