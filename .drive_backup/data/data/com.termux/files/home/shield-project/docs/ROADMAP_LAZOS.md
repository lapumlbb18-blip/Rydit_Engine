# 🔗 PROTOCOLO LAZOS - Roadmap Completo

**Versión objetivo**: v0.7.2.0
**Estado**: EN DESARROLLO
**Filosofía**: Simple sobre complejo, Funcional sobre perfecto

---

## 🎯 VISIÓN

### **El Problema que Resolvemos:**

Los motores actuales:
- ❌ Requieren IDEs pesados
- ❌ Solo funcionan en PC potente
- ❌ Lenguajes en inglés
- ❌ Binarios de 50+ MB
- ❌ No se conectan con otros lenguajes fácilmente

### **La Solución LAZOS:**

- ✅ Android nativo (Termux)
- ✅ Sin IDE (cargo run)
- ✅ Español nativo
- ✅ 700 KB
- ✅ **Conecta TODO** (Python, web, git, etc.)

---

## 🏗️ ARQUITECTURA

### **Niveles del Ecosistema:**

```
┌─────────────────────────────────────────┐
│  NIVEL 4: ECOSISTEMA                    │
│  (Web, Git, Python, IA/ML)              │
├─────────────────────────────────────────┤
│  NIVEL 3: PLATAFORMA                    │
│  (Linux, Windows, Android, WebAssembly) │
├─────────────────────────────────────────┤
│  NIVEL 2: CRATES                        │
│  (science, physics, anim, data)         │
├─────────────────────────────────────────┤
│  NIVEL 1: RYDIT-RS                      │
│  (Binario Principal)                    │
├─────────────────────────────────────────┤
│  NIVEL 0: LAZOS                         │
│  (Protocolo Universal - CORAZÓN)        │
└─────────────────────────────────────────┘
```

### **Componentes de RyDit-RS:**

```
📦 RYDIT-RS (Binario Principal)
│
├── 🧠 RY CORE
│   └── Protocolo LAZOS (JSON-RPC, stdin/stdout)
│
├── 🌍 MUNDOS
│   ├── mundo.nuevo()
│   ├── mundo.actual(id)
│   └── mundo.transicion(a, b)
│
├── 📦 CONTENEDORES
│   ├── contenedor.nuevo()
│   ├── contenedor.posicion(x, y)
│   ├── contenedor.rotacion(angle)
│   ├── contenedor.escala(sx, sy)
│   └── contenedor.agregar(hijo)
│
├── 🎭 ACTORES
│   ├── actor.nuevo(sprite)
│   ├── actor.mover(x, y)
│   ├── actor.rotar(angle)
│   └── actor.escalar(factor)
│
├── 🔧 HERRAMIENTAS
│   ├── debug.fps()
│   ├── debug.memoria()
│   ├── time.delta()
│   └── random.rango(min, max)
│
└── 🔗 LAZOS
    ├── lazos.python(script)
    ├── lazos.http(url)
    ├── lazos.git(repo)
    └── lazos.file(path)
```

---

## 📋 ROADMAP DETALLADO

### **v0.7.2.0 - Protocolo LAZOS Básico** (1 semana)

#### **Día 1-2: Protocolo JSON-RPC**

**Tareas:**
- [ ] Definir formato JSON-RPC
- [ ] Implementar parser JSON
- [ ] Loop stdin/stdout
- [ ] Manejo de errores básico

**Formato:**
```json
// Request
{
    "jsonrpc": "2.0",
    "method": "science::bezier::cubic",
    "params": [0, 0, 30, 100, 70, 100, 100, 0, 0.5],
    "id": 1
}

// Response
{
    "jsonrpc": "2.0",
    "result": [50, 75],
    "id": 1
}
```

**Código Rust:**
```rust
// crates/rydit-rs/src/lazos/protocol.rs

pub struct Request {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<Valor>,
    pub id: u32,
}

pub struct Response {
    pub jsonrpc: String,
    pub result: Valor,
    pub id: u32,
}

pub fn manejar_request(req: Request) -> Response {
    match req.method.as_str() {
        "science::bezier::cubic" => {
            let result = bezier_cubic(&req.params);
            Response { result, id: req.id, .. }
        }
        _ => Response::error("Unknown method"),
    }
}
```

---

#### **Día 3-4: Contenedores**

**Tareas:**
- [ ] Struct `Contenedor`
- [ ] Transform: posición, rotación, escala
- [ ] Jerarquía padre-hijo
- [ ] API RyDit-style

**Código RyDit:**
```rydit
# Crear contenedor
dark.slot jugador = contenedor.nuevo()

# Transform
jugador.posicion(400, 300)
jugador.rotacion(45)
jugador.escala(2, 2)

# Jerarquía
dark.slot arma = contenedor.nuevo()
jugador.agregar(arma)  # arma es hijo de jugador

# Obtener transform mundial
dark.slot pos = jugador.posicion_mundial()
```

**Código Rust:**
```rust
// crates/rydit-rs/src/contenedores.rs

pub struct Contenedor {
    pub id: String,
    pub transform: Transform,
    pub children: Vec<String>,
    pub parent: Option<String>,
}

pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Contenedor {
    pub fn nuevo(id: &str) -> Self {
        Self {
            id: id.to_string(),
            transform: Transform::default(),
            children: vec![],
            parent: None,
        }
    }
}
```

---

#### **Día 5: Mundos**

**Tareas:**
- [ ] Struct `Mundo`
- [ ] Gestión de contenedores
- [ ] Activar/desactivar
- [ ] Transiciones

**Código RyDit:**
```rydit
# Crear mundos
dark.slot menu = mundo.nuevo("menu")
dark.slot juego = mundo.nuevo("juego")

# Agregar contenedores
juego.agregar(jugador)
juego.agregar(enemigos)

# Activar mundo
mundo.activar("juego")

# Transición
mundo.transicion("menu", "juego", "fade")
```

---

#### **Día 6: Python Bridge**

**Tareas:**
- [ ] ry_lazo.py (puente)
- [ ] Ejemplos de uso
- [ ] Tests de integración

**Código Python:**
```python
# ry_lazo.py
import json
import subprocess

class RyLazo:
    def __init__(self, rydit_bin="rydit-rs"):
        self.proc = subprocess.Popen(
            [rydit_bin, "--lazos"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            text=True
        )
    
    def call(self, method, params=[]):
        request = {
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        }
        self.proc.stdin.write(json.dumps(request) + "\n")
        self.proc.stdin.flush()
        response = json.loads(self.proc.stdout.readline())
        return response["result"]
    
    def close(self):
        self.proc.terminate()

# Ejemplo de uso
ry = RyLazo()

# Usar Bezier desde Python
punto = ry.call("science::bezier::cubic", 
    [0, 0, 30, 100, 70, 100, 100, 0, 0.5])
print(punto)  # [50, 75]

# Usar física
trayectoria = ry.call("physics::projectile", [0, 0, 50, 45])
print(trayectoria)

ry.close()
```

---

#### **Día 7: Tests + Demo**

**Tareas:**
- [ ] Tests de protocolo
- [ ] Tests de contenedores
- [ ] Demo completa
- [ ] Documentación

**Demo:**
```rydit
# demo_lazos_v0.7.2.0.rydit

# Crear mundo
dark.slot mundo = mundo.nuevo("demo")

# Jugador con Bezier
dark.slot jugador = contenedor.nuevo()
jugador.posicion(400, 300)

# Trayectoria con Bezier
dark.slot ctrl = [[0, 0], [30, 100], [70, 100], [100, 0]]
dark.slot pts = bezier::generate_points(ctrl, 20)

# Mover jugador a lo largo de la curva
dark.slot i = 0
mientras i < pts.len() {
    dark.slot p = pts[i]
    jugador.posicion(p[0] * 4, p[1] * 3)
    i = i + 1
}

# Conectar con Python
dark.slot py = lazos.python("script.py")
py.enviar({"accion": "calcular", "datos": [1, 2, 3]})
dark.slot resp = py.recibir()

print("Demo LAZOS completada!")
```

---

### **v0.7.3.0 - Gráficos Avanzados** (1 semana)

- [ ] draw.bezier(ctrl_points, color, width)
- [ ] draw.path(points, color, filled)
- [ ] Sprite animations con Bezier
- [ ] Easing functions (ease_in, ease_out)

---

### **v0.8.0.0 - Ecosistema Ry** (2 semanas)

#### **rydit-linux:**
- [ ] Build para x86_64-unknown-linux-gnu
- [ ] .deb package
- [ ] .rpm package

#### **rydit-windows:**
- [ ] Build para x86_64-pc-windows-msvc
- [ ] .exe installer
- [ ] GitHub Actions workflow

#### **ry-py (Python bridge):**
- [ ] Publicar en PyPI
- [ ] Documentación completa
- [ ] Ejemplos de uso

---

### **v0.9.0.0 - Expansión** (2 semanas)

#### **ry-web (WebAssembly):**
- [ ] Compilar a wasm
- [ ] Publicar en npm
- [ ] Demo en navegador

#### **HTTP/WebSocket nativo:**
- [ ] lazos.http() mejorado
- [ ] lazos.websocket()
- [ ] API REST desde RyDit

#### **Git integration:**
- [ ] lazos.git.clone()
- [ ] lazos.git.pull()
- [ ] Control de versiones desde RyDit

---

### **v1.0.0 - Release Estable** (1 semana)

- [ ] API estable (semver)
- [ ] 10+ demos reales
- [ ] Documentación completa
- [ ] Tutoriales YouTube
- [ ] Paper académico

---

## 📊 COMPARATIVA: LAZOS vs RyditModule

| Criterio | RyditModule | LAZOS |
|----------|-------------|-------|
| **Complejidad** | Alta (trait, generics) | Baja (JSON, CLI) |
| **Binario** | ❌ Requiere lib+bin | ✅ Funciona con binario |
| **Lenguajes** | Solo Rust | Cualquiera (Python, C, etc.) |
| **Plataformas** | Solo Rust targets | Cualquiera con JSON |
| **Acoplamiento** | Alto (compile-time) | Bajo (runtime) |
| **Performance** | Alta | Media (aceptable) |
| **Type safety** | Alta | Baja (mejorable) |
| **Flexibilidad** | Media | Alta |
| **Nuestro** | ❌ Copia estándar | ✅ RyDit-style |

**Ganador**: **LAZOS** ✅

---

## 🎯 MÉTRICAS DE ÉXITO

### **v0.7.2.0 (1 semana):**
- ✅ Protocolo LAZOS funcional
- ✅ 10+ comandos implementados
- ✅ Python bridge working
- ✅ Demo completa
- ✅ 5+ tests de LAZOS

### **v0.8.0.0 (1 mes):**
- ✅ Linux build funcional
- ✅ Windows build funcional
- ✅ ry-py en PyPI
- ✅ 100+ estrellas en GitHub

### **v1.0.0 (2 meses):**
- ✅ 1000+ descargas
- ✅ 10+ tutoriales YouTube
- ✅ 1 paper académico
- ✅ Comunidad activa en Discord

---

## 💬 FILOSOFÍA LAZOS

> "Simple sobre complejo. Funcional sobre perfecto. Nuestro sobre copiado."

**No somos Godot. No somos Unity. Somos RyDit.**

- ❌ No copiamos terminología (Nodes, Scenes)
- ✅ Creamos la nuestra (Contenedores, Mundos)
- ❌ No seguimos patrones corporativos
- ✅ Seguimos nuestra visión

**LAZOS no es solo un protocolo... es nuestra filosofía.**

---

<div align="center">

**🔗 PROTOCOLO LAZOS - El Corazón del Ecosistema Ry**

*Próxima parada: v0.7.2.0 - 1 semana*

**Universal. Simple. Nuestro.**

</div>
