#!/usr/bin/env python3
# ============================================================================
# test_lazos.py - Tests del Protocolo LAZOS
# ============================================================================

import sys
import json

# Importar RyLazo
try:
    from ry_lazo import RyLazo, bezier_cubic, projectile, mean, median
except ImportError as e:
    print(f"❌ Error importando ry_lazo: {e}")
    sys.exit(1)


def test_system_ping():
    """Test 1: System ping"""
    print("Test 1: system::ping... ", end="")
    with RyLazo() as ry:
        result = ry.call("system::ping")
        assert result == "pong", f"Expected 'pong', got {result}"
    print("✅")


def test_system_version():
    """Test 2: System version"""
    print("Test 2: system::version... ", end="")
    with RyLazo() as ry:
        result = ry.call("system::version")
        assert "v0.7" in result, f"Expected v0.7.x, got {result}"
    print("✅")


def test_bezier_linear():
    """Test 3: Bezier lineal - punto medio"""
    print("Test 3: bezier::linear (punto medio)... ", end="")
    with RyLazo() as ry:
        result = ry.call("science::bezier::linear", [0, 0, 100, 100, 0.5])
        assert abs(result[0] - 50.0) < 0.01, f"Expected x=50, got {result[0]}"
        assert abs(result[1] - 50.0) < 0.01, f"Expected y=50, got {result[1]}"
    print("✅")


def test_bezier_cubic():
    """Test 4: Bezier cúbica - t=0.5"""
    print("Test 4: bezier::cubic (t=0.5)... ", end="")
    with RyLazo() as ry:
        result = ry.call("science::bezier::cubic", 
                        [0, 0, 30, 100, 70, 100, 100, 0, 0.5])
        assert abs(result[0] - 50.0) < 0.01, f"Expected x=50, got {result[0]}"
        assert abs(result[1] - 75.0) < 0.01, f"Expected y=75, got {result[1]}"
    print("✅")


def test_bezier_start_end():
    """Test 5: Bezier cúbica - inicio y fin"""
    print("Test 5: bezier::cubic (t=0 y t=1)... ", end="")
    with RyLazo() as ry:
        start = ry.call("science::bezier::cubic", 
                       [0, 0, 30, 100, 70, 100, 100, 0, 0.0])
        end = ry.call("science::bezier::cubic", 
                     [0, 0, 30, 100, 70, 100, 100, 0, 1.0])
        
        assert abs(start[0] - 0.0) < 0.01, f"Expected start x=0, got {start[0]}"
        assert abs(start[1] - 0.0) < 0.01, f"Expected start y=0, got {start[1]}"
        assert abs(end[0] - 100.0) < 0.01, f"Expected end x=100, got {end[0]}"
        assert abs(end[1] - 0.0) < 0.01, f"Expected end y=0, got {end[1]}"
    print("✅")


def test_physics_projectile():
    """Test 6: Física - Proyectil"""
    print("Test 6: physics::projectile... ", end="")
    with RyLazo() as ry:
        result = ry.call("physics::projectile", [0, 0, 50, 45])
        
        # Alcance ~254.84m
        assert abs(result[4] - 254.84) < 1.0, f"Expected range ~254.84, got {result[4]}"
        
        # Altura máxima ~63.71m
        assert abs(result[3] - 63.71) < 1.0, f"Expected height ~63.71, got {result[3]}"
        
        # Tiempo de vuelo ~7.21s
        assert abs(result[2] - 7.21) < 0.1, f"Expected time ~7.21, got {result[2]}"
    print("✅")


def test_physics_nbody():
    """Test 7: Física - Gravedad (2 cuerpos)"""
    print("Test 7: physics::nbody_2... ", end="")
    with RyLazo() as ry:
        result = ry.call("physics::nbody_2", 
                        [1000, 500, 0, 0, 10, 0, 6.674e-11])
        
        # Distancia = 10
        assert abs(result[4] - 10.0) < 0.01, f"Expected dist=10, got {result[4]}"
        
        # Fuerza debería ser positiva (atracción)
        assert result[0] > 0, f"Expected positive force, got {result[0]}"
    print("✅")


def test_stats_mean():
    """Test 8: Estadísticas - Media"""
    print("Test 8: stats::mean... ", end="")
    with RyLazo() as ry:
        result = ry.call("stats::mean", [[1, 2, 3, 4, 5]])
        assert abs(result - 3.0) < 0.01, f"Expected mean=3.0, got {result}"
    print("✅")


def test_stats_median_odd():
    """Test 9: Estadísticas - Mediana (impar)"""
    print("Test 9: stats::median (impar)... ", end="")
    with RyLazo() as ry:
        result = ry.call("stats::median", [[1, 2, 3, 4, 5]])
        assert abs(result - 3.0) < 0.01, f"Expected median=3.0, got {result}"
    print("✅")


def test_stats_median_even():
    """Test 10: Estadísticas - Mediana (par)"""
    print("Test 10: stats::median (par)... ", end="")
    with RyLazo() as ry:
        result = ry.call("stats::median", [[1, 2, 3, 4]])
        assert abs(result - 2.5) < 0.01, f"Expected median=2.5, got {result}"
    print("✅")


def test_stats_min_max():
    """Test 11: Estadísticas - Min/Max"""
    print("Test 11: stats::min/max... ", end="")
    with RyLazo() as ry:
        data = [5, 2, 8, 1, 9]
        min_val = ry.call("stats::min", [data])
        max_val = ry.call("stats::max", [data])
        
        assert abs(min_val - 1.0) < 0.01, f"Expected min=1, got {min_val}"
        assert abs(max_val - 9.0) < 0.01, f"Expected max=9, got {max_val}"
    print("✅")


def test_high_level_functions():
    """Test 12: Funciones de alto nivel"""
    print("Test 12: High-level functions... ", end="")
    
    # Bezier cúbica
    punto = bezier_cubic([0, 0], [30, 100], [70, 100], [100, 0], 0.5)
    assert abs(punto[0] - 50.0) < 0.01
    
    # Proyectil
    trayectoria = projectile(0, 0, 50, 45)
    assert abs(trayectoria[4] - 254.84) < 1.0
    
    # Media
    media = mean([1, 2, 3, 4, 5])
    assert abs(media - 3.0) < 0.01
    
    # Mediana
    mediana = median([1, 2, 3, 4])
    assert abs(mediana - 2.5) < 0.01
    
    print("✅")


def test_error_handling():
    """Test 13: Manejo de errores"""
    print("Test 13: Error handling... ", end="")
    with RyLazo() as ry:
        # Método inexistente retorna error en el response
        result = ry.call("nonexistent::method", [])
        # Verificar que sea un dict con error
        assert isinstance(result, dict), f"Expected dict, got {type(result)}"
        assert "error" in result, f"Expected 'error' key, got {result}"
        assert "Unknown method" in result["error"], f"Expected 'Unknown method', got {result['error']}"
    print("✅")


def run_all_tests():
    """Ejecutar todos los tests"""
    print("=" * 60)
    print("🧪 TESTS DEL PROTOCOLO LAZOS v1.0")
    print("=" * 60)
    print()
    
    tests = [
        test_system_ping,
        test_system_version,
        test_bezier_linear,
        test_bezier_cubic,
        test_bezier_start_end,
        test_physics_projectile,
        test_physics_nbody,
        test_stats_mean,
        test_stats_median_odd,
        test_stats_median_even,
        test_stats_min_max,
        test_high_level_functions,
        test_error_handling,
    ]
    
    passed = 0
    failed = 0
    
    for test in tests:
        try:
            test()
            passed += 1
        except AssertionError as e:
            print(f"❌ FALLÓ: {e}")
            failed += 1
        except Exception as e:
            print(f"❌ ERROR: {e}")
            failed += 1
    
    print()
    print("=" * 60)
    print(f"RESULTADOS: {passed}/{len(tests)} tests passing")
    
    if failed > 0:
        print(f"⚠️  {failed} tests fallaron")
        sys.exit(1)
    else:
        print("✅ ¡TODOS LOS TESTS EXITOSOS!")
        print("=" * 60)
        sys.exit(0)


if __name__ == "__main__":
    run_all_tests()
