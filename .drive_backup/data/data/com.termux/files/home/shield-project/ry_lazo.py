#!/usr/bin/env python3
# ============================================================================
# ry_lazo.py - Puente universal entre Python y RyDit Engine
# ============================================================================
# Protocolo LAZOS v1.0 - JSON-RPC sobre stdin/stdout
# 
# Uso:
#   from ry_lazo import RyLazo
#   
#   ry = RyLazo()
#   punto = ry.call("science::bezier::cubic", [0, 0, 30, 100, 70, 100, 100, 0, 0.5])
#   print(punto)  # [50.0, 75.0]
#   ry.close()
# ============================================================================

import subprocess
import json
import sys

class RyLazo:
    """Clase principal para conectar Python con RyDit Engine vía Protocolo LAZOS"""
    
    def __init__(self, rydit_bin="./target/release/rydit-rs"):
        """
        Inicializar conexión con RyDit
        
        Args:
            rydit_bin: Ruta al binario de rydit-rs
        """
        self.rydit_bin = rydit_bin
        self.proc = None
        self._connect()
    
    def _connect(self):
        """Establecer conexión con rydit-rs en modo LAZOS"""
        try:
            self.proc = subprocess.Popen(
                [self.rydit_bin, "--lazos"],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                bufsize=1
            )
            
            # Esperar a que inicie
            import time
            time.sleep(0.1)
            
            # Verificar que está vivo
            if self.proc.poll() is not None:
                stderr = self.proc.stderr.read()
                raise RuntimeError(f"rydit-rs falló al iniciar: {stderr}")
                
        except FileNotFoundError:
            raise RuntimeError(f"No se encontró el binario: {self.rydit_bin}")
        except Exception as e:
            raise RuntimeError(f"Error al conectar con RyDit: {e}")
    
    def call(self, method, params=None):
        """
        Llamar a una función de RyDit
        
        Args:
            method: Nombre del método (ej: "science::bezier::cubic")
            params: Lista de parámetros
            
        Returns:
            Resultado de la función
            
        Raises:
            Exception: Si hay un error en RyDit
        """
        if params is None:
            params = []
        
        request = {
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        }
        
        try:
            # Enviar request
            self.proc.stdin.write(json.dumps(request) + "\n")
            self.proc.stdin.flush()
            
            # Recibir response
            response_line = self.proc.stdout.readline()
            if not response_line:
                raise RuntimeError("Conexión cerrada por RyDit")
            
            response = json.loads(response_line)
            
            # Verificar error
            if "error" in response:
                raise Exception(response["error"])
            
            return response.get("result")
            
        except Exception as e:
            raise RuntimeError(f"Error en llamada LAZOS: {e}")
    
    def ping(self):
        """Verificar conexión con RyDit"""
        try:
            result = self.call("system::ping")
            return result == "pong"
        except:
            return False
    
    def version(self):
        """Obtener versión de RyDit"""
        return self.call("system::version")
    
    def info(self):
        """Obtener información de RyDit"""
        return self.call("system::info")
    
    def close(self):
        """Cerrar conexión con RyDit"""
        if self.proc:
            self.proc.terminate()
            self.proc.wait()
            self.proc = None
    
    def __enter__(self):
        """Context manager entry"""
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit"""
        self.close()


# ============================================================================
# FUNCIONES DE ALTO NIVEL
# ============================================================================

def bezier_linear(p0, p1, t=0.5):
    """Curva de Bezier lineal entre 2 puntos"""
    with RyLazo() as ry:
        return ry.call("science::bezier::linear", [p0[0], p0[1], p1[0], p1[1], t])

def bezier_quadratic(p0, p1, p2, t=0.5):
    """Curva de Bezier cuadrática con 1 punto de control"""
    with RyLazo() as ry:
        return ry.call("science::bezier::quadratic", 
                      [p0[0], p0[1], p1[0], p1[1], p2[0], p2[1], t])

def bezier_cubic(p0, p1, p2, p3, t=0.5):
    """Curva de Bezier cúbica con 2 puntos de control"""
    with RyLazo() as ry:
        return ry.call("science::bezier::cubic",
                      [p0[0], p0[1], p1[0], p1[1], p2[0], p2[1], p3[0], p3[1], t])

def bezier_generate_points(control_points, steps=10):
    """Generar múltiples puntos en una curva de Bezier"""
    with RyLazo() as ry:
        return ry.call("science::bezier::generate_points", [control_points, steps])

def projectile(x0, y0, v0, angle):
    """Calcular trayectoria de proyectil"""
    with RyLazo() as ry:
        return ry.call("physics::projectile", [x0, y0, v0, angle])

def nbody_2(m1, m2, x1, y1, x2, y2, G=6.674e-11):
    """Calcular fuerza gravitacional entre 2 cuerpos"""
    with RyLazo() as ry:
        return ry.call("physics::nbody_2", [m1, m2, x1, y1, x2, y2, G])

def mean(data):
    """Calcular media aritmética"""
    with RyLazo() as ry:
        return ry.call("stats::mean", [data])

def median(data):
    """Calcular mediana"""
    with RyLazo() as ry:
        return ry.call("stats::median", [data])

def min(data):
    """Obtener valor mínimo"""
    with RyLazo() as ry:
        return ry.call("stats::min", [data])

def max(data):
    """Obtener valor máximo"""
    with RyLazo() as ry:
        return ry.call("stats::max", [data])


# ============================================================================
# DEMO
# ============================================================================

if __name__ == "__main__":
    print("=" * 60)
    print("🔗 PROTOCOLO LAZOS - Demo desde Python")
    print("=" * 60)
    print()
    
    # Crear conexión
    ry = RyLazo()
    
    try:
        # Verificar conexión
        print("1. Verificando conexión...")
        if ry.ping():
            print("   ✅ Conexión exitosa")
        else:
            print("   ❌ Error de conexión")
            sys.exit(1)
        
        # Obtener versión
        print("\n2. Versión de RyDit:")
        print(f"   {ry.version()}")
        
        # Obtener información
        print("\n3. Información de RyDit:")
        info = ry.info()
        print(f"   Nombre: {info['name']}")
        print(f"   Versión: {info['version']}")
        print(f"   Protocolo: {info['protocol']}")
        print(f"   Comandos disponibles: {len(info['commands'])}")
        
        # Bezier cúbica
        print("\n4. Curva de Bezier Cúbica:")
        print("   P0=(0,0), P1=(30,100), P2=(70,100), P3=(100,0)")
        print("   Generando 10 puntos:")
        
        for i in range(10):
            t = i / 9.0
            punto = ry.call("science::bezier::cubic", 
                           [0, 0, 30, 100, 70, 100, 100, 0, t])
            print(f"   t={t:.2f}: {punto}")
        
        # Física: Proyectil
        print("\n5. Física - Proyectil:")
        print("   v0=50 m/s, angle=45°")
        trayectoria = ry.call("physics::projectile", [0, 0, 50, 45])
        print(f"   Alcance: {trayectoria[4]:.2f} m")
        print(f"   Altura máxima: {trayectoria[3]:.2f} m")
        print(f"   Tiempo de vuelo: {trayectoria[2]:.2f} s")
        
        # Estadísticas
        print("\n6. Estadísticas:")
        datos = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        print(f"   Datos: {datos}")
        print(f"   Media: {ry.call('stats::mean', [datos])}")
        print(f"   Mediana: {ry.call('stats::median', [datos])}")
        print(f"   Mínimo: {ry.call('stats::min', [datos])}")
        print(f"   Máximo: {ry.call('stats::max', [datos])}")
        
        print("\n" + "=" * 60)
        print("✅ Demo completada exitosamente!")
        print("=" * 60)
        
    except Exception as e:
        print(f"\n❌ Error: {e}")
        sys.exit(1)
    finally:
        ry.close()
