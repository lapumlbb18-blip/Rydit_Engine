// crates/rydit-gfx/shaders/fsr_upscale.glsl
// 🆕 FSR 1.0 - EASU (Edge Adaptive Spatial Upsampling) Simplificado
// v0.11.4 - Versión 2D simple para Termux-X11/Android

#version 330 core

// Input: textura en baja resolución
uniform sampler2D inputTexture;
uniform vec2 inputSize;      // Ej: 1280x720
uniform vec2 outputSize;     // Ej: 1920x1080

// Output: color upscaled
out vec4 fragColor;

// EASU simplificado: bilinear + edge detection básico
void main() {
    // UV coordinates en output resolution
    vec2 uv = gl_FragCoord.xy / outputSize;
    
    // Calcular tamaño de texel en input
    vec2 texelSize = 1.0 / inputSize;
    
    // Coordenadas en espacio de input texture
    vec2 inputUV = uv;
    
    // Samplear 4 texels vecinos para interpolación bilinear
    vec2 texCoord = inputUV * inputSize;
    vec2 baseTexel = floor(texCoord);
    vec2 frac = fract(texCoord);
    
    // 4 corners del texel
    vec4 tl = texture(inputTexture, (baseTexel + vec2(0.0, 0.0)) * texelSize);
    vec4 tr = texture(inputTexture, (baseTexel + vec2(1.0, 0.0)) * texelSize);
    vec4 bl = texture(inputTexture, (baseTexel + vec2(0.0, 1.0)) * texelSize);
    vec4 br = texture(inputTexture, (baseTexel + vec2(1.0, 1.0)) * texelSize);
    
    // Interpolación bilinear
    vec4 top = mix(tl, tr, frac.x);
    vec4 bottom = mix(bl, br, frac.x);
    vec4 color = mix(top, bottom, frac.y);
    
    // Edge detection simple (sobreamuestreo en bordes)
    float edgeStrength = 0.0;
    
    // Detectar bordes horizontales y verticales
    vec4 left = texture(inputTexture, (texCoord - vec2(1.0, 0.0)) * texelSize);
    vec4 right = texture(inputTexture, (texCoord + vec2(1.0, 0.0)) * texelSize);
    vec4 up = texture(inputTexture, (texCoord - vec2(0.0, 1.0)) * texelSize);
    vec4 down = texture(inputTexture, (texCoord + vec2(0.0, 1.0)) * texelSize);
    
    // Gradiente horizontal y vertical
    float gradH = abs(dot(right - left, vec4(0.25)));
    float gradV = abs(dot(up - down, vec4(0.25)));
    
    // Si hay borde, reducir blur
    edgeStrength = max(gradH, gradV);
    
    // Aplicar edge-adaptive: menos blur en bordes
    if (edgeStrength > 0.1) {
        // Usar nearest neighbor en bordes para preservar nitidez
        vec2 nearestTexel = (baseTexel + step(frac, vec2(0.5))) * texelSize;
        color = texture(inputTexture, nearestTexel);
    }
    
    fragColor = color;
}
