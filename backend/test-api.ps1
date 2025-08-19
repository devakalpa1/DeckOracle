# DeckOracle API Test Script
$baseUrl = "http://localhost:8080/api/v1"

Write-Host "DeckOracle API Test Script" -ForegroundColor Cyan
Write-Host "=========================" -ForegroundColor Cyan
Write-Host ""

# Test 1: Health Check
Write-Host "1. Testing Health Endpoint..." -ForegroundColor Yellow
try {
    $health = Invoke-RestMethod -Uri "$baseUrl/health" -Method GET
    Write-Host "   ✓ Server is healthy: $($health.status)" -ForegroundColor Green
} catch {
    Write-Host "   ✗ Health check failed: $_" -ForegroundColor Red
}
Write-Host ""

# Test 2: Register a new user
Write-Host "2. Registering a new user..." -ForegroundColor Yellow
$registerBody = @{
    email = "test@example.com"
    password = "TestPassword123!"
    display_name = "Test User"
} | ConvertTo-Json

try {
    $registerResponse = Invoke-RestMethod -Uri "$baseUrl/auth/register" -Method POST `
        -ContentType "application/json" -Body $registerBody
    Write-Host "   ✓ User registered successfully!" -ForegroundColor Green
    Write-Host "   User ID: $($registerResponse.user.id)" -ForegroundColor Gray
    $global:accessToken = $registerResponse.access_token
    $global:refreshToken = $registerResponse.refresh_token
} catch {
    Write-Host "   ✗ Registration failed: $_" -ForegroundColor Red
    # If user already exists, try logging in
    if ($_.Exception.Response.StatusCode -eq 'Conflict' -or $_.Exception.Response.StatusCode -eq 'BadRequest') {
        Write-Host "   User might already exist, trying to login..." -ForegroundColor Yellow
    }
}
Write-Host ""

# Test 3: Login
Write-Host "3. Logging in..." -ForegroundColor Yellow
$loginBody = @{
    email = "test@example.com"
    password = "TestPassword123!"
} | ConvertTo-Json

try {
    $loginResponse = Invoke-RestMethod -Uri "$baseUrl/auth/login" -Method POST `
        -ContentType "application/json" -Body $loginBody
    Write-Host "   ✓ Login successful!" -ForegroundColor Green
    $global:accessToken = $loginResponse.access_token
    $global:refreshToken = $loginResponse.refresh_token
    Write-Host "   Access Token: $($global:accessToken.Substring(0, 20))..." -ForegroundColor Gray
} catch {
    Write-Host "   ✗ Login failed: $_" -ForegroundColor Red
}
Write-Host ""

# Test 4: Create a folder (requires authentication)
if ($global:accessToken) {
    Write-Host "4. Creating a folder..." -ForegroundColor Yellow
    $folderBody = @{
        name = "My Study Folder"
    } | ConvertTo-Json
    
    $headers = @{
        "Authorization" = "Bearer $global:accessToken"
    }
    
    try {
        $folder = Invoke-RestMethod -Uri "$baseUrl/folders" -Method POST `
            -ContentType "application/json" -Body $folderBody -Headers $headers
        Write-Host "   ✓ Folder created: $($folder.name)" -ForegroundColor Green
        $global:folderId = $folder.id
    } catch {
        Write-Host "   ✗ Failed to create folder: $_" -ForegroundColor Red
    }
    Write-Host ""
}

# Test 5: Create a deck (requires authentication)
if ($global:accessToken -and $global:folderId) {
    Write-Host "5. Creating a deck..." -ForegroundColor Yellow
    $deckBody = @{
        name = "Spanish Vocabulary"
        description = "Basic Spanish words"
        folder_id = $global:folderId
        is_public = $false
    } | ConvertTo-Json
    
    $headers = @{
        "Authorization" = "Bearer $global:accessToken"
    }
    
    try {
        $deck = Invoke-RestMethod -Uri "$baseUrl/decks" -Method POST `
            -ContentType "application/json" -Body $deckBody -Headers $headers
        Write-Host "   ✓ Deck created: $($deck.name)" -ForegroundColor Green
        $global:deckId = $deck.id
    } catch {
        Write-Host "   ✗ Failed to create deck: $_" -ForegroundColor Red
    }
    Write-Host ""
}

# Test 6: Add cards to deck
if ($global:accessToken -and $global:deckId) {
    Write-Host "6. Adding cards to deck..." -ForegroundColor Yellow
    $cards = @(
        @{front = "Hello"; back = "Hola"},
        @{front = "Goodbye"; back = "Adiós"},
        @{front = "Thank you"; back = "Gracias"}
    ) | ConvertTo-Json
    
    $headers = @{
        "Authorization" = "Bearer $global:accessToken"
    }
    
    try {
        $cardsResponse = Invoke-RestMethod -Uri "$baseUrl/cards/bulk?deck_id=$global:deckId" -Method POST `
            -ContentType "application/json" -Body $cards -Headers $headers
        Write-Host "   ✓ Added $($cardsResponse.Count) cards to deck" -ForegroundColor Green
    } catch {
        Write-Host "   ✗ Failed to add cards: $_" -ForegroundColor Red
    }
    Write-Host ""
}

# Test 7: List user's decks
if ($global:accessToken) {
    Write-Host "7. Listing user's decks..." -ForegroundColor Yellow
    $headers = @{
        "Authorization" = "Bearer $global:accessToken"
    }
    
    try {
        $decks = Invoke-RestMethod -Uri "$baseUrl/decks" -Method GET -Headers $headers
        Write-Host "   ✓ Found $($decks.Count) deck(s):" -ForegroundColor Green
        foreach ($d in $decks) {
            Write-Host ("     - " + $d.name + " (" + $d.card_count + " cards)") -ForegroundColor Gray
        }
    } catch {
        Write-Host "   ✗ Failed to list decks: $_" -ForegroundColor Red
    }
    Write-Host ""
}

Write-Host "API Test Complete!" -ForegroundColor Cyan
Write-Host ""
Write-Host "You can now use these tokens to make authenticated requests:" -ForegroundColor Yellow
Write-Host "Access Token: $($global:accessToken)" -ForegroundColor Gray
Write-Host ""
Write-Host "Example curl command:" -ForegroundColor Yellow
Write-Host ("curl -H 'Authorization: Bearer " + $global:accessToken + "' http://localhost:8080/api/v1/decks") -ForegroundColor Gray
