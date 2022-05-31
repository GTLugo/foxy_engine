pub use tracing::{
  error, warn, debug, info, trace,
  Level,
  subscriber::SetGlobalDefaultError
};
pub use tracing_subscriber::{
  fmt::format,
  FmtSubscriber,
  field::MakeExt,
};
pub use tracing_unwrap::*;

pub mod logger {
  pub use tracing::{
    Level,
    subscriber::SetGlobalDefaultError
  };
  pub use tracing_subscriber::{
    fmt::format,
    FmtSubscriber,
    field::MakeExt,
  };
  pub use tracing_unwrap::*;

  pub fn init() -> Result<(), SetGlobalDefaultError> {
    let fmt = format::debug_fn(|writer, _field, value| {
      write!(writer, "[{:?}]", value)
    }).delimited(" ");
    let fmt_event = format()
      .compact()
      .with_target(false);
    let fmt_subscriber = FmtSubscriber::builder()
      .with_max_level(Level::TRACE)
      .fmt_fields(fmt)
      .event_format(fmt_event)
      .finish();
    tracing::subscriber::set_global_default(fmt_subscriber)
  }
}