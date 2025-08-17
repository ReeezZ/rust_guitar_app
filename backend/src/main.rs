use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use shared::models::exercise::{Exercise, ExerciseType};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

type ExerciseStore = Arc<Mutex<HashMap<String, Exercise>>>;

#[tokio::main]
async fn main() {
    // Initialize in-memory store
    let store = Arc::new(Mutex::new(HashMap::new()));
    
    // Add some sample data for development
    {
        let mut exercises = store.lock().unwrap();
        let sample_exercise = Exercise::new(
            "Sample Scale Exercise".to_string(),
            ExerciseType::Scale {
                root_note: shared::music::notes::Note::C,
                scale_type: shared::music::scales::ScaleType::Hepatonic(
                    shared::music::heptatonic_scales::HeptaScaleType::Major
                ),
                fret_range: (3, 7),
            }
        );
        exercises.insert(sample_exercise.id.clone(), sample_exercise);
    }

    // Create router
    let app = Router::new()
        .route("/api/exercises", get(get_exercises))
        .route("/api/exercises", post(create_exercise))
        .route("/api/exercises/:id", get(get_exercise))
        .route("/api/exercises/:id", put(update_exercise))
        .route("/api/exercises/:id", delete(delete_exercise))
        .layer(CorsLayer::permissive())
        .with_state(store);

    println!("🎸 Guitar Practice Backend starting on http://127.0.0.1:8080");
    println!("📝 API Documentation available at /api/exercises");
    
    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// API Handlers
async fn get_exercises(
    axum::extract::State(store): axum::extract::State<ExerciseStore>,
) -> Result<Json<Vec<Exercise>>, StatusCode> {
    let exercises = store.lock().unwrap();
    Ok(Json(exercises.values().cloned().collect()))
}

async fn create_exercise(
    axum::extract::State(store): axum::extract::State<ExerciseStore>,
    Json(exercise): Json<Exercise>,
) -> Result<Json<Exercise>, StatusCode> {
    let mut exercises = store.lock().unwrap();
    exercises.insert(exercise.id.clone(), exercise.clone());
    Ok(Json(exercise))
}

async fn get_exercise(
    axum::extract::State(store): axum::extract::State<ExerciseStore>,
    Path(id): Path<String>,
) -> Result<Json<Exercise>, StatusCode> {
    let exercises = store.lock().unwrap();
    exercises
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_exercise(
    axum::extract::State(store): axum::extract::State<ExerciseStore>,
    Path(id): Path<String>,
    Json(exercise): Json<Exercise>,
) -> Result<Json<Exercise>, StatusCode> {
    let mut exercises = store.lock().unwrap();
    if exercises.contains_key(&id) {
        exercises.insert(id, exercise.clone());
        Ok(Json(exercise))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn delete_exercise(
    axum::extract::State(store): axum::extract::State<ExerciseStore>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let mut exercises = store.lock().unwrap();
    if exercises.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
