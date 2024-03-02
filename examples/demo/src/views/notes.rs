use loco_rs::{
    controller::views::pagination::{Pager, PagerMeta},
    prelude::model::query::PaginatedResponse,
};
use serde::{Deserialize, Serialize};

use crate::models::_entities::notes;

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteResponse {
    title: Option<String>,
    content: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PaginationResponse {}

impl From<notes::Model> for NoteResponse {
    fn from(note: notes::Model) -> Self {
        Self {
            title: note.title.clone(),
            content: note.content,
        }
    }
}

impl PaginationResponse {
    #[must_use]
    pub fn response(data: PaginatedResponse<notes::Model>) -> Pager<Vec<NoteResponse>> {
        Pager {
            results: data
                .rows
                .into_iter()
                .map(NoteResponse::from)
                .collect::<Vec<NoteResponse>>(),
            info: PagerMeta {
                page: data.info.page,
                page_size: data.info.page_size,
                total_pages: data.info.total_pages,
            },
        }
    }
}
