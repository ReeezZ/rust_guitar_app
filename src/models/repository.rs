// TODO ignore warnings in this file until implemented
#![allow(unused)]

use crate::{
  models::exercise::{Exercise, Song},
  music::{heptatonic_scales::HeptaScaleType, Note, Scale, ScaleExt, ScaleType},
};

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

/// Repository trait for song persistence
pub trait SongsRepository {
  fn save(&self, song: &Song) -> Result<(), RepositoryError>;

  fn update(&self, song: &Song) -> Result<(), RepositoryError>;

  fn delete(&self, id: &str) -> Result<(), RepositoryError>;

  fn find_by_id(&self, id: &str) -> Result<Song, RepositoryError>;

  fn find_all(&self) -> Result<Vec<Song>, RepositoryError>;

  /// Check if an song name exists (excluding a specific ID)
  fn name_exists(&self, name: &str, exclude_id: Option<&str>) -> Result<bool, RepositoryError>;
}

/// Local storage implementation of songRepository
pub struct LocalStorageSongRepository;

impl LocalStorageSongRepository {
  pub fn new() -> Self {
    Self
  }
}

impl Default for LocalStorageSongRepository {
  fn default() -> Self {
    Self::new()
  }
}

impl SongsRepository for LocalStorageSongRepository {
  fn save(&self, song: &Song) -> Result<(), RepositoryError> {
    // crate::models::storage::save_song(song).map_err(RepositoryError::ValidationError)
    // TODO: Implement persistence
    Ok(())
  }

  fn update(&self, song: &Song) -> Result<(), RepositoryError> {
    // crate::models::storage::update_song(song).map_err(RepositoryError::ValidationError)
    // TODO: Implement persistence
    Ok(())
  }

  fn delete(&self, id: &str) -> Result<(), RepositoryError> {
    // crate::models::storage::delete_song(id).map_err(RepositoryError::ValidationError)
    // TODO: Implement persistence
    Ok(())
  }

  fn find_by_id(&self, id: &str) -> Result<Song, RepositoryError> {
    // Ok(crate::models::storage::load_song_by_id(id))
    // TODO: Implement persistence
    Ok(Song::new(
      "Sample song".to_string(),
      "sample-id".to_string(),
    ))
  }

  fn find_all(&self) -> Result<Vec<Song>, RepositoryError> {
    // Ok(crate::models::storage::load_songs())
    // TODO: Implement persistence
    Ok(Vec::new())
  }

  fn name_exists(&self, name: &str, exclude_id: Option<&str>) -> Result<bool, RepositoryError> {
    // Ok(crate::models::storage::song_name_exists(
    //   name, exclude_id,
    // ))
    // TODO: Implement persistence
    Ok(false)
  }
}

/// Global repository instance - can be swapped for different implementations
pub fn get_songs_repository() -> impl SongsRepository {
  LocalStorageSongRepository::new()
}

// Future: could return different implementations based on config
// pub fn get_song_repository() -> Box<dyn songRepository> {
//   match std::env::var("STORAGE_TYPE").as_deref() {
//
//     Ok("remote") => Box::new(RemotesongRepository::new()),
//     _ => Box::new(LocalStoragesongRepository::new()),
//   }
// }
