//! Benchmarks para lizer (lexer + parser)
//!
//! Ejecutar con: cargo bench --bench bench_lizer
//!
//! Nota: Requiere nightly Rust para #[feature(test)]

#![feature(test)]
extern crate test;

use lizer::Lizer;
use test::Bencher;

// ============================================================================
// BENCHMARKS DE LEXER
// ============================================================================

#[bench]
fn bench_lexing_simple(b: &mut Bencher) {
    // Lexer para código simple (10 tokens)
    b.iter(|| {
        let code = "shield.init dark.slot x = 100 voz x";
        let tokens = Lizer::new(code).scan();
        assert!(!tokens.is_empty());
    });
}

#[bench]
fn bench_lexing_medium(b: &mut Bencher) {
    // Lexer para código mediano (~50 tokens)
    b.iter(|| {
        let code = r#"
            shield.init
            dark.slot nombre = "player"
            dark.slot vida = 100
            dark.slot posicion = [0, 0, 0]
            
            si vida > 0 {
                voz "vivo"
            } si_no {
                voz "muerto"
            }
        "#;
        let tokens = Lizer::new(code).scan();
        assert!(!tokens.is_empty());
    });
}

#[bench]
fn bench_lexing_complex(b: &mut Bencher) {
    // Lexer para código complejo (~200 tokens)
    b.iter(|| {
        let code = r#"
            shield.init
            
            # Función de movimiento
            funcion mover(x, y, z) {
                dark.slot posicion = [x, y, z]
                retornar posicion
            }
            
            # Game loop
            cada frame en frames {
                dark.slot delta = time::delta()
                dark.slot input = input::get()
                
                si input.arriba {
                    mover(0, 1, 0)
                }
                
                math::sin(frame)
                math::cos(frame)
                random::int(0, 100)
            }
        "#;
        let tokens = Lizer::new(code).scan();
        assert!(!tokens.is_empty());
    });
}

#[bench]
fn bench_lexing_strings(b: &mut Bencher) {
    // Lexer con muchos strings
    b.iter(|| {
        let code = r#"
            dark.slot dialogo = [
                "Hola, viajero",
                "Bienvenido a RyDit",
                "¿Qué te trae por aquí?",
                "Buena suerte en tu aventura"
            ]
        "#;
        let tokens = Lizer::new(code).scan();
        assert!(!tokens.is_empty());
    });
}

#[bench]
fn bench_lexing_symbols(b: &mut Bencher) {
    // Lexer con símbolos en identificadores
    b.iter(|| {
        let code = r#"
            dark.slot player@position = [0, 0]
            dark.slot enemy$health = 100
            dark.slot item%value = 50
            dark.slot flag&active = verdadero
        "#;
        let tokens = Lizer::new(code).scan();
        assert!(!tokens.is_empty());
    });
}

// ============================================================================
// BENCHMARKS DE PARSER
// ============================================================================

#[bench]
fn bench_parsing_simple(b: &mut Bencher) {
    // Parser para código simple
    b.iter(|| {
        let code = "dark.slot x = 100";
        let tokens = Lizer::new(code).scan();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_ok());
    });
}

#[bench]
fn bench_parsing_expressions(b: &mut Bencher) {
    // Parser para expresiones aritméticas
    b.iter(|| {
        let code = "dark.slot resultado = (10 + 20) * 3 - 5 / 2";
        let tokens = Lizer::new(code).scan();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_ok());
    });
}

#[bench]
fn bench_parsing_arrays(b: &mut Bencher) {
    // Parser para arrays
    b.iter(|| {
        let code = "dark.slot lista = [1, 2, 3, 4, 5]";
        let tokens = Lizer::new(code).scan();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_ok());
    });
}

#[bench]
fn bench_parsing_nested_arrays(b: &mut Bencher) {
    // Parser para arrays anidados
    b.iter(|| {
        let code = "dark.slot matriz = [[0, 0], [0, 0], [0, 0]]";
        let tokens = Lizer::new(code).scan();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_ok());
    });
}

#[bench]
fn bench_parsing_conditionals(b: &mut Bencher) {
    // Parser para condicionales
    b.iter(|| {
        let code = r#"
            si vida > 0 {
                voz "vivo"
            } si_no si escudo > 0 {
                voz "protegido"
            } si_no {
                voz "muerto"
            }
        "#;
        let tokens = Lizer::new(code).scan();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_ok());
    });
}

#[bench]
fn bench_parsing_functions(b: &mut Bencher) {
    // Parser para funciones
    b.iter(|| {
        let code = r#"
            funcion calcular_distancia(x1, y1, x2, y2) {
                dark.slot dx = x2 - x1
                dark.slot dy = y2 - y1
                retornar math::sqrt(dx * dx + dy * dy)
            }
        "#;
        let tokens = Lizer::new(code).scan();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_ok());
    });
}

#[bench]
fn bench_parsing_loops(b: &mut Bencher) {
    // Parser para loops
    b.iter(|| {
        let code = r#"
            cada elemento en lista {
                voz elemento
            }
            
            mientras contador < 10 {
                dark.slot contador = contador + 1
            }
        "#;
        let tokens = Lizer::new(code).scan();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_ok());
    });
}

#[bench]
fn bench_parsing_complete_script(b: &mut Bencher) {
    // Parser para script completo (~500 tokens)
    b.iter(|| {
        let code = r#"
            shield.init
            
            # Variables del jugador
            dark.slot player = {
                nombre: "hero",
                vida: 100,
                posicion: [0, 0, 0]
            }
            
            # Función de actualización
            funcion update(delta) {
                dark.slot nueva_pos = [
                    player.posicion[0] + delta,
                    player.posicion[1],
                    player.posicion[2]
                ]
                player.posicion = nueva_pos
            }
            
            # Game loop
            cada frame en frames {
                dark.slot delta = time::delta()
                update(delta)
                
                si player.vida <= 0 {
                    voz "Game Over"
                    romper
                }
                
                math::sin(frame * 0.1)
                math::cos(frame * 0.1)
            }
        "#;
        let tokens = Lizer::new(code).scan();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_ok());
    });
}

// ============================================================================
// BENCHMARKS COMBINADOS (LEXER + PARSER)
// ============================================================================

#[bench]
fn bench_compile_small(b: &mut Bencher) {
    // Compilación completa de script pequeño
    b.iter(|| {
        let code = "dark.slot x = 100 voz x";
        let tokens = Lizer::new(code).scan();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_ok());
    });
}

#[bench]
fn bench_compile_medium(b: &mut Bencher) {
    // Compilación completa de script mediano
    b.iter(|| {
        let code = r#"
            shield.init
            dark.slot x = 10
            dark.slot y = 20
            
            funcion sumar(a, b) {
                retornar a + b
            }
            
            voz sumar(x, y)
        "#;
        let tokens = Lizer::new(code).scan();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_ok());
    });
}

#[bench]
fn bench_compile_large(b: &mut Bencher) {
    // Compilación completa de script grande
    b.iter(|| {
        let code = r#"
            shield.init
            
            # Constants
            dark.slot GRAVEDAD = 9.8
            dark.slot FPS = 60
            
            # Player struct
            dark.slot player = {
                x: 0,
                y: 0,
                vx: 0,
                vy: 0,
                en_suelo: falso
            }
            
            # Funciones
            funcion aplicar_gravedad() {
                player.vy = player.vy - GRAVEDAD * time::delta()
            }
            
            funcion mover(dx, dy) {
                player.x = player.x + dx
                player.y = player.y + dy
            }
            
            funcion saltar() {
                si player.en_suelo {
                    player.vy = 10
                    player.en_suelo = falso
                }
            }
            
            # Game loop
            cada frame en frames {
                aplicar_gravedad()
                
                si input::tecla_presionada("SPACE") {
                    saltar()
                }
                
                mover(player.vx, player.vy)
                
                # Collision detection
                si player.y < 0 {
                    player.y = 0
                    player.en_suelo = verdadero
                    player.vy = 0
                }
                
                # Render
                draw::circle(player.x, player.y, 10, "rojo")
            }
        "#;
        let tokens = Lizer::new(code).scan();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_ok());
    });
}
