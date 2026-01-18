use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;

    pub BoardHeader = {{BoardHeader}} {
        flow: Right,
        width: Fill,
        height: 48,
        align: {x: 0.0, y: 0.5},
        spacing: 8,
        padding: 12,

        title_area = <View> {

            flow: Right,
            width: Fit, height: Fit,
            spacing: 8,

            board_icon = <View> {
                width: 24, height: 24,
                show_bg: true,
                draw_bg: {
                    color: #0079BF
                    border_radius: 4
                },
                align: {x: 0.5, y: 0.5},
                icon = <Label> {
                    text: "📋"
                    draw_text: { font_size: 14 }
                }
            }

            board_title = <Label> {
                width: Fit, height: Fit,
                text: "my kanban",
                draw_text: {
                    text_style: <THEME_FONT_BOLD>{font_size: 16}
                    color: #172B4D
                }
            }
        }

        action_buttons = <View> {

            flow: Right,
            width: Fit, height: Fit,
            margin: {left: 16},
            spacing: 4,

            filter_button = <Button> {
                width: Fit, height: 32,
                padding: 8,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                    color: #5E6C84
                },
                text: "filter"
            }

            sort_button = <Button> {
                width: Fit, height: 32,
                padding: 8,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 13}
                    color: #5E6C84
                },
                text: "order"
            }

            menu_button = <Button> {
                width: 32, height: 32,
                draw_text: {
                    text_style: <THEME_FONT_REGULAR>{font_size: 12}
                    color: #5E6C84
                },
                text: "..."
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BoardHeader {
    #[deref]
    view: View,

    #[rust]
    board_name: String,

    #[rust]
    on_filter_click: Option<Box<dyn FnMut()>>,
    #[rust]
    on_sort_click: Option<Box<dyn FnMut()>>,
    #[rust]
    on_menu_click: Option<Box<dyn FnMut()>>,
}

impl BoardHeader {
    pub fn new(cx: &mut Cx) -> Self {
        Self {
            view: View::new(cx),
            board_name: String::from("我的看板"),
            on_filter_click: None,
            on_sort_click: None,
            on_menu_click: None,
        }
    }

    pub fn set_board_name(&mut self, name: &str) {
        self.board_name = name.to_string();
    }

    pub fn set_on_filter<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_filter_click = Some(Box::new(callback));
    }

    pub fn set_on_sort<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_sort_click = Some(Box::new(callback));
    }

    pub fn set_on_menu<F>(&mut self, callback: F)
    where
        F: FnMut() + 'static,
    {
        self.on_menu_click = Some(Box::new(callback));
    }
}

impl Widget for BoardHeader {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.view.handle_event(cx, event, _scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl BoardHeaderRef {}
impl BoardHeaderSet {}
