use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId};
use crate::kanban::data::models::*;

#[derive(Debug, Clone)]
pub enum KanbanActions {
    LoadBoards,

    SelectBoard(OwnedRoomId),

    CreateBoard {
        name: String,
        description: Option<String>,
    },

    UpdateBoard {
        board_id: OwnedRoomId,
        updates: BoardUpdateRequest,
    },

    DeleteBoard {
        board_id: OwnedRoomId,
    },

    LoadLists {
        board_id: OwnedRoomId,
    },

    CreateList {
        board_id: OwnedRoomId,
        name: String,
    },

    UpdateList {
        board_id: OwnedRoomId,
        list_id: String,
        updates: ListUpdateRequest,
    },

    DeleteList {
        board_id: OwnedRoomId,
        list_id: String,
    },

    MoveList {
        board_id: OwnedRoomId,
        list_id: String,
        new_position: f64,
    },

    LoadCards {
        board_id: OwnedRoomId,
        list_id: String,
    },

    CreateCard {
        board_id: OwnedRoomId,
        list_id: String,
        name: String,
    },

    UpdateCard {
        board_id: OwnedRoomId,
        card_id: String,
        updates: CardUpdateRequest,
    },

    DeleteCard {
        board_id: OwnedRoomId,
        card_id: String,
    },

    MoveCard {
        board_id: OwnedRoomId,
        card_id: String,
        from_list: String,
        to_list: String,
        new_position: f64,
    },

    ArchiveCard {
        board_id: OwnedRoomId,
        card_id: String,
        archived: bool,
    },

    SetFilter(KanbanFilterState),

    SetSort(KanbanSortState),

    Search {
        board_id: OwnedRoomId,
        query: String,
    },

    Error(String),

    Loading(bool),
}

#[derive(Debug, Clone, Default)]
pub struct BoardUpdateRequest {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub background_color: Option<String>,
    pub background_image: Option<Option<String>>,
}

#[derive(Debug, Clone, Default)]
pub struct ListUpdateRequest {
    pub name: Option<String>,
    pub archived: Option<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct CardUpdateRequest {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub label_ids: Option<Vec<String>>,
    pub member_ids: Option<Vec<OwnedUserId>>,
    pub due_date: Option<Option<CardDueDate>>,
    pub is_starred: Option<bool>,
    pub archived: Option<bool>,
}
