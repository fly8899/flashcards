use crate::app::models::FlashCardsUi;
use crate::app::models::UiCard;
use database::models::card::Card;
use slint::{ModelRc, VecModel, Weak};
use sqlx::SqlitePool;
use std::rc::Rc;

use crate::database;

pub async fn refresh_card_list(pool: SqlitePool, app_handle: Weak<FlashCardsUi>) {
    let result = database::models::card::fetch_all_cards(&pool).await;

    if result.is_err() {
        return;
    }

    let list = result
        .unwrap()
        .iter()
        .map(|v: &Card| UiCard {
            id: v.id.into(),
            question: v.question.clone().into(),
            answer: v.answer.clone().into(),
        })
        .collect::<Vec<UiCard>>();

    let _ = app_handle.upgrade_in_event_loop(move |app| {
        app.set_card_list(ModelRc::from(Rc::new(VecModel::from(list))));
    });
}

pub fn init_call_backs(app: &FlashCardsUi, app_handle: &Weak<FlashCardsUi>, pool: &SqlitePool) {
    app.on_show_card_add({
        let app_handle = app_handle.clone();
        move || {
            let app_handle = app_handle.clone();
            let _ = app_handle.upgrade_in_event_loop(move |app| {
                app.set_screen(3);
            });
        }
    });

    app.on_show_card_edit({
        let app_handle = app_handle.clone();
        move |id, question, answer| {
            let _ = app_handle.upgrade_in_event_loop(move |app| {
                app.set_card_id(id);
                app.set_card_question(question.into());
                app.set_card_answer(answer.into());
                app.set_screen(2);
            });
        }
    });

    app.on_show_card_list({
        let app_handle = app_handle.clone();
        move || {
            let _ = app_handle.upgrade_in_event_loop(move |app| {
                app.set_screen(4);
            });
        }
    });

    app.on_show_lesson({
        let app_handle = app_handle.clone();
        let pool = pool.clone();
        move || {
            tokio::spawn({
                let pool = pool.clone();
                let app_handle = app_handle.clone();
                async move {
                    let prepare_lesson_result =
                        database::models::card::mark_cards_not_shown(&pool).await;

                    if prepare_lesson_result.is_err() {
                        return;
                    }

                    let _ = app_handle.upgrade_in_event_loop(move |app| {
                        app.set_screen(5);
                        app.invoke_lesson_set_random_card();
                    });
                }
            });
        }
    });

    app.on_remove_card({
        let pool = pool.clone();
        let app_handle = app_handle.clone();
        move |id| {
            tokio::spawn({
                let pool = pool.clone();
                let app_handle = app_handle.clone();
                async move {
                    let remove_result =
                        database::models::card::remove_card(&pool.clone(), &id).await;

                    if remove_result.is_err() {
                        return;
                    }

                    refresh_card_list(pool.clone(), app_handle.clone()).await;
                }
            });
        }
    });

    app.on_add_card({
        let pool = pool.clone();
        let app_handle = app_handle.clone();
        move |question, answer| {
            if question.is_empty() || answer.is_empty() {
                return;
            }

            tokio::spawn({
                let pool = pool.clone();
                let app_handle = app_handle.clone();
                async move {
                    let add_result = database::models::card::add_card(
                        &pool.clone(),
                        &question.to_string(),
                        &answer.to_string(),
                    )
                    .await;

                    if add_result.is_err() {
                        return;
                    }

                    refresh_card_list(pool.clone(), app_handle.clone()).await;
                }
            });
        }
    });

    app.on_save_card_edit({
        let pool = pool.clone();
        let app_handle = app_handle.clone();
        move |id, question, answer| {
            if question.is_empty() || answer.is_empty() {
                return;
            }

            tokio::spawn({
                let pool = pool.clone();
                let app_handle = app_handle.clone();
                let id = id.clone();
                let answer = answer.clone();
                let question = question.clone();
                async move {
                    let edit_result = database::models::card::update_card(
                        &pool.clone(),
                        &id,
                        &question.to_string(),
                        &answer.to_string(),
                    )
                    .await;

                    if edit_result.is_err() {
                        return;
                    }

                    refresh_card_list(pool.clone(), app_handle.clone()).await;
                }
            });
        }
    });

    app.on_lesson_set_random_card({
        let app_handle = app_handle.clone();
        let pool = pool.clone();
        move || {
            tokio::spawn({
                let app_handle = app_handle.clone();
                let pool = pool.clone();

                async move {
                    let card_result =
                        database::models::card::fetch_random_card_marked_not_shown(&pool).await;

                    if card_result.is_err() {
                        return;
                    }

                    let card_option = card_result.unwrap();

                    if card_option.is_none() {
                        let _ = app_handle.upgrade_in_event_loop(move |app| {
                            app.invoke_show_card_list();
                        });
                    }

                    let card = card_option.unwrap();

                    let mark_shown_result =
                        database::models::card::mark_card_shown(&pool, &card.id).await;

                    if mark_shown_result.is_err() {
                        return;
                    }

                    let _ = app_handle.upgrade_in_event_loop(move |app| {
                        app.set_card_question(card.question.into());
                        app.set_card_answer(card.answer.into());
                    });
                }
            });
        }
    });
}
