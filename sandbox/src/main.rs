use foxy::{
  util::log::error,
  App,
  tokio,
};

#[tokio::main]
async fn main() {
  match App::new("Sandbox", [800, 450]).await {
    Ok(app) => {
      app.run().await;
    }
    Err(e) => {
      error!("{:?}", e);
    }
  };
}
