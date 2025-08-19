mod common;

use axum::http::{header, StatusCode};
use axum_test::TestServer;
use deckoracle_backend::handlers;
use deckoracle_backend::models::{CreateDeckRequest, CreateCardRequest, Deck, Card};
use serde_json::json;

#[tokio::test]
async fn test_csv_export() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create a deck
    let deck_response = server
        .post("/api/v1/decks")
        .json(&CreateDeckRequest {
            name: "Export Test Deck".to_string(),
            description: Some("Deck for CSV export testing".to_string()),
            folder_id: None,
            tags: Some(vec!["test".to_string(), "csv".to_string()]),
            is_public: false,
        })
        .await;

    let deck: Deck = deck_response.json();

    // Add cards to the deck
    let cards_data = vec![
        ("What is Rust?", "A systems programming language", vec!["programming"]),
        ("What is memory safety?", "Protection against memory errors", vec!["concepts"]),
        ("What is ownership?", "Rust's memory management model", vec!["rust", "memory"]),
    ];

    for (front, back, tags) in cards_data {
        server
            .post("/api/v1/cards")
            .json(&CreateCardRequest {
                deck_id: deck.id,
                front: front.to_string(),
                back: back.to_string(),
                tags: Some(tags.iter().map(|s| s.to_string()).collect()),
                position: None,
            })
            .await;
    }

    // Export to CSV
    let export_response = server
        .get(&format!("/api/v1/decks/{}/export", deck.id))
        .await;

    assert_eq!(export_response.status_code(), StatusCode::OK);
    assert_eq!(
        export_response.headers().get(header::CONTENT_TYPE),
        Some(&"text/csv".parse().unwrap())
    );

    let csv_content = export_response.text();
    assert!(csv_content.contains("front,back,tags"));
    assert!(csv_content.contains("What is Rust?"));
    assert!(csv_content.contains("A systems programming language"));
    assert!(csv_content.contains("programming"));
}

#[tokio::test]
async fn test_csv_import() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create a deck
    let deck_response = server
        .post("/api/v1/decks")
        .json(&CreateDeckRequest {
            name: "Import Test Deck".to_string(),
            description: Some("Deck for CSV import testing".to_string()),
            folder_id: None,
            tags: None,
            is_public: false,
        })
        .await;

    let deck: Deck = deck_response.json();

    // Prepare CSV content
    let csv_content = r#"front,back,tags
"What is TypeScript?","JavaScript with types","programming,web"
"What is React?","A JavaScript library for UI","frontend,library"
"What is Node.js?","JavaScript runtime","backend,runtime"
"#;

    // Import CSV
    let import_response = server
        .post(&format!("/api/v1/decks/{}/import", deck.id))
        .header(header::CONTENT_TYPE, "text/csv")
        .text(csv_content)
        .await;

    assert_eq!(import_response.status_code(), StatusCode::OK);
    
    let import_result: serde_json::Value = import_response.json();
    assert_eq!(import_result["imported_count"], 3);

    // Verify cards were imported
    let cards_response = server
        .get(&format!("/api/v1/decks/{}/cards", deck.id))
        .await;

    let cards: Vec<Card> = cards_response.json();
    assert_eq!(cards.len(), 3);
    
    // Verify card content
    let typescript_card = cards.iter().find(|c| c.front.contains("TypeScript")).unwrap();
    assert!(typescript_card.back.contains("JavaScript with types"));
    assert!(typescript_card.tags.as_ref().unwrap().contains(&"programming".to_string()));
    assert!(typescript_card.tags.as_ref().unwrap().contains(&"web".to_string()));
}

#[tokio::test]
async fn test_csv_import_with_special_characters() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create a deck
    let deck_response = server
        .post("/api/v1/decks")
        .json(&CreateDeckRequest {
            name: "Special Chars Test".to_string(),
            description: None,
            folder_id: None,
            tags: None,
            is_public: false,
        })
        .await;

    let deck: Deck = deck_response.json();

    // CSV with special characters, quotes, and newlines
    let csv_content = r#"front,back,tags
"What is ""escaping""?","Using \"" to escape quotes","quotes,csv"
"Multi-line
question?","Multi-line
answer","formatting"
"Unicode: café ☕","Coffee in French","unicode,language"
"#;

    let import_response = server
        .post(&format!("/api/v1/decks/{}/import", deck.id))
        .header(header::CONTENT_TYPE, "text/csv")
        .text(csv_content)
        .await;

    assert_eq!(import_response.status_code(), StatusCode::OK);
    
    // Verify special characters were preserved
    let cards_response = server
        .get(&format!("/api/v1/decks/{}/cards", deck.id))
        .await;

    let cards: Vec<Card> = cards_response.json();
    
    let unicode_card = cards.iter().find(|c| c.front.contains("café")).unwrap();
    assert!(unicode_card.front.contains("☕"));
    assert!(unicode_card.back.contains("Coffee in French"));
}

#[tokio::test]
async fn test_csv_import_validation() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create a deck
    let deck_response = server
        .post("/api/v1/decks")
        .json(&CreateDeckRequest {
            name: "Validation Test".to_string(),
            description: None,
            folder_id: None,
            tags: None,
            is_public: false,
        })
        .await;

    let deck: Deck = deck_response.json();

    // Invalid CSV - missing required columns
    let invalid_csv = r#"question,answer
"Test","Test"
"#;

    let import_response = server
        .post(&format!("/api/v1/decks/{}/import", deck.id))
        .header(header::CONTENT_TYPE, "text/csv")
        .text(invalid_csv)
        .await;

    assert_eq!(import_response.status_code(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_csv_export_empty_deck() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create an empty deck
    let deck_response = server
        .post("/api/v1/decks")
        .json(&CreateDeckRequest {
            name: "Empty Deck".to_string(),
            description: None,
            folder_id: None,
            tags: None,
            is_public: false,
        })
        .await;

    let deck: Deck = deck_response.json();

    // Export empty deck
    let export_response = server
        .get(&format!("/api/v1/decks/{}/export", deck.id))
        .await;

    assert_eq!(export_response.status_code(), StatusCode::OK);
    
    let csv_content = export_response.text();
    // Should contain headers but no data rows
    assert!(csv_content.contains("front,back,tags"));
    let lines: Vec<&str> = csv_content.lines().collect();
    assert_eq!(lines.len(), 1); // Only header line
}

#[tokio::test]
async fn test_csv_import_large_batch() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create a deck
    let deck_response = server
        .post("/api/v1/decks")
        .json(&CreateDeckRequest {
            name: "Large Import Test".to_string(),
            description: None,
            folder_id: None,
            tags: None,
            is_public: false,
        })
        .await;

    let deck: Deck = deck_response.json();

    // Generate large CSV
    let mut csv_content = String::from("front,back,tags\n");
    for i in 1..=100 {
        csv_content.push_str(&format!(
            "\"Question {}\",\"Answer {}\",\"tag{},batch\"\n",
            i, i, i % 10
        ));
    }

    let import_response = server
        .post(&format!("/api/v1/decks/{}/import", deck.id))
        .header(header::CONTENT_TYPE, "text/csv")
        .text(&csv_content)
        .await;

    assert_eq!(import_response.status_code(), StatusCode::OK);
    
    let import_result: serde_json::Value = import_response.json();
    assert_eq!(import_result["imported_count"], 100);

    // Verify all cards were imported
    let cards_response = server
        .get(&format!("/api/v1/decks/{}/cards", deck.id))
        .add_query_param("limit", "200")
        .await;

    let cards: Vec<Card> = cards_response.json();
    assert_eq!(cards.len(), 100);
}

#[tokio::test]
async fn test_deck_statistics_after_import() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create a deck
    let deck_response = server
        .post("/api/v1/decks")
        .json(&CreateDeckRequest {
            name: "Stats Test Deck".to_string(),
            description: None,
            folder_id: None,
            tags: None,
            is_public: false,
        })
        .await;

    let deck: Deck = deck_response.json();

    // Import some cards
    let csv_content = r#"front,back,tags
"Q1","A1","tag1"
"Q2","A2","tag1,tag2"
"Q3","A3","tag2,tag3"
"#;

    server
        .post(&format!("/api/v1/decks/{}/import", deck.id))
        .header(header::CONTENT_TYPE, "text/csv")
        .text(csv_content)
        .await;

    // Get deck with statistics
    let deck_response = server
        .get(&format!("/api/v1/decks/{}", deck.id))
        .await;

    let deck_with_stats: Deck = deck_response.json();
    assert_eq!(deck_with_stats.card_count, Some(3));
}
