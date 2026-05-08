use super::response_helpers::{response_for, upsert_response};
use super::state::SubmissionDraft;
use crate::forms::model::{QuestionOption, Response};
use crate::forms::render::markdown::MarkdownDescription;
use dioxus::prelude::*;

#[component]
pub(super) fn RankedListQuestionInput(
    question_id: String,
    description_markdown: Option<String>,
    options: Vec<QuestionOption>,
    randomize_initial_order: bool,
    draft: Signal<SubmissionDraft>,
) -> Element {
    let initial_ranked_ids = use_signal(|| ranked_option_ids(&options, randomize_initial_order));
    let mut dragging_option_id = use_signal(|| None::<String>);
    let mut drag_over_option_id = use_signal(|| None::<String>);
    let ranked_ids = match response_for(&draft.read().responses, &question_id).map(|r| &r.response)
    {
        Some(Response::RankedList { ranked_option_ids }) => ranked_option_ids.clone(),
        _ => initial_ranked_ids.read().clone(),
    };
    let dragging_id = dragging_option_id.read().clone();
    let drag_over_id = drag_over_option_id.read().clone();

    use_effect({
        let question_id = question_id.clone();
        move || {
            let has_ranked_response = matches!(
                response_for(&draft.read().responses, &question_id)
                    .map(|response| &response.response),
                Some(Response::RankedList { .. })
            );

            if !has_ranked_response {
                upsert_response(
                    draft,
                    question_id.clone(),
                    Response::RankedList {
                        ranked_option_ids: initial_ranked_ids.read().clone(),
                    },
                );
            }
        }
    });

    rsx! {
        if let Some(md) = description_markdown {
            MarkdownDescription { markdown: md }
        }
        p { class: "form-question__prompt", "Drag to reorder, or use the arrows to rank items." }
        div { class: "ranked-list",
            for (rank, option_id) in ranked_ids.iter().enumerate() {
                {
                    let label = options
                        .iter()
                        .find(|o| &o.question_option_id == option_id)
                        .map(|o| o.label.clone())
                        .unwrap_or_else(|| option_id.clone());
                    let option_id = option_id.clone();
                    let ranked_ids_up = ranked_ids.clone();
                    let ranked_ids_down = ranked_ids.clone();
                    let ranked_ids_drag = ranked_ids.clone();
                    let question_id_up = question_id.clone();
                    let question_id_down = question_id.clone();
                    let question_id_drag = question_id.clone();
                    let option_id_drag_start = option_id.clone();
                    let option_id_drag_enter = option_id.clone();
                    let option_id_drag_over = option_id.clone();
                    let option_id_drag_leave = option_id.clone();
                    let is_dragging = dragging_id.as_deref() == Some(option_id.as_str());
                    let is_drop_target = !is_dragging
                        && drag_over_id.as_deref() == Some(option_id.as_str());
                    let item_class = if is_dragging {
                        "ranked-list__item ranked-list__item--dragging"
                    } else if is_drop_target {
                        "ranked-list__item ranked-list__item--drop-target"
                    } else {
                        "ranked-list__item"
                    };
                    rsx! {
                        div {
                            key: "{option_id}",
                            class: "{item_class}",
                            draggable: "true",
                            aria_grabbed: if is_dragging { "true" } else { "false" },
                            ondragstart: move |event| {
                                let transfer = event.data().data_transfer();
                                let _ = transfer.set_data("text/plain", &option_id_drag_start);
                                transfer.set_effect_allowed("move");
                                dragging_option_id.set(Some(option_id_drag_start.clone()));
                            },
                            ondragenter: move |event| {
                                event.prevent_default();
                                drag_over_option_id.set(Some(option_id_drag_enter.clone()));

                                let Some(dragged_option_id) = dragging_option_id.read().clone() else {
                                    return;
                                };

                                let reordered_ids = reorder_ranked_option_ids(
                                    ranked_ids_drag.clone(),
                                    &dragged_option_id,
                                    &option_id_drag_enter,
                                );

                                if reordered_ids != ranked_ids_drag {
                                    update_ranked_list_response(
                                        draft,
                                        question_id_drag.clone(),
                                        reordered_ids,
                                    );
                                }
                            },
                            ondragover: move |event| {
                                event.prevent_default();
                                event.data().data_transfer().set_drop_effect("move");
                                drag_over_option_id.set(Some(option_id_drag_over.clone()));
                            },
                            ondragleave: move |_| {
                                if drag_over_option_id.read().as_deref()
                                    == Some(option_id_drag_leave.as_str())
                                {
                                    drag_over_option_id.set(None);
                                }
                            },
                            ondrop: move |event| {
                                event.prevent_default();
                                dragging_option_id.set(None);
                                drag_over_option_id.set(None);
                            },
                            ondragend: move |_| {
                                dragging_option_id.set(None);
                                drag_over_option_id.set(None);
                            },
                            span { class: "ranked-list__rank", "{rank + 1}" }
                            span { class: "ranked-list__label", "{label}" }
                            div { class: "ranked-list__controls",
                                button {
                                    class: "ranked-list__btn",
                                    r#type: "button",
                                    disabled: rank == 0,
                                    onclick: move |_| {
                                        let mut ids = ranked_ids_up.clone();
                                        ids.swap(rank, rank - 1);
                                        update_ranked_list_response(
                                            draft,
                                            question_id_up.clone(),
                                            ids,
                                        );
                                    },
                                    "↑"
                                }
                                button {
                                    class: "ranked-list__btn",
                                    r#type: "button",
                                    disabled: rank == ranked_ids.len() - 1,
                                    onclick: move |_| {
                                        let mut ids = ranked_ids_down.clone();
                                        ids.swap(rank, rank + 1);
                                        update_ranked_list_response(
                                            draft,
                                            question_id_down.clone(),
                                            ids,
                                        );
                                    },
                                    "↓"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn update_ranked_list_response(
    draft: Signal<SubmissionDraft>,
    question_id: String,
    ranked_option_ids: Vec<String>,
) {
    upsert_response(
        draft,
        question_id,
        Response::RankedList { ranked_option_ids },
    );
}

fn reorder_ranked_option_ids(
    mut ranked_ids: Vec<String>,
    dragged_option_id: &str,
    target_option_id: &str,
) -> Vec<String> {
    if dragged_option_id == target_option_id {
        return ranked_ids;
    }

    let Some(from_index) = ranked_ids
        .iter()
        .position(|option_id| option_id == dragged_option_id)
    else {
        return ranked_ids;
    };
    let Some(to_index) = ranked_ids
        .iter()
        .position(|option_id| option_id == target_option_id)
    else {
        return ranked_ids;
    };

    let dragged_id = ranked_ids.remove(from_index);
    ranked_ids.insert(to_index.min(ranked_ids.len()), dragged_id);
    ranked_ids
}

fn ranked_option_ids(options: &[QuestionOption], randomize_initial_order: bool) -> Vec<String> {
    let mut ids: Vec<_> = options
        .iter()
        .map(|option| option.question_option_id.clone())
        .collect();

    if randomize_initial_order {
        shuffle_ranked_option_ids(&mut ids);
    }

    ids
}

fn shuffle_ranked_option_ids(ids: &mut [String]) {
    for index in (1..ids.len()).rev() {
        let swap_index = (js_sys::Math::random() * (index + 1) as f64).floor() as usize;
        ids.swap(index, swap_index);
    }
}

#[cfg(test)]
mod tests {
    use super::reorder_ranked_option_ids;

    #[test]
    fn reorder_ranked_option_ids_moves_items_to_target_rank() {
        let ids = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];

        assert_eq!(
            reorder_ranked_option_ids(ids.clone(), "a", "c"),
            vec!["b", "c", "a", "d"]
        );
        assert_eq!(
            reorder_ranked_option_ids(ids.clone(), "d", "b"),
            vec!["a", "d", "b", "c"]
        );
    }
}
