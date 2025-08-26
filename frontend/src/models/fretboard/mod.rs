pub mod model;
pub mod model_builder;
pub mod model_ext;

pub use model::FretboardModel;
pub use model_builder::{default_tuning, FretboardModelBuilder};
pub use model_ext::FretboardModelExt;
