pub mod logging {
  #[allow(unused_imports)]
  pub use tracing::{error, debug, info, trace, Level};
  pub use tracing::subscriber::SetGlobalDefaultError;
  pub use tracing_subscriber::{
      fmt::format,
      FmtSubscriber,
      field::MakeExt,
  };
  pub use tracing_unwrap::*;

  #[macro_export]
  macro_rules! fox_trace {
    ( $src:literal, $( $msg:literal ),* ) => {
      {
        let mut message = String::new();
        $(
          message += $msg;
        )*
        trace!("{}: {}", $src, message)
      }
    }
  }
  
  #[macro_export]
  macro_rules! fox_info {
    ( $src:literal, $( $msg:literal ),* ) => {
      {
        let mut message = String::new();
        $(
          message += $msg;
        )*
        info!("{}: {}", $src, message)
      }
    }
  }

  #[macro_export]
  macro_rules! fox_debug {
    ( $src:literal, $( $msg:literal ),* ) => {
      {
        let mut message = String::new();
        $(
          message += $msg;
        )*
        debug!("{}: {}", $src, message)
      }
    }
  }

  #[macro_export]
  macro_rules! fox_error {
    ( $src:literal, $( $msg:literal ),* ) => {
      {
        let mut message = String::new();
        $(
          message += $msg;
        )*
        error!("{}: {}", $src, message)
      }
    }
  }
  
  pub fn setup_logging() -> Result<(), SetGlobalDefaultError> {
    let fmt = format::debug_fn(|writer, _field, value| {
        //write!(writer, "[{}: {:?}]", field, value)
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