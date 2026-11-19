use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, sqlx::Type, PartialEq, Debug)]
#[sqlx(type_name = "seat_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
enum SeatStatus {
    Available,
    Held,
    Booked,
}

#[derive(Serialize, sqlx::FromRow)]
struct Seat {
    id: Uuid,
    label: String,
    price_pence: i32,
}

// Actions:
//
// see seats - all available seats for an event (or all seats and their status), join on hold and booking tables for availability
// hold seats - create hold object and rows in seats held
// book seats - create booking object and rows in seats booked, delete hold object and rows
//
// later
// un hold seats
// see bookings
// cancel booking
// see events
//
// Objects: events, seat, hold, booking
pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/events/{id}/seats", get(list_seats))
        //.route("/holds/", post(hold_seat))
        //.route("/bookings/", post(book_seat))
        .with_state(pool)
}

async fn list_seats(
    State(pool): State<PgPool>,
    Path(event_id): Path<Uuid>,
) -> Result<Json<Vec<Seat>>, String> {
    // An expired hold is reported as available, matching the hold query's rules
    let seats = sqlx::query_as::<_, Seat>(
        "SELECT id, label, price_pence
         FROM seats WHERE event_id = $1 ORDER BY label",
    )
    .bind(event_id)
    .fetch_all(&pool)
    .await
    .unwrap();
    Ok(Json(seats))
}
