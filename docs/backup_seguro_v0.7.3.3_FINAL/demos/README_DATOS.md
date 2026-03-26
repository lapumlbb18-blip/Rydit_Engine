# 📊 Módulo de Ciencia de Datos - RyDit v0.7.1.3

## Descripción

El módulo de ciencia de datos proporciona funciones para parsear CSV, calcular estadísticas y generar gráficos.

## Funciones Disponibles

### CSV

#### `csv::parse(csv_text)`

Parsea CSV con headers en la primera línea.

**Parámetros:**
- `csv_text` (texto): Contenido CSV completo

**Retorna:**
- Array de arrays: `[[fila1_col1, fila1_col2, ...], [fila2_col1, ...], ...]`
- **Nota:** Los headers NO se incluyen en el resultado

**Ejemplo:**
```rydit
dark.slot datos = "nombre,edad,ciudad
Juan,25,México
María,30,España"

dark.slot filas = csv::parse(datos)
print("Filas: " + filas.len())  # 2
print("Primera fila: " + filas[0][0] + ", " + filas[0][1])
```

---

#### `csv::parse_no_headers(csv_text)`

Parsea CSV sin headers (todas las líneas son datos).

**Parámetros:**
- `csv_text` (texto): Contenido CSV completo

**Retorna:**
- Array de arrays con todas las filas

**Ejemplo:**
```rydit
dark.slot datos = "1,2,3
4,5,6
7,8,9"

dark.slot filas = csv::parse_no_headers(datos)
print("Fila 1: " + filas[0])  # [1, 2, 3]
```

---

### Estadísticas

#### `stats::mean(array)`

Calcula la media aritmética (promedio).

**Parámetros:**
- `array` (array): Array de números

**Retorna:**
- Número: Media aritmética

**Fórmula:**
```
mean = (x1 + x2 + ... + xn) / n
```

**Ejemplo:**
```rydit
dark.slot edades = [25, 30, 35, 28, 22]
dark.slot promedio = stats::mean(edades)
print("Edad promedio: " + promedio + " años")  # 28.0
```

---

#### `stats::median(array)`

Calcula la mediana (valor central).

**Parámetros:**
- `array` (array): Array de números

**Retorna:**
- Número: Mediana

**Algoritmo:**
1. Ordenar array
2. Si n impar: valor central
3. Si n par: promedio de dos valores centrales

**Ejemplo:**
```rydit
dark.slot datos = [1, 3, 5, 7, 9]
dark.slot med = stats::median(datos)
print("Mediana: " + med)  # 5

dark.slot datos2 = [1, 2, 3, 4]
dark.slot med2 = stats::median(datos2)
print("Mediana: " + med2)  # 2.5
```

---

#### `stats::std_dev(array)`

Calcula la desviación estándar muestral.

**Parámetros:**
- `array` (array): Array de números

**Retorna:**
- Número: Desviación estándar

**Fórmula:**
```
varianza = Σ(xi - mean)² / (n - 1)
std_dev = √varianza
```

**Ejemplo:**
```rydit
dark.slot datos = [2, 4, 4, 4, 5, 5, 7, 9]
dark.slot desv = stats::std_dev(datos)
print("Desviación estándar: " + desv)  # ~2.14
```

---

#### `stats::min(array)`

Encuentra el valor mínimo.

**Parámetros:**
- `array` (array): Array de números

**Retorna:**
- Número: Valor mínimo

**Ejemplo:**
```rydit
dark.slot temperaturas = [22, 25, 19, 28, 21]
dark.slot min_temp = stats::min(temperaturas)
print("Temperatura mínima: " + min_temp + "°C")  # 19
```

---

#### `stats::max(array)`

Encuentra el valor máximo.

**Parámetros:**
- `array` (array): Array de números

**Retorna:**
- Número: Valor máximo

**Ejemplo:**
```rydit
dark.slot ventas = [1000, 1500, 1200, 1800, 900]
dark.slot max_ventas = stats::max(ventas)
print("Venta máxima: $" + max_ventas)  # 1800
```

---

### Gráficos

#### `plot::ascii_chart(data, width)`

Genera un gráfico ASCII de líneas.

**Parámetros:**
- `data` (array): Array de números (valores Y)
- `width` (número): Ancho del gráfico en caracteres

**Retorna:**
- Texto: Gráfico ASCII multi-línea

**Ejemplo:**
```rydit
dark.slot ventas = [100, 150, 200, 180, 220, 250]
dark.slot grafico = plot::ascii_chart(ventas, 40)
print(grafico)
```

**Salida:**
```
                                        
                                        
                                        
                                        
                    *                   
                  *   *                 
                *       *               
              *           *   *         
            *               *     *     
----------------------------------------
```

---

#### `plot::svg_chart(data, width, height)`

Genera un gráfico SVG de líneas.

**Parámetros:**
- `data` (array): Array de números
- `width` (número): Ancho del SVG en píxeles
- `height` (número): Alto del SVG en píxeles

**Retorna:**
- Texto: Código SVG completo

**Ejemplo:**
```rydit
dark.slot datos = [10, 25, 18, 30, 22, 35]
dark.slot svg = plot::svg_chart(datos, 300, 200)
print(svg)  # <svg width='300' height='200'>...</svg>
```

**Uso en HTML:**
```html
<div id="chart"></div>
<script>
    // Insertar SVG generado por RyDit
    document.getElementById('chart').innerHTML = svg_output;
</script>
```

---

## Casos de Uso

### 1. Análisis de Ventas

```rydit
# Datos mensuales
dark.slot csv_ventas = "mes,ventas,gastos
Ene,1000,800
Feb,1200,850
Mar,950,780
Abr,1100,820
May,1300,900"

# Parsear
dark.slot filas = csv::parse(csv_ventas)

# Extraer ventas
dark.slot ventas = []
dark.slot i = 0
mientras i < filas.len() {
    ventas.push(filas[i][1].numero())
    i = i + 1
}

# Calcular estadísticas
dark.slot promedio = stats::mean(ventas)
dark.slot max_venta = stats::max(ventas)
dark.slot min_venta = stats::min(ventas)

print("Ventas:")
print("  Promedio: $" + promedio)
print("  Máxima: $" + max_venta)
print("  Mínima: $" + min_venta)

# Gráfico
print("\nGráfico de ventas:")
print(plot::ascii_chart(ventas, 50))
```

---

### 2. Análisis de Temperaturas

```rydit
# Temperaturas diarias (°C)
dark.slot temps = [22, 24, 23, 25, 27, 26, 28, 30, 29, 28, 27, 26]

# Estadísticas
dark.slot media = stats::mean(temps)
dark.slot mediana = stats::median(temps)
dark.slot desv = stats::std_dev(temps)

print("Análisis de temperaturas:")
print("  Media: " + media + "°C")
print("  Mediana: " + mediana + "°C")
print("  Desviación: " + desv + "°C")
print("  Rango: " + stats::min(temps) + "°C - " + stats::max(temps) + "°C")

# Gráfico
print("\nTendencia:")
print(plot::ascii_chart(temps, 60))
```

---

### 3. Procesamiento de Datos CSV

```rydit
# Leer archivo (asumiendo file::read existe)
dark.slot csv_content = file::read("datos.csv")

# Parsear
dark.slot datos = csv::parse_no_headers(csv_content)

# Procesar cada fila
dark.slot i = 0
mientras i < datos.len() {
    dark.slot fila = datos[i]
    dark.slot valor = fila[0].numero()
    
    # Procesar valor
    si valor > 100 {
        print("Fila " + i + ": " + valor + " (ALTO)")
    } si_no {
        print("Fila " + i + ": " + valor)
    }
    
    i = i + 1
}
```

---

### 4. Generar Reporte SVG

```rydit
# Datos trimestrales
dark.slot q1 = [150, 180, 200, 170]
dark.slot q2 = [220, 250, 230, 260]

# Generar SVGs
dark.slot svg1 = plot::svg_chart(q1, 400, 300)
dark.slot svg2 = plot::svg_chart(q2, 400, 300)

# Guardar en archivos HTML
dark.slot html = "<html><body>" + svg1 + svg2 + "</body></html>"
# file::write("reporte.html", html)
```

---

## Conversión de Tipos

### String a Número

Los datos CSV son strings. Usar `.numero()` para convertir:

```rydit
dark.slot texto = "123"
dark.slot numero = texto.numero()  # 123.0

dark.slot datos = csv::parse("valor\n123\n456")
dark.slot primer_valor = datos[0][0].numero()  # 123.0
```

---

## Errores Comunes

### Array Vacío
```
Error: stats::mean() array vacío
```
**Solución:** Verificar `array.len() > 0` antes de calcular

---

### Datos No Numéricos
```
Error: stats::mean() requiere array de números
```
**Solución:** Asegurar que todos los elementos sean `Valor::Num`

---

### CSV Mal Formateado
```
Error: Error parseando CSV: CSV error: record 2 has 3 fields, but previous records had 4
```
**Solución:** Verificar que todas las filas tengan el mismo número de columnas

---

## Limitaciones

1. **CSV simple**: Sin comillas, escapes o campos multi-línea
2. **Sin tipos automáticos**: Todos los campos son strings (convertir manualmente)
3. **Estadísticas básicas**: Solo mean, median, std_dev, min, max
4. **Gráficos simples**: Sin ejes, labels o leyendas automáticas
5. **Sin pandas/numpy**: No hay DataFrames o arrays multidimensionales

---

## Ejemplos en el Repositorio

- `demos/demo_datos_v0.7.1.3.rydit` - Demo completo

---

<div align="center">

**🛡️ RyDit v0.7.1.3 - Ciencia de Datos**

*CSV + Estadísticas + Gráficos desde RyDit*

</div>
