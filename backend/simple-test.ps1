# Simple DeckOracle API Test
$baseUrl = "http://localhost:8080/api/v1"

Write-Host "Testing DeckOracle API..." -ForegroundColor Green

# Test health
$health = Invoke-RestMethod -Uri "$baseUrl/health" -Method GET
Write-Host "Server Status: $($health.status)" -ForegroundColor Cyan

# Register user
Write-Host "`nRegistering user..." -ForegroundColor Yellow
$registerBody = @{
    email = "demo@test.com"
    password = "DemoPass123!"
    display_name = "Demo User"
} | ConvertTo-Json

try {
    $auth = Invoke-RestMethod -Uri "$baseUrl/auth/register" -Method POST -ContentType "application/json" -Body $registerBody
    Write-Host "Registration successful!" -ForegroundColor Green
    $token = $auth.access_token
} catch {
    Write-Host "Registration failed, trying login..." -ForegroundColor Yellow
    
    # Try login instead
    $loginBody = @{
        email = "demo@test.com"
        password = "DemoPass123!"
    } | ConvertTo-Json
    
    $auth = Invoke-RestMethod -Uri "$baseUrl/auth/login" -Method POST -ContentType "application/json" -Body $loginBody
    Write-Host "Login successful!" -ForegroundColor Green
    $token = $auth.access_token
}

Write-Host "`nAccess Token obtained!" -ForegroundColor Green
Write-Host "Token (first 30 chars): $($token.Substring(0, [Math]::Min(30, $token.Length)))..." -ForegroundColor Gray

# Create a test deck
Write-Host "`nCreating a test deck..." -ForegroundColor Yellow
$headers = @{ "Authorization" = "Bearer $token" }

$deckBody = @{
    name = "Test Deck"
    description = "A test flashcard deck"
    is_public = $false
} | ConvertTo-Json

$deck = Invoke-RestMethod -Uri "$baseUrl/decks" -Method POST -ContentType "application/json" -Body $deckBody -Headers $headers
Write-Host "Deck created with ID: $($deck.id)" -ForegroundColor Green

# Add some cards
Write-Host "`nAdding cards..." -ForegroundColor Yellow
$cardBody = @{
    front = "What is the capital of France?"
    back = "Paris"
} | ConvertTo-Json

$card = Invoke-RestMethod -Uri "$baseUrl/cards?deck_id=$($deck.id)" -Method POST -ContentType "application/json" -Body $cardBody -Headers $headers
Write-Host "Card added!" -ForegroundColor Green

# List decks
Write-Host "`nYour decks:" -ForegroundColor Yellow
$decks = Invoke-RestMethod -Uri "$baseUrl/decks" -Method GET -Headers $headers
foreach ($d in $decks) {
    Write-Host "- $($d.name): $($d.card_count) cards" -ForegroundColor Cyan
}

Write-Host "`n================================" -ForegroundColor Green
Write-Host "API is working! You can now:" -ForegroundColor Green
Write-Host "1. Use the API with tools like Postman or Insomnia" -ForegroundColor White
Write-Host "2. Build a frontend application (React, Vue, etc.)" -ForegroundColor White
Write-Host "3. Use the provided access token for authenticated requests" -ForegroundColor White
Write-Host "`nAPI Documentation available in API.md" -ForegroundColor Yellow
