# Backend Implementation Guide - Agent 1

## 🎯 Implementation Checklist to Match Frontend

### Phase 1: Critical Fixes (MUST DO NOW)

#### 1. ✅ Apply Database Migration
```bash
# Run the performance optimization migration
sqlx migrate run
```
Files: `migrations/006_performance_indexes.sql`

#### 2. ✅ Update Study Handler
Apply changes from `study_updated.patch`:
- Update `RecordProgressDto` to accept `user_answer` and `is_correct`
- Update `models/mod.rs` SubmitCardAnswerDto

#### 3. ✅ Add Stub Endpoints
Add to `main.rs`:
```rust
// In api_routes function, add:
.nest("/study", handlers::study::routes())
// ADD THESE:
.nest("/study", handlers::stats_stub::routes())  
.nest("/progress", handlers::stats_stub::progress_stub::routes())
```

Create `handlers/stats_stub.rs` from provided file.

#### 4. ✅ Fix Card Pagination
Update `handlers/card.rs`:
- Add `limit` and `offset` to `CardsQuery`
- Return `PaginatedResponse<Card>` instead of `Vec<Card>`
- Add pagination logic from `card_pagination.patch`

### Phase 2: Service Layer Updates

#### 1. Card Service Pagination
```rust
// services/card.rs - Add these methods:
impl CardService {
    pub async fn count_deck_cards(
        db: &PgPool,
        deck_id: Uuid,
        user_id: Uuid,
    ) -> Result<i32> {
        // Implementation
    }
    
    pub async fn list_deck_cards_paginated(
        db: &PgPool,
        deck_id: Uuid,
        user_id: Uuid,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Card>> {
        // Implementation with LIMIT and OFFSET
    }
}
```

#### 2. Study Service Extension
```rust
// services/study.rs - Add method to handle extra fields:
pub async fn record_card_progress_extended(
    db: &PgPool,
    session_id: Uuid,
    card_id: Uuid,
    user_id: Uuid,
    status: CardStatus,
    response_time_ms: Option<i32>,
    user_answer: Option<String>,
    is_correct: Option<bool>,
) -> Result<CardProgress> {
    // Store user_answer and is_correct in card_progress table
}
```

### Phase 3: Model Updates

#### 1. Add PaginatedResponse to models/mod.rs
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i32,
    pub page: i32,
    pub page_size: i32,
    pub has_next: bool,
    pub has_previous: bool,
}
```

#### 2. Ensure Proper JSON Serialization
```rust
// In DeckWithStats - ensure proper field naming:
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckWithStats {
    #[serde(flatten)]
    pub deck: Deck,
    pub card_count: i64,
    #[serde(rename = "lastStudied")]  // Frontend expects camelCase
    pub last_studied: Option<DateTime<Utc>>,
}
```

### Testing Commands

```bash
# Test pagination
curl "http://localhost:8080/api/v1/cards?deck_id=UUID&limit=10&offset=0"

# Test stats stub
curl -H "Authorization: Bearer TOKEN" "http://localhost:8080/api/v1/study/stats"

# Test progress recording with new fields
curl -X POST "http://localhost:8080/api/v1/study/sessions/UUID/progress" \
  -H "Content-Type: application/json" \
  -d '{
    "card_id": "UUID",
    "status": "easy",
    "response_time_ms": 1500,
    "user_answer": "test answer",
    "is_correct": true
  }'
```

## 🔄 Coordination Points

### What Frontend Expects (Agent 2 Implemented):
1. ✅ Pagination on `/cards` endpoint
2. ✅ Empty but valid `/study/stats` and `/study/achievements`
3. ✅ `/progress/*` endpoints return empty arrays/objects
4. ✅ Study progress accepts `user_answer` and `is_correct`
5. ✅ Error format: `{ "error": "message", "status": 400 }`

### What Backend Must Provide:
1. ⏳ Paginated card responses with metadata
2. ⏳ Stub endpoints that don't 404
3. ⏳ Accept extra study session fields
4. ⏳ Consistent error responses

## 📊 Success Metrics

After implementation:
- [ ] No 404 errors in browser console
- [ ] Cards load with pagination (max 100 per request)
- [ ] Study progress saves with user answers
- [ ] Stats/achievements return valid JSON (even if empty)
- [ ] Database queries < 100ms for typical operations

## 🚀 Quick Implementation Script

```bash
# 1. Apply migration
cd backend
sqlx migrate run

# 2. Apply patches (manually copy code from .patch files)
# 3. Update main.rs to include new routes
# 4. Test endpoints

cargo build
cargo run
```

## 🐛 Common Issues & Solutions

1. **Migration fails**: Check PostgreSQL is running and connection string is correct
2. **Pagination returns wrong page**: Ensure offset calculation is `(page - 1) * limit`
3. **Stats endpoint 404**: Ensure routes are added to main.rs
4. **Card progress fails**: Check if user_answer and is_correct columns exist in DB

## ✅ Verification Checklist

- [ ] Database migration applied successfully
- [ ] All endpoints return 200/201 (no 404s)
- [ ] Pagination works on cards endpoint
- [ ] Study progress accepts all fields
- [ ] Error responses match expected format
- [ ] Performance: deck listing < 200ms
- [ ] Performance: card pagination < 100ms

## 📝 Notes for Phase 3

Future improvements (not critical now):
- Implement actual stats calculation
- Add WebSocket support for real-time updates
- Implement caching layer with Redis
- Add comprehensive integration tests
- Optimize queries with materialized views
