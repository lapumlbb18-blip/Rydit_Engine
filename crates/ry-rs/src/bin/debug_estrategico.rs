// crates/rydit-rs/src/bin/debug_estrategico.rs
// 🐛 DEBUG ESTRATÉGICO - 3 medios + 2 críticos

fn main() {
    println!("🔍 DEBUG ESTRATÉGICO - 5 Errores Clave");
    println!("=======================================\n");

    // CRÍTICO #1: importing_stack.push() - Línea 394
    println!("CRÍTICO #1: importing_stack.push()");
    debug_critico_1();

    // CRÍTICO #2: funcs.insert() - Línea 1451
    println!("\nCRÍTICO #2: funcs.insert()");
    debug_critico_2();

    // MEDIO #1: funcs.get() - Línea 1740
    println!("\nMEDIO #1: funcs.get()");
    debug_medio_1();

    // MEDIO #2: importing_stack en gfx - Línea 1831
    println!("\nMEDIO #2: importing_stack en gfx");
    debug_medio_2();

    // MEDIO #3: HashMap contains - Línea 4522
    println!("\nMEDIO #3: HashMap contains()");
    debug_medio_3();

    println!("\n✅ DEBUG ESTRATÉGICO COMPLETADO");
    println!("================================");
    println!("Fixes identificados - Listo para aplicar");
}

fn debug_critico_1() {
    // ERROR #1/13 - Línea 394
    // importing_stack: &mut Vec<String>
    // module: &str
    
    let mut importing_stack: Vec<String> = vec!["module1".to_string()];
    let module: &str = "module2";
    
    println!("  importing_stack: Vec<String>");
    println!("  module: &str = \"{}\"", module);
    
    // ❌ ERROR: importing_stack.push(module.clone())
    // module.clone() retorna &str, no String
    
    // ✅ FIX: importing_stack.push(module.to_string())
    importing_stack.push(module.to_string());
    
    println!("  ✅ FIX: module.to_string()");
    println!("     - importing_stack.push(module.to_string())");
    println!("     - Result: {:?}", importing_stack);
}

fn debug_critico_2() {
    // ERROR #2/13 - Línea 1451
    // funcs: HashMap<String, (Vec<String>, Vec<Stmt>)>
    // name: &str (del pattern match)
    
    use std::collections::HashMap;
    
    let mut funcs: HashMap<String, (Vec<String>, Vec<i32>)> = HashMap::new();
    let name: &str = "mi_funcion";
    let params: Vec<&str> = vec!["param1", "param2"];
    let body: Vec<i32> = vec![1, 2, 3];
    
    println!("  funcs: HashMap<String, (Vec<String>, Vec<Stmt>)>");
    println!("  name: &str = \"{}\"", name);
    
    // ❌ ERROR: funcs.insert(name.clone(), ...)
    // name.clone() retorna &str, no String
    
    // ✅ FIX: funcs.insert(name.to_string(), ...)
    funcs.insert(name.to_string(), (params.iter().map(|s| s.to_string()).collect(), body.clone()));
    
    println!("  ✅ FIX: name.to_string()");
    println!("     - funcs.insert(name.to_string(), ...)");
    println!("     - funcs.len() = {}", funcs.len());
}

fn debug_medio_1() {
    // ERROR #3/13 - Línea 1740
    // funcs.get(func_name) requiere &K, no K
    
    use std::collections::HashMap;
    
    let mut funcs: HashMap<String, i32> = HashMap::new();
    funcs.insert("key".to_string(), 42);
    
    let func_name: String = "key".to_string();
    
    println!("  funcs: HashMap<String, i32>");
    println!("  func_name: String = \"{}\"", func_name);
    
    // ❌ ERROR: funcs.get(func_name)
    // get() requiere &String o &str, no String
    
    // ✅ FIX: funcs.get(&func_name)
    let result = funcs.get(&func_name);
    
    println!("  ✅ FIX: funcs.get(&func_name)");
    println!("     - result = {:?}", result);
}

fn debug_medio_2() {
    // ERROR #4/13 - Línea 1831
    // Mismo problema que CRÍTICO #1 pero en gfx
    
    let mut importing_stack: Vec<String> = vec!["module1".to_string()];
    let module: &str = "module2";
    
    println!("  importing_stack: Vec<String> (en gfx)");
    println!("  module: &str = \"{}\"", module);
    
    // ✅ FIX: module.to_string()
    importing_stack.push(module.to_string());
    
    println!("  ✅ FIX: module.to_string()");
    println!("     - importing_stack.push(module.to_string())");
    println!("     - Result: {:?}", importing_stack);
}

fn debug_medio_3() {
    // ERROR #5/13 - Línea 4522
    // loaded_modules.contains(module) requiere &String
    
    use std::collections::HashSet;
    
    let mut loaded_modules: HashSet<String> = HashSet::new();
    loaded_modules.insert("module1".to_string());
    
    let module: &str = "module1";
    
    println!("  loaded_modules: HashSet<String>");
    println!("  module: &str = \"{}\"", module);
    
    // ❌ ERROR: loaded_modules.contains(module)
    // contains() requiere &String, no &str
    
    // ✅ FIX: loaded_modules.contains(&module.to_string())
    let result = loaded_modules.contains(&module.to_string());
    
    println!("  ✅ FIX: &module.to_string()");
    println!("     - loaded_modules.contains(&module.to_string())");
    println!("     - result = {}", result);
}
