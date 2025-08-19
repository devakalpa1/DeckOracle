use chrono::Utc;
use csv::Writer;
use sqlx::PgPool;
use std::fmt::Write;
use uuid::Uuid;

use crate::{
    models::{
        Card, Deck,
        import_export::*,
    },
    utils::{error::AppError, Result},
};

pub struct ImportExportService;

impl ImportExportService {
    // Export a single deck
    pub async fn export_deck(
        db: &PgPool,
        user_id: Uuid,
        deck_id: Uuid,
        format: ExportFormat,
        include_progress: bool,
        include_media: bool,
    ) -> Result<Vec<u8>> {
        // Get deck details
        let deck = sqlx::query_as!(
            Deck,
            r#"
            SELECT id, folder_id, owner_id as user_id, title as name, 
                   description, is_public, created_at, updated_at
            FROM decks
            WHERE id = $1 AND owner_id = $2
            "#,
            deck_id,
            user_id
        )
        .fetch_one(db)
        .await
        .map_err(|_| AppError::NotFound("Deck not found".to_string()))?;

        // Get cards for the deck
        let cards = sqlx::query_as!(
            Card,
            r#"
            SELECT id, deck_id, front, back, position, created_at, updated_at
            FROM cards
            WHERE deck_id = $1
            ORDER BY position
            "#,
            deck_id
        )
        .fetch_all(db)
        .await?;

        // Get progress data if requested
        let card_progress = if include_progress {
            Self::get_card_progress(db, user_id, deck_id).await?
        } else {
            vec![]
        };

        // Convert to export format
        match format {
            ExportFormat::Json => Self::export_as_json(deck, cards, card_progress),
            ExportFormat::Csv => Self::export_as_csv(deck, cards),
            ExportFormat::Anki => Self::export_as_anki(deck, cards, card_progress),
            ExportFormat::Markdown => Self::export_as_markdown(deck, cards),
        }
    }

    // Export multiple decks
    pub async fn export_decks(
        db: &PgPool,
        user_id: Uuid,
        deck_ids: Vec<Uuid>,
        format: ExportFormat,
        include_progress: bool,
        include_media: bool,
    ) -> Result<Vec<u8>> {
        let mut all_data = Vec::new();

        for deck_id in deck_ids {
            let deck_data = Self::export_deck(
                db,
                user_id,
                deck_id,
                format.clone(),
                include_progress,
                include_media,
            )
            .await?;
            all_data.extend_from_slice(&deck_data);
        }

        Ok(all_data)
    }

    // Import decks from data
    pub async fn import_decks(
        db: &PgPool,
        user_id: Uuid,
        data: Vec<u8>,
        format: ImportFormat,
        folder_id: Option<Uuid>,
        merge_duplicates: bool,
    ) -> Result<ImportResult> {
        // Validate import data
        let validation = Self::validate_import(&data, &format)?;
        if !validation.is_valid {
            return Ok(ImportResult {
                success: false,
                imported_decks: vec![],
                errors: validation.errors,
                warnings: validation.warnings,
                total_cards_imported: 0,
                total_decks_imported: 0,
            });
        }

        // Parse and import based on format
        match format {
            ImportFormat::Json => Self::import_from_json(db, user_id, data, folder_id, merge_duplicates).await,
            ImportFormat::Csv => Self::import_from_csv(db, user_id, data, folder_id, merge_duplicates).await,
            ImportFormat::Anki => Self::import_from_anki(db, user_id, data, folder_id, merge_duplicates).await,
            ImportFormat::Markdown => Self::import_from_markdown(db, user_id, data, folder_id, merge_duplicates).await,
        }
    }

    // Format-specific export functions
    fn export_as_json(deck: Deck, cards: Vec<Card>, progress: Vec<CardProgressData>) -> Result<Vec<u8>> {
        let exported_cards: Vec<ExportedCard> = cards
            .into_iter()
            .enumerate()
            .map(|(i, card)| ExportedCard {
                id: card.id,
                front: card.front,
                back: card.back,
                explanation: None,
                tags: vec![],
                difficulty: None,
                media: vec![],
                created_at: card.created_at,
                updated_at: card.updated_at,
                progress: progress.get(i).cloned(),
            })
            .collect();

        let total_cards = exported_cards.len();
        let exported_deck = ExportedDeck {
            id: deck.id,
            title: deck.name,
            description: deck.description,
            tags: vec![],
            created_at: deck.created_at,
            updated_at: deck.updated_at,
            cards: exported_cards,
            metadata: ExportMetadata {
                version: "1.0".to_string(),
                exported_at: Utc::now(),
                platform: "DeckOracle".to_string(),
                format: "json".to_string(),
                total_cards,
                includes_progress: !progress.is_empty(),
                includes_media: false,
            },
        };

        let json = serde_json::to_vec_pretty(&exported_deck)?;
        Ok(json)
    }

    fn export_as_csv(_deck: Deck, cards: Vec<Card>) -> Result<Vec<u8>> {
        let mut wtr = Writer::from_writer(vec![]);
        
        // Write header
        wtr.write_record(&["Front", "Back", "Tags", "Explanation", "Difficulty"])?;
        
        // Write cards
        for card in cards {
            let csv_card = CsvCard {
                front: card.front,
                back: card.back,
                tags: String::new(),
                explanation: String::new(),
                difficulty: None,
            };
            
            wtr.write_record(&[
                csv_card.front,
                csv_card.back,
                csv_card.tags,
                csv_card.explanation,
                csv_card.difficulty.map_or(String::new(), |d| d.to_string()),
            ])?;
        }
        
        let data = wtr.into_inner()?;
        Ok(data)
    }

    fn export_as_anki(deck: Deck, cards: Vec<Card>, progress: Vec<CardProgressData>) -> Result<Vec<u8>> {
        // Create Anki model (note type)
        let model = AnkiModel {
            id: 1,
            name: "Basic".to_string(),
            flds: vec![
                AnkiField { name: "Front".to_string(), ord: 0 },
                AnkiField { name: "Back".to_string(), ord: 1 },
            ],
            tmpls: vec![
                AnkiTemplate {
                    name: "Card 1".to_string(),
                    qfmt: "{{Front}}".to_string(),
                    afmt: "{{FrontSide}}<hr id=\"answer\">{{Back}}".to_string(),
                },
            ],
        };

        // Convert cards to Anki format
        let anki_notes: Vec<AnkiNote> = cards
            .iter()
            .enumerate()
            .map(|(i, card)| AnkiNote {
                id: i as i64 + 1,
                guid: card.id.to_string(),
                mid: 1,
                fields: vec![card.front.clone(), card.back.clone()],
                tags: vec![],
            })
            .collect();

        let anki_cards: Vec<AnkiCard> = cards
            .iter()
            .enumerate()
            .map(|(i, _card)| {
                let progress = progress.get(i);
                AnkiCard {
                    nid: i as i64 + 1,
                    ord: 0,
                    did: 1,
                    due: 0,
                    ivl: progress.map_or(0, |p| p.interval_days),
                    factor: progress.map_or(2500, |p| (p.ease_factor * 1000.0) as i32),
                    reps: progress.map_or(0, |p| p.review_count),
                    lapses: 0,
                }
            })
            .collect();

        let anki_deck = AnkiDeck {
            name: deck.name,
            desc: deck.description.unwrap_or_default(),
            cards: anki_cards,
            notes: anki_notes,
            models: vec![model],
        };

        // For now, return JSON representation
        // In production, this would create a proper .apkg file
        let json = serde_json::to_vec(&anki_deck)?;
        Ok(json)
    }

    fn export_as_markdown(deck: Deck, cards: Vec<Card>) -> Result<Vec<u8>> {
        let mut markdown = String::new();
        
        // Write deck header
        writeln!(markdown, "# {}", deck.name)?;
        if let Some(desc) = deck.description {
            writeln!(markdown, "\n{}\n", desc)?;
        }
        writeln!(markdown, "---\n")?;

        // Write cards
        for (i, card) in cards.iter().enumerate() {
            writeln!(markdown, "## Card {}", i + 1)?;
            writeln!(markdown, "\n**Front:** {}", card.front)?;
            writeln!(markdown, "\n**Back:** {}", card.back)?;
            writeln!(markdown, "\n---\n")?;
        }

        Ok(markdown.into_bytes())
    }

    // Format-specific import functions
    async fn import_from_json(
        db: &PgPool,
        user_id: Uuid,
        data: Vec<u8>,
        folder_id: Option<Uuid>,
        merge_duplicates: bool,
    ) -> Result<ImportResult> {
        let exported_deck: ExportedDeck = serde_json::from_slice(&data)?;
        
        let mut tx = db.begin().await?;
        
        // Check if deck with same name exists
        let existing_deck = sqlx::query!(
            "SELECT id FROM decks WHERE owner_id = $1 AND title = $2",
            user_id,
            exported_deck.title
        )
        .fetch_optional(&mut *tx)
        .await?;

        let deck_id = if let Some(ref existing) = existing_deck {
            if !merge_duplicates {
                return Err(AppError::BadRequest("Deck with same name already exists".to_string()));
            }
            existing.id
        } else {
            // Create new deck
            let new_deck_id = Uuid::new_v4();
            sqlx::query!(
                r#"
                INSERT INTO decks (id, owner_id, folder_id, title, description, is_public, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                "#,
                new_deck_id,
                user_id,
                folder_id,
                exported_deck.title,
                exported_deck.description,
                false,
                Utc::now(),
                Utc::now()
            )
            .execute(&mut *tx)
            .await?;
            new_deck_id
        };

        // Import cards
        let mut imported_cards = 0;
        for (position, card) in exported_deck.cards.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO cards (id, deck_id, front, back, position, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (id) DO NOTHING
                "#,
                Uuid::new_v4(),
                deck_id,
                card.front,
                card.back,
                position as i32,
                Utc::now(),
                Utc::now()
            )
            .execute(&mut *tx)
            .await?;
            imported_cards += 1;
        }

        tx.commit().await?;

        Ok(ImportResult {
            success: true,
            imported_decks: vec![ImportedDeck {
                id: deck_id,
                title: exported_deck.title,
                card_count: imported_cards,
                was_merged: existing_deck.is_some(),
            }],
            errors: vec![],
            warnings: vec![],
            total_cards_imported: imported_cards,
            total_decks_imported: 1,
        })
    }

    async fn import_from_csv(
        db: &PgPool,
        user_id: Uuid,
        data: Vec<u8>,
        folder_id: Option<Uuid>,
        _merge_duplicates: bool,
    ) -> Result<ImportResult> {
        let mut rdr = csv::Reader::from_reader(&data[..]);
        let mut cards = Vec::new();

        for result in rdr.records() {
            let record = result?;
            if record.len() >= 2 {
                cards.push(CsvCard {
                    front: record[0].to_string(),
                    back: record[1].to_string(),
                    tags: record.get(2).unwrap_or("").to_string(),
                    explanation: record.get(3).unwrap_or("").to_string(),
                    difficulty: record.get(4).and_then(|s| s.parse().ok()),
                });
            }
        }

        // Create a new deck for CSV import
        let deck_id = Uuid::new_v4();
        let deck_title = format!("Imported Deck {}", Utc::now().format("%Y-%m-%d"));

        let mut tx = db.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO decks (id, owner_id, folder_id, title, description, is_public, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            deck_id,
            user_id,
            folder_id,
            deck_title,
            Some("Imported from CSV".to_string()),
            false,
            Utc::now(),
            Utc::now()
        )
        .execute(&mut *tx)
        .await?;

        // Import cards
        for (position, card) in cards.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO cards (id, deck_id, front, back, position, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                Uuid::new_v4(),
                deck_id,
                card.front,
                card.back,
                position as i32,
                Utc::now(),
                Utc::now()
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(ImportResult {
            success: true,
            imported_decks: vec![ImportedDeck {
                id: deck_id,
                title: deck_title.clone(),
                card_count: cards.len(),
                was_merged: false,
            }],
            errors: vec![],
            warnings: vec![],
            total_cards_imported: cards.len(),
            total_decks_imported: 1,
        })
    }

    async fn import_from_anki(
        db: &PgPool,
        user_id: Uuid,
        data: Vec<u8>,
        folder_id: Option<Uuid>,
        _merge_duplicates: bool,
    ) -> Result<ImportResult> {
        // Parse Anki JSON (simplified - real implementation would handle .apkg files)
        let anki_deck: AnkiDeck = serde_json::from_slice(&data)?;

        let deck_id = Uuid::new_v4();
        let mut tx = db.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO decks (id, owner_id, folder_id, title, description, is_public, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            deck_id,
            user_id,
            folder_id,
            anki_deck.name,
            Some(anki_deck.desc),
            false,
            Utc::now(),
            Utc::now()
        )
        .execute(&mut *tx)
        .await?;

        // Import notes as cards
        for (position, note) in anki_deck.notes.iter().enumerate() {
            if note.fields.len() >= 2 {
                sqlx::query!(
                    r#"
                    INSERT INTO cards (id, deck_id, front, back, position, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    "#,
                    Uuid::new_v4(),
                    deck_id,
                    note.fields[0],
                    note.fields[1],
                    position as i32,
                    Utc::now(),
                    Utc::now()
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        Ok(ImportResult {
            success: true,
            imported_decks: vec![ImportedDeck {
                id: deck_id,
                title: anki_deck.name.clone(),
                card_count: anki_deck.notes.len(),
                was_merged: false,
            }],
            errors: vec![],
            warnings: vec![],
            total_cards_imported: anki_deck.notes.len(),
            total_decks_imported: 1,
        })
    }

    async fn import_from_markdown(
        db: &PgPool,
        user_id: Uuid,
        data: Vec<u8>,
        folder_id: Option<Uuid>,
        merge_duplicates: bool,
    ) -> Result<ImportResult> {
        let content = String::from_utf8(data)?;
        let lines: Vec<&str> = content.lines().collect();
        
        let mut deck_title = "Imported from Markdown".to_string();
        let mut deck_description: Option<String> = None;
        let mut cards = Vec::new();
        let mut current_card: Option<(String, String)> = None;
        let mut in_front = false;
        let mut in_back = false;

        for line in lines {
            if line.starts_with("# ") {
                deck_title = line[2..].trim().to_string();
            } else if line.starts_with("## Card") {
                if let Some((front, back)) = current_card.take() {
                    cards.push((front, back));
                }
                current_card = Some((String::new(), String::new()));
            } else if line.starts_with("**Front:**") {
                in_front = true;
                in_back = false;
                if let Some((ref mut front, _)) = current_card {
                    *front = line[10..].trim().to_string();
                }
            } else if line.starts_with("**Back:**") {
                in_front = false;
                in_back = true;
                if let Some((_, ref mut back)) = current_card {
                    *back = line[9..].trim().to_string();
                }
            }
        }

        // Add last card if exists
        if let Some((front, back)) = current_card {
            cards.push((front, back));
        }

        // Create deck and cards
        let deck_id = Uuid::new_v4();
        let mut tx = db.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO decks (id, owner_id, folder_id, title, description, is_public, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            deck_id,
            user_id,
            folder_id,
            deck_title,
            deck_description,
            false,
            Utc::now(),
            Utc::now()
        )
        .execute(&mut *tx)
        .await?;

        for (position, (front, back)) in cards.iter().enumerate() {
            sqlx::query!(
                r#"
                INSERT INTO cards (id, deck_id, front, back, position, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
                Uuid::new_v4(),
                deck_id,
                front,
                back,
                position as i32,
                Utc::now(),
                Utc::now()
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(ImportResult {
            success: true,
            imported_decks: vec![ImportedDeck {
                id: deck_id,
                title: deck_title.clone(),
                card_count: cards.len(),
                was_merged: false,
            }],
            errors: vec![],
            warnings: vec![],
            total_cards_imported: cards.len(),
            total_decks_imported: 1,
        })
    }

    // Helper functions
    async fn get_card_progress(
        _db: &PgPool,
        _user_id: Uuid,
        _deck_id: Uuid,
    ) -> Result<Vec<CardProgressData>> {
        // Query card progress from database
        // This is a simplified version
        Ok(vec![])
    }

    pub fn validate_import(data: &[u8], format: &ImportFormat) -> Result<ImportValidationResult> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut deck_count = 0;
        let mut card_count = 0;

        match format {
            ImportFormat::Json => {
                match serde_json::from_slice::<ExportedDeck>(data) {
                    Ok(deck) => {
                        deck_count = 1;
                        card_count = deck.cards.len();
                        if deck.cards.is_empty() {
                            warnings.push("Deck contains no cards".to_string());
                        }
                    }
                    Err(e) => {
                        errors.push(format!("Invalid JSON format: {}", e));
                    }
                }
            }
            ImportFormat::Csv => {
                let rdr = csv::Reader::from_reader(data);
                card_count = rdr.into_records().count();
                deck_count = 1;
                if card_count == 0 {
                    errors.push("CSV file contains no valid records".to_string());
                }
            }
            ImportFormat::Anki => {
                match serde_json::from_slice::<AnkiDeck>(data) {
                    Ok(deck) => {
                        deck_count = 1;
                        card_count = deck.notes.len();
                        if deck.notes.is_empty() {
                            warnings.push("Anki deck contains no notes".to_string());
                        }
                    }
                    Err(e) => {
                        errors.push(format!("Invalid Anki format: {}", e));
                    }
                }
            }
            ImportFormat::Markdown => {
                if let Ok(content) = String::from_utf8(data.to_vec()) {
                    deck_count = 1;
                    card_count = content.matches("## Card").count();
                    if card_count == 0 {
                        warnings.push("Markdown file contains no cards".to_string());
                    }
                } else {
                    errors.push("Invalid UTF-8 encoding in Markdown file".to_string());
                }
            }
        }

        Ok(ImportValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            deck_count,
            card_count,
        })
    }
}
