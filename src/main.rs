use axum::{extract::State, response::IntoResponse, routing::get, Router};
use rand::Rng;
use std::net::SocketAddr;

#[derive(Clone)]
struct AppState {
    questions: Vec<String>,
}

#[tokio::main]
async fn main() {
    // TODO: putting the Questions in String is not optimal but im too lazy to deal with lifetimes
    // now.
    // As the Size of the Array is also known at Compile Time we dont need a Vec either
    let questions = include_str!("../questions.txt")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let state = AppState { questions };

    let app = Router::new()
        .route("/", get(get_question))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_question(State(state): State<AppState>) -> impl IntoResponse {
    let num = rand::thread_rng().gen_range(0..state.questions.len());
    state.questions[num].to_string()
}
