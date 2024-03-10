use super::methods;
use crate::app::models::FlashCardsUi;
use slint::Weak;
use sqlx::SqlitePool;

pub fn build(app: &FlashCardsUi, app_handle: &Weak<FlashCardsUi>, pool: &SqlitePool) {
    let _ = app_handle.upgrade_in_event_loop(move |app| {
        app.set_screen(1);
    });

    methods::init_call_backs(&app, &app_handle, &pool);

    tokio::spawn({
        let pool = pool.clone();
        let app_handle = app_handle.clone();
        async move {
            methods::refresh_card_list(pool.clone(), app_handle.clone()).await;
            let _ = app_handle.upgrade_in_event_loop(move |app| {
                app.invoke_show_card_list();
            });
        }
    });
}
