# DeckOracle Backend Development Script for Windows

param(
    [Parameter(Position=0)]
    [string]$Command = "help"
)

$ErrorActionPreference = "Stop"

function Show-Help {
    Write-Host "DeckOracle Backend Development Commands" -ForegroundColor Cyan
    Write-Host "=======================================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\dev.ps1 [command]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Commands:" -ForegroundColor Green
    Write-Host "  help        - Show this help message"
    Write-Host "  dev         - Run development server with hot reload"
    Write-Host "  build       - Build release binary"
    Write-Host "  test        - Run tests"
    Write-Host "  migrate     - Run database migrations"
    Write-Host "  db-reset    - Reset database (drop, create, migrate)"
    Write-Host "  db-create   - Create database"
    Write-Host "  db-drop     - Drop database"
    Write-Host "  lint        - Run clippy linter"
    Write-Host "  fmt         - Format code"
    Write-Host "  check       - Check code without building"
    Write-Host "  install     - Install required tools"
    Write-Host "  clean       - Clean build artifacts"
}

function Run-Dev {
    Write-Host "Starting development server with hot reload..." -ForegroundColor Green
    cargo watch -x run
}

function Build-Release {
    Write-Host "Building release binary..." -ForegroundColor Green
    cargo build --release
}

function Run-Tests {
    Write-Host "Running tests..." -ForegroundColor Green
    cargo test
}

function Run-Migrations {
    Write-Host "Running database migrations..." -ForegroundColor Green
    sqlx migrate run
}

function Create-Database {
    Write-Host "Creating database..." -ForegroundColor Green
    sqlx database create
}

function Drop-Database {
    Write-Host "Dropping database..." -ForegroundColor Yellow
    sqlx database drop
}

function Reset-Database {
    Write-Host "Resetting database..." -ForegroundColor Yellow
    Drop-Database
    Create-Database
    Run-Migrations
    Write-Host "Database reset complete!" -ForegroundColor Green
}

function Run-Lint {
    Write-Host "Running clippy linter..." -ForegroundColor Green
    cargo clippy -- -D warnings
}

function Format-Code {
    Write-Host "Formatting code..." -ForegroundColor Green
    cargo fmt
}

function Check-Code {
    Write-Host "Checking code..." -ForegroundColor Green
    cargo check
}

function Install-Tools {
    Write-Host "Installing required tools..." -ForegroundColor Green
    
    Write-Host "Installing SQLx CLI..." -ForegroundColor Cyan
    cargo install sqlx-cli --no-default-features --features postgres
    
    Write-Host "Installing cargo-watch..." -ForegroundColor Cyan
    cargo install cargo-watch
    
    Write-Host "Tools installed successfully!" -ForegroundColor Green
}

function Clean-Build {
    Write-Host "Cleaning build artifacts..." -ForegroundColor Green
    cargo clean
}

# Main command switch
switch ($Command.ToLower()) {
    "help"      { Show-Help }
    "dev"       { Run-Dev }
    "build"     { Build-Release }
    "test"      { Run-Tests }
    "migrate"   { Run-Migrations }
    "db-reset"  { Reset-Database }
    "db-create" { Create-Database }
    "db-drop"   { Drop-Database }
    "lint"      { Run-Lint }
    "fmt"       { Format-Code }
    "check"     { Check-Code }
    "install"   { Install-Tools }
    "clean"     { Clean-Build }
    default     { 
        Write-Host "Unknown command: $Command" -ForegroundColor Red
        Write-Host ""
        Show-Help
    }
}
