use sqlx::PgPool;
use uuid::Uuid;

use axum::{body::Body, http::Request};
use tower::util::ServiceExt;

#[sqlx::test(migrations = "./migrations")]
async fn hello_ticketing(pool: PgPool) {
    let event_id = Uuid::new_v4();
    let seat_id = Uuid::new_v4();
    sqlx::query("INSERT INTO events (id, name, starts_at) VALUES ($1, 'Test', now())")
        .bind(event_id)
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO seats (id, event_id, label, price_pence) VALUES ($1, $2, 'A1', 5000)")
        .bind(seat_id)
        .bind(event_id)
        .execute(&pool)
        .await
        .unwrap();

    let app = ticketing::router(pool);
    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/events/{event_id}/seats"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}
