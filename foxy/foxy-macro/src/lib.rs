pub trait Component: Send + Sync {
  fn register();
}
