# =============================================================================
# RyDit Engine - Instalador para Windows
# =============================================================================
# Versión: v0.8.0
# Plataforma: Windows 10/11
# Arquitectura: x86_64
# =============================================================================

#Requires -Version 5.0

Write-Host ""
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "  🛡️  RyDit Engine - Instalador para Windows" -ForegroundColor Cyan
Write-Host "  Versión: v0.8.0" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""

# Verificar arquitectura
$arch = (Get-CimInstance Win32_Processor).AddressWidth | Select-Object -First 1
Write-Host "[INFO] Arquitectura: $arch-bit" -ForegroundColor Blue

if ($arch -ne 64) {
    Write-Host "[✗] Error: RyDit requiere Windows 64-bit" -ForegroundColor Red
    exit 1
}

Write-Host "[✓] Arquitectura x86_64 detectada" -ForegroundColor Green
Write-Host ""

# Verificar winget
Write-Host "[INFO] Verificando winget..." -ForegroundColor Blue

try {
    $winget = Get-Command winget -ErrorAction Stop
    Write-Host "[✓] winget disponible" -ForegroundColor Green
} catch {
    Write-Host "[⚠] winget no disponible. Se usará instalación manual." -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Por favor instala:" -ForegroundColor Yellow
    Write-Host "  1. Rust: https://rustup.rs" -ForegroundColor Yellow
    Write-Host "  2. Visual Studio Build Tools: https://aka.ms/vs/17/release/vs_buildtools.exe" -ForegroundColor Yellow
    Write-Host "  3. raylib: winget install raylib.raylib" -ForegroundColor Yellow
    exit 1
}

# Instalar Rust
Write-Host "[INFO] Verificando Rust..." -ForegroundColor Blue

try {
    $rustc = Get-Command rustc -ErrorAction Stop
    $rustVersion = & rustc --version
    Write-Host "[✓] Rust instalado: $rustVersion" -ForegroundColor Green
} catch {
    Write-Host "[INFO] Instalando Rust..." -ForegroundColor Blue
    
    # Usar winget para instalar Rust
    try {
        winget install --id Rustlang.Rustup -e --silent | Out-Null
        Write-Host "[✓] Rust instalado" -ForegroundColor Green
        
        # Recargar PATH
        $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
    } catch {
        Write-Host "[⚠] Instalación automática falló. Instala manualmente:" -ForegroundColor Yellow
        Write-Host "  https://rustup.rs" -ForegroundColor Yellow
        exit 1
    }
}

# Verificar Cargo
try {
    $cargo = Get-Command cargo -ErrorAction Stop
    $cargoVersion = & cargo --version
    Write-Host "[✓] Cargo disponible: $cargoVersion" -ForegroundColor Green
} catch {
    Write-Host "[✗] Cargo no encontrado. Reinstala Rust." -ForegroundColor Red
    exit 1
}

Write-Host ""

# Instalar Visual Studio Build Tools
Write-Host "[INFO] Verificando Visual Studio Build Tools..." -ForegroundColor Blue

$vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (Test-Path $vsWhere) {
    $vsInstalled = & $vsWhere -latest -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64
    if ($vsInstalled) {
        Write-Host "[✓] Visual Studio Build Tools instalado" -ForegroundColor Green
    } else {
        Write-Host "[⚠] Build Tools no encontrado" -ForegroundColor Yellow
    }
} else {
    Write-Host "[INFO] Instalando Visual Studio Build Tools..." -ForegroundColor Blue
    
    try {
        # Descargar e instalar Build Tools
        $installerUrl = "https://aka.ms/vs/17/release/vs_buildtools.exe"
        $installerPath = "$env:TEMP\vs_buildtools.exe"
        
        Write-Host "[INFO] Descargando instalador..." -ForegroundColor Blue
        Invoke-WebRequest -Uri $installerUrl -OutFile $installerPath
        
        Write-Host "[INFO] Ejecutando instalador..." -ForegroundColor Blue
        Start-Process -FilePath $installerPath -ArgumentList "--quiet", "--wait", "--add", "Microsoft.VisualStudio.Workload.VCTools" -Wait
        
        Write-Host "[✓] Visual Studio Build Tools instalado" -ForegroundColor Green
        Remove-Item $installerPath -Force
    } catch {
        Write-Host "[⚠] Instalación automática falló. Instala manualmente:" -ForegroundColor Yellow
        Write-Host "  $installerUrl" -ForegroundColor Yellow
    }
}

Write-Host ""

# Instalar raylib
Write-Host "[INFO] Verificando raylib..." -ForegroundColor Blue

try {
    $raylib = winget list raylib -e --silent | Select-String "raylib"
    if ($raylib) {
        Write-Host "[✓] raylib instalado" -ForegroundColor Green
    } else {
        throw "No instalado"
    }
} catch {
    Write-Host "[INFO] Instalando raylib..." -ForegroundColor Blue
    
    try {
        winget install raylib.raylib -e --silent | Out-Null
        Write-Host "[✓] raylib instalado" -ForegroundColor Green
    } catch {
        Write-Host "[⚠] raylib no instalado. Puedes compilar sin él (sin gráficos)." -ForegroundColor Yellow
    }
}

Write-Host ""

# Instalar Git (opcional)
Write-Host "[INFO] Verificando Git..." -ForegroundColor Blue

try {
    $git = Get-Command git -ErrorAction Stop
    $gitVersion = & git --version
    Write-Host "[✓] Git disponible: $gitVersion" -ForegroundColor Green
} catch {
    Write-Host "[INFO] ¿Quieres instalar Git? (recomendado)" -ForegroundColor Blue
    $installGit = Read-Host "Instalar Git? (s/n)"
    
    if ($installGit -eq "s" -or $installGit -eq "S") {
        try {
            winget install Git.Git -e --silent | Out-Null
            Write-Host "[✓] Git instalado" -ForegroundColor Green
            
            # Recargar PATH
            $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
        } catch {
            Write-Host "[⚠] Git no instalado. Puedes instalarlo manualmente: https://git-scm.com" -ForegroundColor Yellow
        }
    }
}

Write-Host ""

# Compilar proyecto
Write-Host "[INFO] ¿Deseas compilar RyDit Engine ahora?" -ForegroundColor Blue
$compile = Read-Host "Compilar? (s/n)"

if ($compile -eq "s" -or $compile -eq "S") {
    Write-Host ""
    Write-Host "[INFO] Compilando RyDit Engine..." -ForegroundColor Blue
    Write-Host ""
    
    # Build release
    $buildOutput = cargo build --release 2>&1 | Tee-Object -Variable buildResult
    $buildOutput | Select-Object -Last 20
    
    # Verificar compilación
    if (Test-Path "target\release\rydit-rs.exe") {
        Write-Host "[✓] ¡Compilación exitosa!" -ForegroundColor Green
        
        # Tamaño del binario
        $binSize = (Get-Item "target\release\rydit-rs.exe").Length / 1MB
        Write-Host "[INFO] Tamaño del binario: $([math]::Round($binSize, 2)) MB" -ForegroundColor Blue
        
        # Agregar al PATH (opcional)
        Write-Host ""
        Write-Host "[INFO] ¿Quieres agregar RyDit al PATH del sistema?" -ForegroundColor Blue
        $addToPath = Read-Host "Agregar al PATH? (s/n)"
        
        if ($addToPath -eq "s" -or $addToPath -eq "S") {
            try {
                $installPath = "$env:LOCALAPPDATA\RyDit"
                New-Item -ItemType Directory -Force -Path $installPath | Out-Null
                Copy-Item "target\release\rydit-rs.exe" "$installPath\rydit.exe" -Force
                
                $currentPath = [System.Environment]::GetEnvironmentVariable("Path","User")
                if ($currentPath -notlike "*$installPath*") {
                    [System.Environment]::SetEnvironmentVariable("Path", "$currentPath;$installPath", "User")
                    Write-Host "[✓] RyDit agregado al PATH (reinicia la terminal para usar)" -ForegroundColor Green
                }
            } catch {
                Write-Host "[⚠] No se pudo agregar al PATH. Puedes ejecutar desde: .\target\release\rydit-rs.exe" -ForegroundColor Yellow
            }
        }
    } else {
        Write-Host "[✗] La compilación falló. Revisa los errores arriba." -ForegroundColor Red
        exit 1
    }
} else {
    Write-Host "[INFO] Puedes compilar después con: cargo build --release" -ForegroundColor Blue
}

Write-Host ""

# Crear script de demo
$demoScript = @"
# RyDit - Lanzador de demos
param(
    [string]`$demoName
)

if ([string]::IsNullOrEmpty(`$demoName)) {
    Write-Host "Uso: rydit-demo <nombre_demo>"
    Write-Host ""
    Write-Host "Demos disponibles:"
    Get-ChildItem "demos\*.rydit" -ErrorAction SilentlyContinue | ForEach-Object {
        Write-Host "  - " `$_.BaseName
    }
    exit 1
}

& rydit --gfx "demos\`$demoName.rydit"
"@

$demoScriptPath = "$env:LOCALAPPDATA\RyDit\rydit-demo.ps1"
New-Item -ItemType Directory -Force -Path (Split-Path $demoScriptPath) | Out-Null
$demoScript | Out-File -FilePath $demoScriptPath -Encoding UTF8

Write-Host "[✓] Script 'rydit-demo' creado" -ForegroundColor Green

# Resumen final
Write-Host ""
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "  ✅ ¡Instalación de RyDit Engine completada!" -ForegroundColor Green
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "📦 Comandos disponibles:" -ForegroundColor White
Write-Host ""

if (Test-Path "$env:LOCALAPPDATA\RyDit\rydit.exe") {
    Write-Host "  • rydit              - Ejecutar RyDit" -ForegroundColor White
    Write-Host "  • rydit --repl       - Modo REPL interactivo" -ForegroundColor White
    Write-Host "  • rydit --gfx <file> - Ejecutar demo gráfico" -ForegroundColor White
} else {
    Write-Host "  • .\target\release\rydit-rs.exe - Ejecutar RyDit" -ForegroundColor White
}

Write-Host "  • rydit-demo <demo>  - Ejecutar demo (PowerShell)" -ForegroundColor White
Write-Host ""
Write-Host "📚 Documentación:" -ForegroundColor White
Write-Host "  • README.md          - Documentación principal" -ForegroundColor White
Write-Host "  • docs/              - Más documentación" -ForegroundColor White
Write-Host ""
Write-Host "🎮 Primeros pasos:" -ForegroundColor White
Write-Host "  1. Ejecuta: rybot" -ForegroundColor White
Write-Host "  2. O prueba: rydit --repl" -ForegroundColor White
Write-Host ""
Write-Host "🛠️  Desarrollo:" -ForegroundColor White
Write-Host "  • cargo build --release  - Compilar" -ForegroundColor White
Write-Host "  • cargo test             - Ejecutar tests" -ForegroundColor White
Write-Host "  • cargo run --bin rydit-rs -- --repl" -ForegroundColor White
Write-Host ""
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""

# Ofrecer ejecutar Rybot
Write-Host "[INFO] ¿Quieres ejecutar Rybot ahora?" -ForegroundColor Blue
$runRybot = Read-Host "Ejecutar rybot? (s/n)"

if ($runRybot -eq "s" -or $runRybot -eq "S") {
    if (Test-Path ".\rybot.sh") {
        & bash rybot.sh
    } elseif (Test-Path ".\rybot.ps1") {
        & .\rybot.ps1
    } else {
        Write-Host "[⚠] rybot no encontrado en el directorio actual" -ForegroundColor Yellow
    }
}

Write-Host ""
Write-Host "¡Gracias por instalar RyDit Engine! 🛡️" -ForegroundColor Green
Write-Host ""

exit 0
