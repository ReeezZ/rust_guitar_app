use leptos::prelude::*;
use std::cell::RefCell;
use web_sys::{AudioContext, AudioContextState};

thread_local! {
  /// Global audio manager instance using thread-local storage
  /// This is appropriate for WASM's single-threaded environment
  static GLOBAL_AUDIO_MANAGER: RefCell<Option<AudioContext>> = const { RefCell::new(None) };
}

/// Shared audio manager for the application
/// Handles AudioContext lifecycle and provides audio utilities
pub struct AudioManager;

impl AudioManager {
  /// Get or create the shared AudioContext
  /// Returns None if AudioContext creation fails or context is in unusable state
  pub fn get_context() -> Option<AudioContext> {
    GLOBAL_AUDIO_MANAGER.with(|manager| {
      let mut ctx_ref = manager.borrow_mut();

      // Check if we have an existing context
      if let Some(existing_ctx) = ctx_ref.as_ref() {
        // Validate the existing context state
        match existing_ctx.state() {
          AudioContextState::Running => {
            return Some(existing_ctx.clone());
          }
          AudioContextState::Suspended => {
            leptos::logging::log!("AudioContext is suspended, attempting to resume");
            // Try to resume the context
            if let Ok(_promise) = existing_ctx.resume() {
              // Note: We can't await here since we're not async, but the context
              // should resume automatically when user interacts with the page
              leptos::logging::log!("AudioContext resume initiated");
            }
            return Some(existing_ctx.clone());
          }
          AudioContextState::Closed => {
            leptos::logging::warn!("AudioContext is closed, creating new one");
            // Context is closed, need to create a new one
            *ctx_ref = None;
          }
          _ => {
            leptos::logging::warn!("AudioContext in unknown state: {:?}", existing_ctx.state());
          }
        }
      }

      // Create new AudioContext
      match AudioContext::new() {
        Ok(new_ctx) => {
          leptos::logging::log!(
            "AudioContext created successfully (state: {:?})",
            new_ctx.state()
          );
          *ctx_ref = Some(new_ctx.clone());
          Some(new_ctx)
        }
        Err(err) => {
          leptos::logging::error!("Failed to create AudioContext: {:?}", err);
          None
        }
      }
    })
  }

  /// Check if AudioContext is available and in a usable state
  pub fn is_available() -> bool {
    if let Some(ctx) = Self::get_context() {
      matches!(
        ctx.state(),
        AudioContextState::Running | AudioContextState::Suspended
      )
    } else {
      false
    }
  }

  /// Get the current time from AudioContext (useful for precise timing)
  pub fn current_time() -> Option<f64> {
    Self::get_context().map(|ctx| ctx.current_time())
  }

  /// Resume AudioContext if suspended (typically called on user interaction)
  pub fn resume() -> Result<(), String> {
    if let Some(ctx) = Self::get_context() {
      if ctx.state() == AudioContextState::Suspended {
        let _ = ctx
          .resume()
          .map_err(|err| format!("Failed to resume AudioContext: {err:?}"))?;
        leptos::logging::log!("AudioContext resume initiated");
      }
      Ok(())
    } else {
      Err("No AudioContext available".to_string())
    }
  }

  /// Close the AudioContext (cleanup)
  pub fn close() {
    GLOBAL_AUDIO_MANAGER.with(|manager| {
      let mut ctx_ref = manager.borrow_mut();
      if let Some(ctx) = ctx_ref.take() {
        if let Err(err) = ctx.close() {
          leptos::logging::warn!("Failed to close AudioContext: {:?}", err);
        } else {
          leptos::logging::log!("AudioContext closed successfully");
        }
      }
    });
  }
}

/// Convenience function to get AudioContext directly
/// This is a shorthand for AudioManager::get_context()
pub fn use_audio_context() -> Option<AudioContext> {
  AudioManager::get_context()
}

#[cfg(test)]
mod tests {
  #[cfg(target_arch = "wasm32")]
  use super::*;

  #[test]
  #[cfg(target_arch = "wasm32")]
  fn test_audio_manager_methods() {
    // Note: AudioContext creation will fail in test environment without DOM
    // This test just ensures the methods can be called
    assert!(!AudioManager::is_available());
  }
}
