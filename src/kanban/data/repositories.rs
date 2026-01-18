use std::sync::Arc;
use tokio::sync::Mutex;
use crate::kanban::data::models::{KanbanBoard, KanbanList, KanbanCard, CardDueDate};

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
    pub member_ids: Option<Vec<matrix_sdk::ruma::OwnedUserId>>,
    pub due_date: Option<Option<CardDueDate>>,
    pub is_starred: Option<bool>,
    pub archived: Option<bool>,
}

pub struct MatrixBoardRepository {
    local_boards: Arc<Mutex<Vec<KanbanBoard>>>,
}

impl MatrixBoardRepository {
    pub fn new() -> Self {
        Self {
            local_boards: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for MatrixBoardRepository {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MatrixListRepository {
    local_lists: Arc<Mutex<Vec<KanbanList>>>,
}

impl MatrixListRepository {
    pub fn new() -> Self {
        Self {
            local_lists: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for MatrixListRepository {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MatrixCardRepository {
    local_cards: Arc<Mutex<Vec<KanbanCard>>>,
}

impl MatrixCardRepository {
    pub fn new() -> Self {
        Self {
            local_cards: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for MatrixCardRepository {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RepositoryFactory {
    pub board_repository: MatrixBoardRepository,
    pub list_repository: MatrixListRepository,
    pub card_repository: MatrixCardRepository,
}

impl RepositoryFactory {
    pub fn new() -> Self {
        Self {
            board_repository: MatrixBoardRepository::new(),
            list_repository: MatrixListRepository::new(),
            card_repository: MatrixCardRepository::new(),
        }
    }
}

impl Default for RepositoryFactory {
    fn default() -> Self {
        Self::new()
    }
}
