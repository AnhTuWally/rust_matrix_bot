use std::path::Path;

use matrix_sdk::Client;
use rand::{Rng, distributions::Alphanumeric, thread_rng};
use tokio::fs;

// use the session structs from session.rs
use crate::session::{ClientSession, FullSession};

use crate::config::{ load_config };


pub async fn login_or_restore(data_dir: &Path, session_file: &Path) -> anyhow::Result<(Client, Option<String>)> {
    if session_file.exists() {
        restore_session(session_file).await
    } else {
        let client = login(data_dir, session_file).await?;
        Ok((client, None))
    }
}


/// Restore a previous session.
async fn restore_session(session_file: &Path) -> anyhow::Result<(Client, Option<String>)> {
    println!("Previous session found in '{}'", session_file.to_string_lossy());

    // The session was serialized as JSON in a file.
    let serialized_session = fs::read_to_string(session_file).await?;
    let FullSession { client_session, user_session, sync_token } =
        serde_json::from_str(&serialized_session)?;

    // Build the client with the previous settings from the session.
    let client = Client::builder()
        .homeserver_url(client_session.homeserver)
        .sqlite_store(client_session.db_path, Some(&client_session.passphrase))
        .build()
        .await?;

    println!("Restoring session for {}…", user_session.meta.user_id);

    // Restore the Matrix user session.
    client.restore_session(user_session).await?;

    Ok((client, sync_token))
}

/// Login with a new device.
async fn login(data_dir: &Path, session_file: &Path) -> anyhow::Result<Client> {
    println!("No previous session found, logging in…");
    let config = load_config("config.toml");

    let homeserver_url = &config.matrix.homeserver;

    let (client, client_session) = build_client(data_dir, homeserver_url).await?;
    let matrix_auth = client.matrix_auth();

    let username = config.matrix.username;
    let password = config.matrix.password;


    match matrix_auth
        .login_username(&username, &password)
        .initial_device_display_name("matrix-firefly-iii-bot")
        .await
    {
        Ok(_) => {
            println!("Logged in as {username}");
        }
        Err(error) => {
            println!("Error logging in: {error}");
            println!("Exiting…");
            return Err(error.into());
        }
    }

    let user_session = matrix_auth.session().expect("A logged-in client should have a session");
    let serialized_session: String =
        serde_json::to_string(&FullSession { client_session, user_session, sync_token: None })?;
    fs::write(session_file, serialized_session).await?;

    println!("Session persisted in {}", session_file.to_string_lossy());

    Ok(client)
}

/// Build a new client.
async fn build_client(data_dir: &Path, homeserver_url: &String) -> anyhow::Result<(Client, ClientSession)> {
    let mut rng = thread_rng();

    let db_subfolder: String =
        (&mut rng).sample_iter(Alphanumeric).take(7).map(char::from).collect();
    let db_path = data_dir.join(db_subfolder);

    // Generate a random passphrase.
    let passphrase: String =
        (&mut rng).sample_iter(Alphanumeric).take(32).map(char::from).collect();


    match Client::builder()
        .homeserver_url(homeserver_url)
        .sqlite_store(&db_path, Some(&passphrase))
        .build()
        .await
    {
        Ok(client) => Ok((client, ClientSession { homeserver: homeserver_url.clone(), db_path, passphrase })),
        Err(error) => {
            println!("Error building the client: {error}");
            println!("Exiting…");
            Err(error.into())
        }
    }

 
}
