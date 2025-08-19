# DeckOracle API Documentation

## Base URL
```
http://localhost:8080/api/v1
```

## Authentication
> ⚠️ **Note**: Authentication is not yet implemented. All endpoints currently use a placeholder user ID.

Future authentication will use JWT tokens in the Authorization header:
```
Authorization: Bearer <token>
```

## Endpoints

### 📁 Folders

#### List Folders
```http
GET /folders
```

**Response:**
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "user_id": "user-uuid",
    "parent_folder_id": null,
    "name": "Spanish Learning",
    "position": 0,
    "created_at": "2024-01-15T10:00:00Z",
    "updated_at": "2024-01-15T10:00:00Z"
  }
]
```

#### Create Folder
```http
POST /folders
Content-Type: application/json

{
  "name": "French Vocabulary",
  "parent_folder_id": null,
  "position": 1
}
```

#### Get Folder
```http
GET /folders/{id}
```

#### Update Folder
```http
PATCH /folders/{id}
Content-Type: application/json

{
  "name": "Updated Folder Name",
  "position": 2
}
```

#### Delete Folder
```http
DELETE /folders/{id}
```

#### Get Folder Contents
```http
GET /folders/{id}/contents
```

**Response:**
```json
{
  "id": "folder-uuid",
  "name": "Language Learning",
  "subfolders": [...],
  "decks": [
    {
      "id": "deck-uuid",
      "name": "Spanish Basics",
      "card_count": 50,
      "last_studied": "2024-01-14T15:30:00Z"
    }
  ]
}
```

### 📚 Decks

#### List Decks
```http
GET /decks
```

**Response:**
```json
[
  {
    "id": "deck-uuid",
    "folder_id": "folder-uuid",
    "user_id": "user-uuid",
    "name": "Spanish Verbs",
    "description": "Common Spanish verbs",
    "is_public": false,
    "created_at": "2024-01-10T08:00:00Z",
    "updated_at": "2024-01-15T10:00:00Z",
    "card_count": 100,
    "last_studied": "2024-01-14T15:30:00Z"
  }
]
```

#### Create Deck
```http
POST /decks
Content-Type: application/json

{
  "name": "French Basics",
  "description": "Essential French words and phrases",
  "folder_id": "folder-uuid",
  "is_public": false
}
```

#### Get Deck
```http
GET /decks/{id}
```

#### Get Deck with Statistics
```http
GET /decks/{id}/stats
```

#### Update Deck
```http
PATCH /decks/{id}
Content-Type: application/json

{
  "name": "Updated Deck Name",
  "description": "Updated description",
  "is_public": true
}
```

#### Delete Deck
```http
DELETE /decks/{id}
```

### 📥 CSV Import/Export

#### Import CSV
```http
POST /decks/{id}/csv
Content-Type: text/csv

front,back
Hello,Hola
Good morning,Buenos días
Thank you,Gracias
Goodbye,Adiós
```

**Response:**
```json
{
  "message": "CSV imported successfully",
  "cards_created": 4,
  "cards": [...]
}
```

#### Export CSV
```http
GET /decks/{id}/csv
```

**Response:**
```csv
front,back
Hello,Hola
Good morning,Buenos días
Thank you,Gracias
Goodbye,Adiós
```

### 🃏 Cards

#### List Cards
```http
GET /cards?deck_id={deck_id}
```

**Response:**
```json
[
  {
    "id": "card-uuid",
    "deck_id": "deck-uuid",
    "front": "Hello",
    "back": "Hola",
    "position": 0,
    "created_at": "2024-01-10T08:00:00Z",
    "updated_at": "2024-01-10T08:00:00Z"
  }
]
```

#### Create Card
```http
POST /cards?deck_id={deck_id}
Content-Type: application/json

{
  "front": "Good night",
  "back": "Buenas noches",
  "position": 5
}
```

#### Bulk Create Cards
```http
POST /cards/bulk?deck_id={deck_id}
Content-Type: application/json

[
  {
    "front": "One",
    "back": "Uno"
  },
  {
    "front": "Two",
    "back": "Dos"
  },
  {
    "front": "Three",
    "back": "Tres"
  }
]
```

#### Get Card
```http
GET /cards/{id}
```

#### Update Card
```http
PATCH /cards/{id}
Content-Type: application/json

{
  "front": "Updated front",
  "back": "Updated back"
}
```

#### Delete Card
```http
DELETE /cards/{id}
```

### 📖 Study Sessions

#### List Study Sessions
```http
GET /study/sessions?limit=10
```

**Response:**
```json
[
  {
    "id": "session-uuid",
    "user_id": "user-uuid",
    "deck_id": "deck-uuid",
    "started_at": "2024-01-15T14:00:00Z",
    "completed_at": "2024-01-15T14:30:00Z",
    "cards_studied": 25,
    "cards_correct": 20
  }
]
```

#### Create Study Session
```http
POST /study/sessions
Content-Type: application/json

{
  "deck_id": "deck-uuid"
}
```

#### Get Study Session
```http
GET /study/sessions/{id}
```

#### Complete Study Session
```http
POST /study/sessions/{id}/complete
```

#### Get Session Progress
```http
GET /study/sessions/{id}/progress
```

**Response:**
```json
[
  {
    "id": "progress-uuid",
    "session_id": "session-uuid",
    "card_id": "card-uuid",
    "status": "easy",
    "response_time_ms": 2500,
    "studied_at": "2024-01-15T14:05:00Z"
  }
]
```

#### Record Card Progress
```http
POST /study/sessions/{id}/progress
Content-Type: application/json

{
  "card_id": "card-uuid",
  "status": "medium",
  "response_time_ms": 3000
}
```

**Status options:** `easy`, `medium`, `hard`, `forgot`

## Error Responses

### 400 Bad Request
```json
{
  "error": "Validation error: Field 'name' is required",
  "status": 400
}
```

### 401 Unauthorized
```json
{
  "error": "Unauthorized",
  "status": 401
}
```

### 403 Forbidden
```json
{
  "error": "Forbidden",
  "status": 403
}
```

### 404 Not Found
```json
{
  "error": "Resource not found",
  "status": 404
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal server error",
  "status": 500
}
```

## Rate Limiting
> Not yet implemented. Future versions will include rate limiting headers:
- `X-RateLimit-Limit`: Maximum requests per hour
- `X-RateLimit-Remaining`: Requests remaining
- `X-RateLimit-Reset`: Time when limit resets

## Pagination
> Not yet implemented. Future versions will support pagination parameters:
- `?page=1&limit=20`
- Response headers will include pagination metadata

## WebSocket Events
> Not yet implemented. Future WebSocket endpoints:
- `/ws/study/{session_id}` - Real-time study updates
- `/ws/collaboration/{deck_id}` - Collaborative editing
- `/ws/notifications` - Real-time notifications
