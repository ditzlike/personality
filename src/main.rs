use axum::{
    extract::{Json, Query, State},
    http::{Response, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use rand::Rng;
use std::net::SocketAddr;

#[derive(Clone)]
struct AppState {
    questions: Vec<String>,
    players: Vec<Player>,
}

#[derive(Clone)]
struct Player {
    name: String,
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

    let mut state = AppState {
        questions: questions,
        players: Vec::new(),
    };

    let mut players: Vec<Player> = Vec::new();

    let app = Router::new()
        .route("/", get(get_question))
        .route("/", post(post_question_with_query))
        .route("/newplayer", post(post_player))
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

#[derive(Debug, serde::Deserialize)]
struct QuestionParams {
    question_number: usize,
}

async fn post_question_with_query(
    State(state): State<AppState>,
    Query(params): Query<QuestionParams>,
) -> impl IntoResponse {
    if !(0..=10).contains(&params.question_number) {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Question number must be between 0 and 10".into())
            .unwrap();
    }

    let num = params.question_number;
    Response::new(state.questions[num].to_string())
}

async fn post_player(
    State(mut state): State<AppState>,
    Query(params): Query<PlayerName>,
) -> impl IntoResponse {
    let new_player = Player {
        name: params.player_name,
    };
    state.players.push(new_player);
    Response::new(format!(
        "Created new player {}",
        state.players.last().unwrap().name
    ))
}

#[derive(Debug, serde::Deserialize)]
struct PlayerName {
    player_name: String,
}
