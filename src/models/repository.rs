use crate::models::exercise::Exercise;

/// Error types for repository operations
#[derive(Debug, Clone)]
pub enum RepositoryError {
  NotFound(String),
  StorageUnavailable,
  SerializationError(String),
  ValidationError(String),
}

impl std::fmt::Display for RepositoryError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      RepositoryError::NotFound(msg) => write!(f, "Not found: {msg}"),
      RepositoryError::StorageUnavailable => write!(f, "Storage is not available"),
      RepositoryError::SerializationError(msg) => write!(f, "Serialization error: {msg}"),
      RepositoryError::ValidationError(msg) => write!(f, "Validation error: {msg}"),
    }
  }
}

/// Repository trait for exercise persistence
pub trait ExerciseRepository {
  /// Save a new exercise
  fn save(&self, exercise: &Exercise) -> Result<(), RepositoryError>;
  
  /// Update an existing exercise
  fn update(&self, exercise: &Exercise) -> Result<(), RepositoryError>;
  
  /// Delete an exercise by ID
  fn delete(&self, id: &str) -> Result<(), RepositoryError>;
  
  /// Find an exercise by ID
  fn find_by_id(&self, id: &str) -> Result<Option<Exercise>, RepositoryError>;
  
  /// Find all exercises
  fn find_all(&self) -> Result<Vec<Exercise>, RepositoryError>;
  
  /// Check if an exercise name exists (excluding a specific ID)
  fn name_exists(&self, name: &str, exclude_id: Option<&str>) -> Result<bool, RepositoryError>;
}

/// Local storage implementation of ExerciseRepository
pub struct LocalStorageExerciseRepository;

impl LocalStorageExerciseRepository {
  pub fn new() -> Self {
    Self
  }
}

impl Default for LocalStorageExerciseRepository {
  fn default() -> Self {
    Self::new()
  }
}

impl ExerciseRepository for LocalStorageExerciseRepository {
  fn save(&self, exercise: &Exercise) -> Result<(), RepositoryError> {
    crate::models::storage::save_exercise(exercise);
    Ok(())
  }
  
  fn update(&self, exercise: &Exercise) -> Result<(), RepositoryError> {
    crate::models::storage::update_exercise(exercise)
      .map_err(RepositoryError::ValidationError)
  }
  
  fn delete(&self, id: &str) -> Result<(), RepositoryError> {
    crate::models::storage::delete_exercise(id);
    Ok(())
  }
  
  fn find_by_id(&self, id: &str) -> Result<Option<Exercise>, RepositoryError> {
    Ok(crate::models::storage::load_exercise_by_id(id))
  }
  
  fn find_all(&self) -> Result<Vec<Exercise>, RepositoryError> {
    Ok(crate::models::storage::load_exercises())
  }
  
  fn name_exists(&self, name: &str, exclude_id: Option<&str>) -> Result<bool, RepositoryError> {
    Ok(crate::models::storage::exercise_name_exists(name, exclude_id))
  }
}

/// Global repository instance - can be swapped for different implementations
pub fn get_exercise_repository() -> impl ExerciseRepository {
  LocalStorageExerciseRepository::new()
}

// Future: could return different implementations based on config
// pub fn get_exercise_repository() -> Box<dyn ExerciseRepository> {
//   match std::env::var("STORAGE_TYPE").as_deref() {
//     Ok("remote") => Box::new(RemoteExerciseRepository::new()),
//     _ => Box::new(LocalStorageExerciseRepository::new()),
//   }
// }
