mod common;

use axum::http::StatusCode;
use axum_test::TestServer;
use chrono::Utc;
use deckoracle_backend::handlers;
use deckoracle_backend::models::{
    CreateDeckRequest, CreateCardRequest, CreateStudySessionRequest,
    UpdateStudyProgressRequest, Deck, Card, StudySession, ReviewResponse
};
use serde_json::json;

#[tokio::test]
async fn test_create_study_session() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create a deck with cards
    let deck = create_test_deck_with_cards(&server).await;

    // Create a study session
    let session_response = server
        .post("/api/v1/study-sessions")
        .json(&CreateStudySessionRequest {
            deck_id: deck.id,
            session_type: "practice".to_string(),
            target_card_count: Some(5),
        })
        .await;

    assert_eq!(session_response.status_code(), StatusCode::CREATED);
    
    let session: StudySession = session_response.json();
    assert_eq!(session.deck_id, deck.id);
    assert_eq!(session.session_type, "practice");
    assert!(session.start_time <= Utc::now());
}

#[tokio::test]
async fn test_get_next_card() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    let deck = create_test_deck_with_cards(&server).await;

    // Create a study session
    let session_response = server
        .post("/api/v1/study-sessions")
        .json(&CreateStudySessionRequest {
            deck_id: deck.id,
            session_type: "review".to_string(),
            target_card_count: None,
        })
        .await;

    let session: StudySession = session_response.json();

    // Get next card
    let card_response = server
        .get(&format!("/api/v1/study-sessions/{}/next-card", session.id))
        .await;

    assert_eq!(card_response.status_code(), StatusCode::OK);
    
    let card: Card = card_response.json();
    assert_eq!(card.deck_id, deck.id);
    assert!(!card.front.is_empty());
    assert!(!card.back.is_empty());
}

#[tokio::test]
async fn test_submit_card_review() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    let deck = create_test_deck_with_cards(&server).await;

    // Create a study session
    let session_response = server
        .post("/api/v1/study-sessions")
        .json(&CreateStudySessionRequest {
            deck_id: deck.id,
            session_type: "review".to_string(),
            target_card_count: None,
        })
        .await;

    let session: StudySession = session_response.json();

    // Get a card
    let card_response = server
        .get(&format!("/api/v1/study-sessions/{}/next-card", session.id))
        .await;
    
    let card: Card = card_response.json();

    // Submit review for the card
    let review_response = server
        .post(&format!("/api/v1/study-sessions/{}/cards/{}/review", session.id, card.id))
        .json(&ReviewResponse {
            difficulty: "medium".to_string(),
            time_spent: 5,
            confidence: 3,
        })
        .await;

    assert_eq!(review_response.status_code(), StatusCode::OK);
    
    let result: serde_json::Value = review_response.json();
    assert!(result["success"].as_bool().unwrap());
}

#[tokio::test]
async fn test_complete_study_session() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    let deck = create_test_deck_with_cards(&server).await;

    // Create a study session
    let session_response = server
        .post("/api/v1/study-sessions")
        .json(&CreateStudySessionRequest {
            deck_id: deck.id,
            session_type: "practice".to_string(),
            target_card_count: Some(2),
        })
        .await;

    let session: StudySession = session_response.json();

    // Study 2 cards
    for _ in 0..2 {
        let card_response = server
            .get(&format!("/api/v1/study-sessions/{}/next-card", session.id))
            .await;
        
        if card_response.status_code() == StatusCode::OK {
            let card: Card = card_response.json();
            
            server
                .post(&format!("/api/v1/study-sessions/{}/cards/{}/review", session.id, card.id))
                .json(&ReviewResponse {
                    difficulty: "easy".to_string(),
                    time_spent: 3,
                    confidence: 4,
                })
                .await;
        }
    }

    // Complete the session
    let complete_response = server
        .post(&format!("/api/v1/study-sessions/{}/complete", session.id))
        .await;

    assert_eq!(complete_response.status_code(), StatusCode::OK);
    
    let completed_session: StudySession = complete_response.json();
    assert!(completed_session.end_time.is_some());
    assert!(completed_session.cards_studied >= 0);
}

#[tokio::test]
async fn test_study_session_statistics() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    let deck = create_test_deck_with_cards(&server).await;

    // Create and complete a study session
    let session_response = server
        .post("/api/v1/study-sessions")
        .json(&CreateStudySessionRequest {
            deck_id: deck.id,
            session_type: "review".to_string(),
            target_card_count: Some(3),
        })
        .await;

    let session: StudySession = session_response.json();

    // Study cards with different responses
    let difficulties = vec!["easy", "medium", "hard"];
    for difficulty in &difficulties {
        let card_response = server
            .get(&format!("/api/v1/study-sessions/{}/next-card", session.id))
            .await;
        
        if card_response.status_code() == StatusCode::OK {
            let card: Card = card_response.json();
            
            server
                .post(&format!("/api/v1/study-sessions/{}/cards/{}/review", session.id, card.id))
                .json(&ReviewResponse {
                    difficulty: difficulty.to_string(),
                    time_spent: 5,
                    confidence: 3,
                })
                .await;
        }
    }

    // Get session statistics
    let stats_response = server
        .get(&format!("/api/v1/study-sessions/{}/stats", session.id))
        .await;

    assert_eq!(stats_response.status_code(), StatusCode::OK);
    
    let stats: serde_json::Value = stats_response.json();
    assert!(stats["total_cards"].as_i64().unwrap() > 0);
    assert!(stats["average_time"].as_f64().is_some());
}

#[tokio::test]
async fn test_spaced_repetition_scheduling() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    let deck = create_test_deck_with_cards(&server).await;

    // Create a study session
    let session_response = server
        .post("/api/v1/study-sessions")
        .json(&CreateStudySessionRequest {
            deck_id: deck.id,
            session_type: "review".to_string(),
            target_card_count: None,
        })
        .await;

    let session: StudySession = session_response.json();

    // Get a card
    let card_response = server
        .get(&format!("/api/v1/study-sessions/{}/next-card", session.id))
        .await;
    
    let card: Card = card_response.json();
    let original_review_date = card.next_review_date;

    // Submit an "easy" review (should push review date further)
    server
        .post(&format!("/api/v1/study-sessions/{}/cards/{}/review", session.id, card.id))
        .json(&ReviewResponse {
            difficulty: "easy".to_string(),
            time_spent: 3,
            confidence: 5,
        })
        .await;

    // Get the card again to check updated review date
    let updated_card_response = server
        .get(&format!("/api/v1/cards/{}", card.id))
        .await;
    
    let updated_card: Card = updated_card_response.json();
    
    // Verify the review date was updated
    if original_review_date.is_some() && updated_card.next_review_date.is_some() {
        assert!(updated_card.next_review_date > original_review_date);
    }
    assert_eq!(updated_card.review_count, card.review_count + 1);
}

#[tokio::test]
async fn test_session_with_no_cards() {
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

    // Try to create a study session
    let session_response = server
        .post("/api/v1/study-sessions")
        .json(&CreateStudySessionRequest {
            deck_id: deck.id,
            session_type: "practice".to_string(),
            target_card_count: None,
        })
        .await;

    // Should still create session but with no cards available
    assert_eq!(session_response.status_code(), StatusCode::CREATED);
    
    let session: StudySession = session_response.json();

    // Try to get next card
    let card_response = server
        .get(&format!("/api/v1/study-sessions/{}/next-card", session.id))
        .await;

    // Should return no content or appropriate message
    assert!(
        card_response.status_code() == StatusCode::NO_CONTENT ||
        card_response.status_code() == StatusCode::NOT_FOUND
    );
}

// Helper function to create a deck with test cards
async fn create_test_deck_with_cards(server: &TestServer) -> Deck {
    // Create a deck
    let deck_response = server
        .post("/api/v1/decks")
        .json(&CreateDeckRequest {
            name: "Test Study Deck".to_string(),
            description: Some("Deck for study session testing".to_string()),
            folder_id: None,
            tags: Some(vec!["test".to_string()]),
            is_public: false,
        })
        .await;

    let deck: Deck = deck_response.json();

    // Add test cards
    let cards = vec![
        ("What is Rust?", "A systems programming language"),
        ("What is ownership?", "Rust's memory management model"),
        ("What is a trait?", "Interface definition in Rust"),
        ("What is async/await?", "Asynchronous programming syntax"),
        ("What is a lifetime?", "Reference validity scope"),
    ];

    for (front, back) in cards {
        server
            .post("/api/v1/cards")
            .json(&CreateCardRequest {
                deck_id: deck.id,
                front: front.to_string(),
                back: back.to_string(),
                tags: Some(vec!["rust".to_string(), "programming".to_string()]),
                position: None,
            })
            .await;
    }

    deck
}
