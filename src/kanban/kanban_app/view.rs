use makepad_widgets::*;

use crate::kanban::ui::components::board_header::BoardHeader;
use crate::kanban::ui::components::board_toolbar::BoardToolbar;
use crate::kanban::ui::components::boards_sidebar::BoardsSidebar;
use crate::shared::styles::*;

use super::KanbanApp;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;
    use crate::kanban::ui::components::boards_sidebar::BoardsSidebar;
    use crate::kanban::ui::components::board_header::BoardHeader;
    use crate::kanban::ui::components::board_toolbar::BoardToolbar;

    pub KanbanApp = {{KanbanApp}} {
        width: Fill, height: Fill
        flow: Right
        
        sidebar = <BoardsSidebar> {}
        
        main_content = <View> {
            width: Fill, height: Fill
            flow: Down
            
            board_header = <BoardHeader> {}
            
            board_toolbar = <BoardToolbar> {}
            
            board_canvas = <View> {
                width: Fill, height: Fill
                flow: Right
                scroll: vec2(1.0, 0.0)

                padding: 12
                spacing: 12
                
                lists_container = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 8
                    align: {x: 0.0, y: 0.0}
                }
            }
        }
    }
}
