use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId};
use crate::kanban::data::models::{KanbanBoard, KanbanList, KanbanCard};

#[derive(Debug, Clone)]
pub enum KanbanRequest {
    CreateBoard {
        name: String,
        description: Option<String>,
        background_color: Option<String>,
        invite: Vec<String>,
    },

    GetBoards {
        include_archived: bool,
    },

    GetBoard {
        board_id: OwnedRoomId,
    },

    UpdateBoard {
        board_id: OwnedRoomId,
        updates: crate::kanban::state::kanban_actions::BoardUpdateRequest,
    },

    DeleteBoard {
        board_id: OwnedRoomId,
        permanent: bool,
    },

    ArchiveBoard {
        board_id: OwnedRoomId,
        archived: bool,
    },

    AddMember {
        board_id: OwnedRoomId,
        user_id: OwnedUserId,
    },

    RemoveMember {
        board_id: OwnedRoomId,
        user_id: OwnedUserId,
    },
}

#[derive(Debug, Clone)]
pub enum KanbanResponse {
    Board(KanbanBoard),
    Boards(Vec<KanbanBoard>),
    List(KanbanList),
    Lists(Vec<KanbanList>),
    Card(KanbanCard),
    Cards(Vec<KanbanCard>),
    Success,
    Error(String),
}
