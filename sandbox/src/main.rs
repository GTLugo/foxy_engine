use foxy::{
  util::log::error,
  app::*,
  tokio,
};

struct AppState {
  var0: i32,
  var1: f64,
}

struct TestComponent {
  value: i32,
}

/*
fn test_system(time: Resource<Time>) {

}
*/

#[tokio::main]
async fn main() {
  match App::new("Sandbox", [800, 450]).await {
    Ok(app) => {
      /*
      app.add_global_data::<AppState>();
      app.add_step_before(FoxyStep::Update, CustomStep::Update)
      app.add_system_to_step(FoxyStep::Update, test_system)
       */
      app.run().await;
    }
    Err(e) => {
      error!("{:?}", e);
    }
  };
}
