# DeckOracle Launch Script for Windows
# This script starts both backend and frontend in development mode

param(
    [Parameter(Position=0)]
    [string]$Mode = "dev"
)

$ErrorActionPreference = "Continue"

Write-Host @"
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║     ____            _     ___                  _            ║
║    |  _ \  ___  ___| | __/ _ \ _ __ __ _  ___| | ___       ║
║    | | | |/ _ \/ __| |/ / | | | '__/ _` |/ __| |/ _ \      ║
║    | |_| |  __/ (__|   <| |_| | | | (_| | (__| |  __/      ║
║    |____/ \___|___|_|\_\\___/|_|  \__,_|\___|_|\___|      ║
║                                                              ║
║           AI-Powered Flashcard Learning Platform            ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
"@ -ForegroundColor Cyan

function Test-Prerequisites {
    Write-Host "`nChecking prerequisites..." -ForegroundColor Yellow
    
    $hasErrors = $false
    
    # Check PostgreSQL
    try {
        $pgVersion = & psql --version 2>$null
        if ($pgVersion) {
            Write-Host "[OK] PostgreSQL found: $pgVersion" -ForegroundColor Green
        } else {
            Write-Host "[X] PostgreSQL not found in PATH" -ForegroundColor Red
            $hasErrors = $true
        }
    } catch {
        Write-Host "[X] PostgreSQL not found" -ForegroundColor Red
        $hasErrors = $true
    }
    
    # Check Rust
    try {
        $rustVersion = & "$env:USERPROFILE\.cargo\bin\rustc.exe" --version 2>$null
        if ($rustVersion) {
            Write-Host "[OK] Rust found: $rustVersion" -ForegroundColor Green
        } else {
            Write-Host "[X] Rust not found" -ForegroundColor Red
            $hasErrors = $true
        }
    } catch {
        Write-Host "[X] Rust not found" -ForegroundColor Red
        $hasErrors = $true
    }
    
    # Check Node.js
    try {
        $nodeVersion = & node --version 2>$null
        if ($nodeVersion) {
            Write-Host "[OK] Node.js found: $nodeVersion" -ForegroundColor Green
        } else {
            Write-Host "[X] Node.js not found" -ForegroundColor Red
            $hasErrors = $true
        }
    } catch {
        Write-Host "[X] Node.js not found" -ForegroundColor Red
        $hasErrors = $true
    }
    
    # Check if database exists
    try {
        $dbExists = & psql -U postgres -lqt 2>$null | Select-String "deckoracle_db"
        if ($dbExists) {
            Write-Host "[OK] Database 'deckoracle_db' exists" -ForegroundColor Green
        } else {
            Write-Host "[!] Database 'deckoracle_db' not found - creating..." -ForegroundColor Yellow
            & psql -U postgres -c "CREATE DATABASE deckoracle_db;" 2>$null
            Write-Host "[OK] Database created" -ForegroundColor Green
        }
    } catch {
        Write-Host "[!] Could not check database" -ForegroundColor Yellow
    }
    
    return -not $hasErrors
}

function Start-Backend {
    Write-Host "`nStarting Backend Server..." -ForegroundColor Cyan
    
    $backendPath = Join-Path $PSScriptRoot "backend"
    
    # Check if .env exists
    $envPath = Join-Path $backendPath ".env"
    if (-not (Test-Path $envPath)) {
        $envExamplePath = Join-Path $backendPath ".env.example"
        if (Test-Path $envExamplePath) {
            Copy-Item $envExamplePath $envPath
            Write-Host "Created .env from .env.example - please update with your database password" -ForegroundColor Yellow
        }
    }
    
    # Start backend in new terminal
    $backendScript = @"
cd '$backendPath'
Write-Host 'DeckOracle Backend' -ForegroundColor Cyan
Write-Host '==================' -ForegroundColor Cyan
Write-Host ''
Write-Host 'Running migrations...' -ForegroundColor Yellow
& '$env:USERPROFILE\.cargo\bin\sqlx.exe' migrate run
Write-Host 'Starting server...' -ForegroundColor Green
& '$env:USERPROFILE\.cargo\bin\cargo.exe' run
"@
    
    Start-Process powershell -ArgumentList @("-NoExit", "-Command", $backendScript)
    
    Write-Host "Backend starting on http://localhost:8080" -ForegroundColor Green
}

function Start-Frontend {
    Write-Host "`nStarting Frontend Server..." -ForegroundColor Cyan
    
    $frontendPath = Join-Path $PSScriptRoot "frontend"
    
    # Check if node_modules exists
    $nodeModulesPath = Join-Path $frontendPath "node_modules"
    if (-not (Test-Path $nodeModulesPath)) {
        Write-Host "Installing frontend dependencies..." -ForegroundColor Yellow
        Push-Location $frontendPath
        & npm install
        Pop-Location
    }
    
    # Start frontend in new terminal
    $frontendScript = @"
cd '$frontendPath'
Write-Host 'DeckOracle Frontend' -ForegroundColor Cyan
Write-Host '===================' -ForegroundColor Cyan
Write-Host ''
Write-Host 'Starting development server...' -ForegroundColor Green
npm run dev
"@
    
    Start-Process powershell -ArgumentList @("-NoExit", "-Command", $frontendScript)
    
    Write-Host "Frontend starting on http://localhost:5173" -ForegroundColor Green
}

function Start-Docker {
    Write-Host "`nStarting with Docker..." -ForegroundColor Cyan
    
    # Check if Docker is running
    try {
        & docker ps 2>$null | Out-Null
    } catch {
        Write-Host "[X] Docker is not running. Please start Docker Desktop." -ForegroundColor Red
        return
    }
    
    # Start services
    if ($Mode -eq "prod") {
        & docker-compose --profile prod up
    } else {
        & docker-compose --profile dev up
    }
}

# Main execution
switch ($Mode.ToLower()) {
    "dev" {
        if (Test-Prerequisites) {
            Start-Backend
            Start-Sleep -Seconds 3
            Start-Frontend
            
            Write-Host "`n" -NoNewline
            Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Green
            Write-Host "  DeckOracle is starting!" -ForegroundColor Green
            Write-Host "═══════════════════════════════════════════════════════" -ForegroundColor Green
            Write-Host ""
            Write-Host "  Backend:  http://localhost:8080" -ForegroundColor Cyan
            Write-Host "  Frontend: http://localhost:5173" -ForegroundColor Cyan
            Write-Host "  API Docs: http://localhost:8080/api/v1/health" -ForegroundColor Cyan
            Write-Host ""
            Write-Host "  Press Ctrl+C in each terminal to stop" -ForegroundColor Yellow
            Write-Host ""
            
            # Open browser after a delay
            Start-Sleep -Seconds 5
            Start-Process "http://localhost:5173"
        } else {
            Write-Host "`n[X] Prerequisites check failed. Please install missing components." -ForegroundColor Red
        }
    }
    "docker" {
        Start-Docker
    }
    "stop" {
        Write-Host "Stopping services..." -ForegroundColor Yellow
        Get-Process | Where-Object { $_.ProcessName -match "cargo|node" } | Stop-Process -Force
        Write-Host "[OK] Services stopped" -ForegroundColor Green
    }
    default {
        Write-Host "Usage: .\launch.ps1 [dev|docker|stop]" -ForegroundColor Yellow
        Write-Host "  dev    - Start in development mode (default)" -ForegroundColor Gray
        Write-Host "  docker - Start with Docker" -ForegroundColor Gray
        Write-Host "  stop   - Stop all services" -ForegroundColor Gray
    }
}