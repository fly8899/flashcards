use crate::app::models::FlashCardsUi;
use slint::ComponentHandle;
use tokio;

mod app;
mod database;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let pool = database::create::create_project_resources().await;
    let app = FlashCardsUi::new().unwrap();
    let app_handle = app.as_weak();
    app::setup::build(&app, &app_handle, &pool);
    app.run()
}
