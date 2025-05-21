mod data;
mod description;
mod store;
mod title;

use tokio::net::TcpListener;

use data::TicketDraft;
use std::sync::Arc;
use std::sync::RwLock;
use store::TicketId;
use store::TicketStore;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

pub type Store = Arc<RwLock<TicketStore>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let ticket_store = Arc::new(RwLock::new(TicketStore::new()));

    let app = Router::new()
        .route("/", get(root))
        .route("/ticket", post(create_ticket))
        .route("/ticket/{id}", get(get_ticket).patch(update_ticket))
        .with_state(ticket_store);

    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    println!("hello");
    "Hello, World!"
}

async fn create_ticket(
    State(store): State<Store>,
    Json(ticket_draft): Json<TicketDraft>,
) -> impl IntoResponse {
    let ticket_id = store.write().unwrap().add_ticket(ticket_draft);
    (StatusCode::CREATED, Json(ticket_id))
}

async fn get_ticket(
    Path(id): Path<u64>,
    State(store): State<Store>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("id = {}", id);
    let ticket_id = TicketId(id);
    let ticket = store.read().unwrap().get(ticket_id);
    match ticket {
        Some(ticket) => {
            let ticket = ticket.read().unwrap();
            let ticket = ticket.clone();
            println!("{:?}", ticket);
            Ok((StatusCode::OK, Json(ticket)))
        }
        None => {
            println!("id={} not found", id);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn update_ticket(
    Path(id): Path<u64>,
    State(store): State<Store>,
    Json(ticket_draft): Json<TicketDraft>,
) -> Result<impl IntoResponse, StatusCode> {
    println!("id = {}", id);
    let ticket_id = TicketId(id);
    let ticket = store.read().unwrap().get(ticket_id);
    match ticket {
        Some(ticket) => {
            let mut ticket = ticket.write().unwrap();
            ticket.title = ticket_draft.title;
            ticket.description = ticket_draft.description;
            let ticket = ticket.clone();
            println!("{:?}", ticket);
            Ok((StatusCode::OK, Json(ticket)))
        }
        None => {
            println!("id={} not found", id);
            Err(StatusCode::NOT_FOUND)
        }
    }
}
