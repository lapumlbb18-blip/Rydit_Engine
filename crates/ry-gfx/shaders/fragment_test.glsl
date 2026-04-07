// Fragment shader TEST: sin círculo — solo quad sólido
#version 330 core

in vec4 vColor;
in vec2 vLocalPos;

out vec4 fragColor;

void main() {
    // CUAD SÓLIDO — sin discard, sin distancia
    fragColor = vec4(1.0, 1.0, 1.0, 1.0); // BLANCO PURO
}
