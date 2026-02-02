use matrix_sdk::ruma::events::AnyMessageLikeEventContent;
use matrix_sdk::{Room, RoomState};
use matrix_sdk::ruma::events::{
    reaction::ReactionEventContent,
    relation::Annotation,
    room::message::{MessageType, OriginalSyncRoomMessageEvent},
};

use regex::Regex;
use chrono::Local;

use crate::transaction::{ FireflyClient, Transaction };
use crate::config::{ load_config };


fn parse_spend_message(message: &str) -> Option<Transaction> {
    // Define a regex pattern to capture amount, description, and optional note
    let re = Regex::new(r"^[Ss]pend\s+(\d+(?:\.\d{1,2})?)\s+(.+?)(?:\s+[Nn]ote:?\s+(.+))?$").unwrap();

    let caps = re.captures(message.trim())?;

    // if no captures, return None
    if caps.len() < 2 {
        return None;
    }

    let amount_str = caps.get(1)?.as_str();
    let mut description = caps.get(2)?.as_str().to_string();
    // remove on or for if exists at the start of description

    if description.to_lowercase().starts_with("on ") {
        description = description[3..].to_string();
    } else if description.to_lowercase().starts_with("for ") {
        description = description[4..].to_string();
    }

    // remove the ending punctuation from description
    if description.ends_with('.') || description.ends_with('!') || description.ends_with('?') {
        description.pop();
    }

    if description.is_empty() {
        return None;
    }

    let note = caps.get(3).map(|m| m.as_str().to_string());

    let today = Local::now().date_naive();
    let today_str = today.format("%Y-%m-%d").to_string();

    Some(Transaction {
        amount: amount_str.parse().ok()?,
        description: description,
        note: note,
        date: Some(today_str),
        transaction_type: Some("withdrawal".to_string()),
    })
}



pub async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    if room.state() != RoomState::Joined {
        return;
    }
    let MessageType::Text(text_content) = event.content.msgtype else {
        return;
    };

    let content = text_content.body;

    // if not start with "spend", ignore
    
    if !content.to_lowercase().starts_with("spend") {
        return;
    }

    let mut process_sucessed = false; // Simulate some processing logic

    let spend_data_opt = parse_spend_message(&content);

    // if spend_data_opt is None, parsing failed
    if let Some(spend_data) = spend_data_opt {
        println!(
            "Parsed spend data: amount={}, description='{}', note='{:?}'",
            spend_data.amount, spend_data.description, spend_data.note
        );

        let config = load_config("config.toml");
        let firefly_client = config.firefly.base_url;
        let firefly_token = config.firefly.token;

        let firefly_client = FireflyClient::new(firefly_client, firefly_token);

        match firefly_client.create_transaction(&spend_data).await {
            Ok(success) => {
                if success {
                    println!("Transaction created successfully in Firefly III.");
                    process_sucessed = true; // Simulate successful processing
                } else {
                    println!("Failed to create transaction in Firefly III.");
                }
            }
            Err(e) => {
                eprintln!("Error creating transaction in Firefly III: {}", e);
            }
        }
    } else {
        println!("Failed to parse spend message: {}", content);
    }


    let success_annotation = Annotation::new(event.event_id.clone(), "✅".to_string());
    let fail_annotation = Annotation::new(event.event_id.clone(), "❌".to_string());

    
    // Choose the appropriate annotation based on processing result
    let annotation = if process_sucessed {
        success_annotation
    } else {
        fail_annotation
    };

    let reaction_event_content = ReactionEventContent::new(annotation);

    let reaction_content = AnyMessageLikeEventContent::Reaction(reaction_event_content);

    if let Err(e) = room.send(reaction_content).await {
        eprintln!("Failed to send reaction: {}", e);
    } else {
        println!("Reaction sent successfully");
    }
}