use foxy::app::*;

fn main() {
  let app_data = AppInfo {
    title: "Sandbox",
    width: 800,
    height: 450
  };
  let app = App::new(app_data);
  app.run();
}
