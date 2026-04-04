// crates/rydit-gfx/shaders/fsr_sharpen.glsl
// 🆕 FSR 1.0 - RCAS (Robust Contrast Adaptive Sharpening) Simplificado
// v0.11.4 - Versión 2D simple para Termux-X11/Android

#version 330 core

// Input: textura upscaled
uniform sampler2D inputTexture;
uniform vec2 texelSize;      // 1.0 / outputSize
uniform float sharpness;     // 0.0 - 1.0 (0.5 recomendado)

// Output: color sharpened
out vec4 fragColor;

// RCAS simplificado: sharpen adaptativo basado en contraste
void main() {
    vec2 uv = gl_FragCoord.xy * texelSize;
    
    // Sample center + 4 neighbors (cruz)
    vec4 center = texture(inputTexture, uv);
    vec4 up    = texture(inputTexture, uv + vec2(0.0, -texelSize.y));
    vec4 down  = texture(inputTexture, uv + vec2(0.0, texelSize.y));
    vec4 left  = texture(inputTexture, uv + vec2(-texelSize.x, 0.0));
    vec4 right = texture(inputTexture, uv + vec2(texelSize.x, 0.0));
    
    // Calcular contraste local (varianza)
    vec4 avg = (up + down + left + right) * 0.25;
    float contrast = length(center - avg);
    
    // Adaptive sharpen: menos sharpen en áreas de bajo contraste (ruido)
    float adaptiveSharpness = sharpness;
    
    if (contrast < 0.05) {
        // Bajo contraste (posible ruido) - reducir sharpen
        adaptiveSharpness = sharpness * 0.3;
    } else if (contrast > 0.3) {
        // Alto contraste (borde definido) - sharpen completo
        adaptiveSharpness = sharpness;
    }
    
    // Sharpen filter: center + (center - avg) * sharpness
    vec4 sharpened = center + (center - avg) * adaptiveSharpness;
    
    // Clamp para evitar overshoot
    sharpened = clamp(sharpened, 0.0, 1.0);
    
    fragColor = sharpened;
}
