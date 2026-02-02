mod bot;
mod login;
mod session;
mod sync;
mod emoji_verification;
mod transaction;
mod config;

use tracing_subscriber;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(tracing_subscriber::filter::LevelFilter::WARN).init();

    // The folder containing this example's data.
    let data_dir = dirs::data_dir().expect("no data_dir directory found").join("persist_session");
    // The file where the session is persisted.
    let session_file = data_dir.join("session");

    // print out the data_dir and session_file paths
    println!("Data directory: {}", data_dir.display());
    println!("Session file: {}", session_file.display());

    let (client, initial_sync_token) = login::login_or_restore(&data_dir, &session_file).await?;
    sync::sync(client, initial_sync_token, &session_file).await

}
