use std::collections::HashMap;
use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId};
use crate::kanban::data::models::*;

#[derive(Debug, Clone, Default)]
pub struct KanbanAppState {
    pub current_user_id: Option<OwnedUserId>,

    pub current_board_id: Option<OwnedRoomId>,

    pub boards: HashMap<OwnedRoomId, KanbanBoard>,

    pub lists: HashMap<String, KanbanList>,

    pub cards: HashMap<String, KanbanCard>,

    pub loading: bool,

    pub error: Option<String>,

    pub filter_state: Option<KanbanFilterState>,

    pub sort_state: Option<KanbanSortState>,
}

impl KanbanAppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current_board(&self) -> Option<&KanbanBoard> {
        self.current_board_id
            .as_ref()
            .and_then(|id| self.boards.get(id))
    }

    pub fn current_board_lists(&self) -> Vec<&KanbanList> {
        if let Some(board) = self.current_board() {
            board
                .list_ids
                .iter()
                .filter_map(|list_id| self.lists.get(list_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn list_cards(&self, list_id: &str) -> Vec<&KanbanCard> {
        if let Some(list) = self.lists.get(list_id) {
            list.card_ids
                .iter()
                .filter_map(|card_id| self.cards.get(card_id))
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    pub fn upsert_board(&mut self, board: KanbanBoard) {
        self.boards.insert(board.id.clone(), board);
    }

    pub fn upsert_list(&mut self, list: KanbanList) {
        self.lists.insert(list.id.clone(), list);
    }

    pub fn upsert_card(&mut self, card: KanbanCard) {
        self.cards.insert(card.id.clone(), card);
    }

    pub fn remove_board(&mut self, board_id: &OwnedRoomId) {
        self.boards.remove(board_id);
        if self.current_board_id.as_ref() == Some(board_id) {
            self.current_board_id = None;
        }
    }

    pub fn remove_list(&mut self, list_id: &str) {
        self.lists.remove(list_id);
    }

    pub fn remove_card(&mut self, card_id: &str) {
        self.cards.remove(card_id);
    }
}
