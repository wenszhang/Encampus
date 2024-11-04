use sqlx::postgres::PgPool;
use tokio::sync::broadcast;

/// Listen for PostgreSQL notifications and broadcast them to WebSocket clients.
pub async fn listen_for_db_notifications(pool: PgPool, tx: broadcast::Sender<String>) {
    let mut conn = pool
        .acquire()
        .await
        .expect("Could not acquire DB connection");

    sqlx::query("LISTEN announcements")
        .execute(&mut conn)
        .await
        .expect("Could not start listening for announcements");

    sqlx::query("LISTEN posts")
        .execute(&mut conn)
        .await
        .expect("Could not start listening for posts");

    let mut listener = sqlx::postgres::PgListener::connect_with(&pool)
        .await
        .expect("Failed to create listener");

    listener
        .listen("announcements")
        .await
        .expect("Failed to listen to announcements");
    listener
        .listen("posts")
        .await
        .expect("Failed to listen to posts");

    while let Ok(notification) = listener.recv().await {
        let channel = notification.channel();
        let payload = notification.payload();

        // Broadcast different messages depending on the channel
        let message = match channel {
            "announcements" => format!("New announcement: {}", payload),
            "posts" => format!("New post: {}", payload),
            _ => continue,
        };

        let _ = tx.send(message);
    }
}
