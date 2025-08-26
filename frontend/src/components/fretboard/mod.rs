pub mod base;
pub(crate) mod visual_config;

mod model_adapter;

pub use base::{FretClickEvent, FretCoord, FretState, FretStateColor, FretStateSignals, Fretboard};

pub use base::definitions;
pub use model_adapter::FretboardModelAdapter;
pub use visual_config::{FretboardVisualConfig, FretboardVisualConfigBuilder};
