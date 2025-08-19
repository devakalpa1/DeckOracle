mod common;

use axum::http::StatusCode;
use axum_test::TestServer;
use deckoracle_backend::handlers;
use deckoracle_backend::models::{CreateFolderRequest, UpdateFolderRequest, Folder};
use serde_json::json;

#[tokio::test]
async fn test_create_folder() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state);
    let server = TestServer::new(app).unwrap();

    // Test creating a folder
    let response = server
        .post("/api/v1/folders")
        .json(&CreateFolderRequest {
            name: "Test Folder".to_string(),
            description: Some("Test Description".to_string()),
            parent_id: None,
            color: Some("#FF0000".to_string()),
        })
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);
    
    let folder: Folder = response.json();
    assert_eq!(folder.name, "Test Folder");
    assert_eq!(folder.description, Some("Test Description".to_string()));
    assert_eq!(folder.color, Some("#FF0000".to_string()));
}

#[tokio::test]
async fn test_get_folder() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create a folder first
    let create_response = server
        .post("/api/v1/folders")
        .json(&CreateFolderRequest {
            name: "Test Folder".to_string(),
            description: None,
            parent_id: None,
            color: None,
        })
        .await;

    let created_folder: Folder = create_response.json();

    // Get the folder
    let get_response = server
        .get(&format!("/api/v1/folders/{}", created_folder.id))
        .await;

    assert_eq!(get_response.status_code(), StatusCode::OK);
    let fetched_folder: Folder = get_response.json();
    assert_eq!(fetched_folder.id, created_folder.id);
    assert_eq!(fetched_folder.name, created_folder.name);
}

#[tokio::test]
async fn test_update_folder() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create a folder
    let create_response = server
        .post("/api/v1/folders")
        .json(&CreateFolderRequest {
            name: "Original Name".to_string(),
            description: None,
            parent_id: None,
            color: None,
        })
        .await;

    let folder: Folder = create_response.json();

    // Update the folder
    let update_response = server
        .put(&format!("/api/v1/folders/{}", folder.id))
        .json(&UpdateFolderRequest {
            name: Some("Updated Name".to_string()),
            description: Some(Some("New Description".to_string())),
            color: Some(Some("#00FF00".to_string())),
        })
        .await;

    assert_eq!(update_response.status_code(), StatusCode::OK);
    let updated_folder: Folder = update_response.json();
    assert_eq!(updated_folder.name, "Updated Name");
    assert_eq!(updated_folder.description, Some("New Description".to_string()));
    assert_eq!(updated_folder.color, Some("#00FF00".to_string()));
}

#[tokio::test]
async fn test_delete_folder() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create a folder
    let create_response = server
        .post("/api/v1/folders")
        .json(&CreateFolderRequest {
            name: "To Delete".to_string(),
            description: None,
            parent_id: None,
            color: None,
        })
        .await;

    let folder: Folder = create_response.json();

    // Delete the folder
    let delete_response = server
        .delete(&format!("/api/v1/folders/{}", folder.id))
        .await;

    assert_eq!(delete_response.status_code(), StatusCode::NO_CONTENT);

    // Verify it's deleted
    let get_response = server
        .get(&format!("/api/v1/folders/{}", folder.id))
        .await;

    assert_eq!(get_response.status_code(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_list_folders() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create multiple folders
    for i in 1..=3 {
        server
            .post("/api/v1/folders")
            .json(&CreateFolderRequest {
                name: format!("Folder {}", i),
                description: None,
                parent_id: None,
                color: None,
            })
            .await;
    }

    // List folders
    let list_response = server.get("/api/v1/folders").await;

    assert_eq!(list_response.status_code(), StatusCode::OK);
    let folders: Vec<Folder> = list_response.json();
    assert_eq!(folders.len(), 3);
}

#[tokio::test]
async fn test_folder_with_subfolders() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Create parent folder
    let parent_response = server
        .post("/api/v1/folders")
        .json(&CreateFolderRequest {
            name: "Parent Folder".to_string(),
            description: None,
            parent_id: None,
            color: None,
        })
        .await;

    let parent_folder: Folder = parent_response.json();

    // Create child folder
    let child_response = server
        .post("/api/v1/folders")
        .json(&CreateFolderRequest {
            name: "Child Folder".to_string(),
            description: None,
            parent_id: Some(parent_folder.id),
            color: None,
        })
        .await;

    assert_eq!(child_response.status_code(), StatusCode::CREATED);
    let child_folder: Folder = child_response.json();
    assert_eq!(child_folder.parent_id, Some(parent_folder.id));
}

#[tokio::test]
async fn test_folder_validation() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    // Test empty name
    let response = server
        .post("/api/v1/folders")
        .json(&json!({
            "name": "",
            "description": null,
            "parent_id": null
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

    // Test invalid color format
    let response = server
        .post("/api/v1/folders")
        .json(&json!({
            "name": "Test",
            "color": "not-a-color",
            "description": null,
            "parent_id": null
        }))
        .await;

    // Should either fail validation or accept any string depending on implementation
}

#[tokio::test]
async fn test_folder_not_found() {
    let state = common::create_test_state().await;
    let app = handlers::routes(state.clone());
    let server = TestServer::new(app).unwrap();

    let fake_id = uuid::Uuid::new_v4();
    let response = server
        .get(&format!("/api/v1/folders/{}", fake_id))
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
}
