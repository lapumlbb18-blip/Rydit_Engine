# 📚 Context - Shield Project

**Contenido:** Archivos de contexto y estado de sesión.

## Estructura

```
context/
├── current/         # Estado de sesión ACTUAL
│   └── SESSION_STATE.md  ← ÚNICO archivo activo
└── historical/      # Contextos de sesiones anteriores
    ├── CONTEXTO_PROXIMA_SESION_v0.0.10.txt
    └── ...
```

## ¿Qué hay en current/?

| Archivo | Descripción |
|---------|-------------|
| `SESSION_STATE.md` | Estado actual de la sesión (v0.1.1 - Sistema de Módulos) |

## ¿Qué hay en historical/?

- Contextos de sesiones anteriores
- Referencias para entender evolución del proyecto

## Flujo de Trabajo

1. **Al iniciar sesión:** Lee `context/current/SESSION_STATE.md`
2. **Al terminar sesión:** Actualiza `SESSION_STATE.md` con lo logrado
3. **Al cerrar versión:** Mueve el `SESSION_STATE.md` viejo a `historical/` y crea uno nuevo

---

**Importante:** Solo debe haber UN archivo activo en `current/`.

**Próxima versión:** v0.1.2 - Librería Estándar & Bindings Universales
