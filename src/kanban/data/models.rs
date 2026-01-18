use serde::{Deserialize, Serialize};
use matrix_sdk::ruma::{OwnedRoomId, OwnedUserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanBoard {
    pub id: OwnedRoomId,

    pub name: String,

    pub description: Option<String>,

    pub background_color: String,

    pub background_image: Option<String>,

    pub labels: Vec<KanbanLabel>,

    pub member_ids: Vec<OwnedUserId>,

    pub list_ids: Vec<String>,

    pub is_archived: bool,

    pub created_at: String,

    pub updated_at: String,

    pub extensions: BoardExtensions,
}

impl Default for KanbanBoard {
    fn default() -> Self {
        Self {
            id: matrix_sdk::ruma::OwnedRoomId::try_from("!dummy:matrix.local").unwrap(),
            name: String::new(),
            description: None,
            background_color: "#0079BF".to_string(),
            background_image: None,
            labels: Vec::new(),
            member_ids: Vec::new(),
            list_ids: Vec::new(),
            is_archived: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            extensions: BoardExtensions::default(),
        }
    }
}

impl KanbanBoard {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            ..Default::default()
        }
    }

    pub fn member_count(&self) -> usize {
        self.member_ids.len()
    }

    pub fn list_count(&self) -> usize {
        self.list_ids.len()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanLabel {
    pub id: String,

    pub name: String,

    pub color: LabelColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LabelColor {
    Green,
    Yellow,
    Orange,
    Red,
    Purple,
    Blue,
    Sky,
    Lime,
    Pink,
    Black,
}

impl LabelColor {
    pub fn to_hex(&self) -> &'static str {
        match self {
            LabelColor::Green => "#61BD4F",
            LabelColor::Yellow => "#F2D600",
            LabelColor::Orange => "#FF9F1A",
            LabelColor::Red => "#EB5A46",
            LabelColor::Purple => "#9775FA",
            LabelColor::Blue => "#0079BF",
            LabelColor::Sky => "#00C2E0",
            LabelColor::Lime => "#51E898",
            LabelColor::Pink => "#FF78CB",
            LabelColor::Black => "#343434",
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BoardExtensions {
    pub view_settings: ViewSettings,

    pub filter_state: Option<KanbanFilterState>,

    pub sort_state: Option<KanbanSortState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewSettings {
    pub card_view_mode: CardViewMode,

    pub show_completed: bool,

    pub page_size: u32,
}

impl Default for ViewSettings {
    fn default() -> Self {
        Self {
            card_view_mode: CardViewMode::Detailed,
            show_completed: true,
            page_size: 50,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CardViewMode {
    Compact,
    Detailed,
    Cover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanList {
    pub id: String,

    pub name: String,

    pub board_id: OwnedRoomId,

    pub position: f64,

    pub is_archived: bool,

    pub card_ids: Vec<String>,

    pub created_at: String,

    pub updated_at: String,
}

impl Default for KanbanList {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            board_id: matrix_sdk::ruma::OwnedRoomId::try_from("!dummy:matrix.local").unwrap(),
            position: 1000.0,
            is_archived: false,
            card_ids: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl KanbanList {
    pub fn new(name: &str, board_id: OwnedRoomId) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            board_id,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            ..Default::default()
        }
    }

    pub fn card_count(&self) -> usize {
        self.card_ids.len()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCard {
    pub id: String,

    pub title: String,

    pub description: Option<String>,

    pub list_id: String,

    pub board_id: OwnedRoomId,

    pub position: f64,

    pub label_ids: Vec<String>,

    pub member_ids: Vec<OwnedUserId>,

    pub due_date: Option<CardDueDate>,

    pub cover: Option<CardCover>,

    pub attachment_count: u32,

    pub comment_count: u32,

    pub checklists: Vec<CardChecklist>,

    pub is_starred: bool,

    pub is_archived: bool,

    pub created_at: String,

    pub updated_at: String,
}

impl Default for KanbanCard {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            description: None,
            list_id: String::new(),
            board_id: matrix_sdk::ruma::OwnedRoomId::try_from("!dummy:matrix.local").unwrap(),
            position: 1000.0,
            label_ids: Vec::new(),
            member_ids: Vec::new(),
            due_date: None,
            cover: None,
            attachment_count: 0,
            comment_count: 0,
            checklists: Vec::new(),
            is_starred: false,
            is_archived: false,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl KanbanCard {
    pub fn new(title: &str, list_id: String, board_id: OwnedRoomId) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            list_id,
            board_id,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            ..Default::default()
        }
    }

    pub fn is_completed(&self) -> bool {
        self.checklists.iter().all(|cl| cl.is_completed())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDueDate {
    pub date: String,
    pub is_completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardCover {
    pub url: String,
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardChecklist {
    pub id: String,
    pub name: String,
    pub items: Vec<ChecklistItem>,
}

impl CardChecklist {
    pub fn is_completed(&self) -> bool {
        self.items.iter().all(|item| item.is_checked)
    }

    pub fn completed_count(&self) -> usize {
        self.items.iter().filter(|item| item.is_checked).count()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
    pub id: String,
    pub name: String,
    pub is_checked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanFilterState {
    pub keyword: Option<String>,
    pub label_ids: Vec<String>,
    pub member_ids: Vec<OwnedUserId>,
    pub due_date: Option<DueDateFilter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DueDateFilter {
    Overdue,
    Today,
    Tomorrow,
    ThisWeek,
    NextWeek,
    NoDue,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanSortState {
    pub field: SortField,
    pub direction: SortDirection,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortField {
    Position,
    CreatedAt,
    UpdatedAt,
    Title,
    DueDate,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    Descending,
}
