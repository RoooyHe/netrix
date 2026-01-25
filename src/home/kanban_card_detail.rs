use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::shared::styles::*;

    pub KanbanCardDetail = {{KanbanCardDetail}} {
        width: Fill, height: Fill
        flow: Down
        show_bg: true
        draw_bg: {
            color: #F4F5F7
        }

        content = <View> {
            width: Fill, height: Fill
            flow: Right
            spacing: 16
            padding: 16

            main_column = <View> {
                width: Fill, height: Fit
                flow: Down
                spacing: 16

                title_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    title_icon = <View> {
                        width: 24, height: 24
                        show_bg: true
                        draw_bg: { color: #EBECF0 }
                    }

                    card_title = <Label> {
                        width: Fill, height: Fit
                        text: "Card title"
                        draw_text: {
                            text_style: <THEME_FONT_BOLD>{font_size: 20}
                            color: #172B4D
                        }
                    }

                    in_list_label = <Label> {
                        width: Fill, height: Fit
                        text: "In List"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #5E6C84
                        }
                    }
                }

                description_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    section_header = <View> {
                        width: Fill, height: 32
                        flow: Right
                        spacing: 8
                        align: {y: 0.5}

                        desc_icon = <View> {
                            width: 24, height: 24
                            show_bg: true
                            draw_bg: { color: #EBECF0 }
                        }

                        desc_label = <Label> {
                            width: Fit, height: Fit
                            text: "Des"
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{font_size: 16}
                                color: #172B4D
                            }
                        }
                    }

                    desc_content = <View> {
                        width: Fill, height: Fit
                        padding: 8
                        show_bg: true
                        draw_bg: {
                            color: #FFFFFF
                        }

                        desc_text = <Label> {
                            width: Fill, height: Fit
                            text: "This is a card detail."
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{font_size: 14}
                                color: #172B4D
                            }
                        }
                    }
                }

                activity_section = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 8

                    activity_header = <View> {
                        width: Fill, height: 32
                        flow: Right
                        spacing: 8
                        align: {y: 0.5}

                        activity_icon = <View> {
                            width: 24, height: 24
                            show_bg: true
                            draw_bg: { color: #EBECF0 }
                        }

                        activity_label = <Label> {
                            width: Fit, height: Fit
                            text: "active"
                            draw_text: {
                                text_style: <THEME_FONT_BOLD>{font_size: 16}
                                color: #172B4D
                            }
                        }
                    }

                    activity_item_1 = <View> {
                        width: Fill, height: 40
                        flow: Right
                        spacing: 8
                        align: {y: 0.5}

                        avatar_1 = <View> {
                            width: 32, height: 32
                            show_bg: true
                            draw_bg: { color: #0079BF }
                        }

                        activity_text_1 = <Label> {
                            width: Fill, height: Fit
                            text: "Roy add this to will"
                            draw_text: {
                                text_style: <THEME_FONT_REGULAR>{font_size: 13}
                                color: #172B4D
                            }
                        }
                    }
                }
            }

            sidebar = <View> {
                width: 180, height: Fill
                flow: Down
                spacing: 8

                add_to_card_label = <Label> {
                    width: Fill, height: Fit
                    text: "add to card"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                members_label = <Label> {
                    width: Fill, height: Fit
                    text: "user"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                labels_label = <Label> {
                    width: Fill, height: Fit
                    text: "tag"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                labels_row = <View> {
                    width: Fill, height: Fit
                    flow: Right
                    spacing: 4

                    label_blue = <View> {
                        width: 40, height: 8
                        show_bg: true
                        draw_bg: { color: #0079BF }
                    }
                    label_green = <View> {
                        width: 40, height: 8
                        show_bg: true
                        draw_bg: { color: #61BD4F }
                    }
                    label_orange = <View> {
                        width: 40, height: 8
                        show_bg: true
                        draw_bg: { color: #FF9F1A }
                    }
                }

                dates_label = <Label> {
                    width: Fill, height: Fit
                    text: "date"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                date_value = <Label> {
                    width: Fill, height: Fit
                    text: "2026.1.25"
                    draw_text: {
                        text_style: <THEME_FONT_REGULAR>{font_size: 14}
                        color: #172B4D
                    }
                }

                checklist_label = <Label> {
                    width: Fill, height: Fit
                    text: "todo"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                checklist_progress = <View> {
                    width: Fill, height: Fit
                    flow: Down
                    spacing: 4

                    progress_bar = <View> {
                        width: Fill, height: 8
                        show_bg: true
                        draw_bg: { color: #EBECF0 }

                        progress_fill = <View> {
                            width: 50, height: 8
                            show_bg: true
                            draw_bg: { color: #00C2E0 }
                        }
                    }

                    checklist_text = <Label> {
                        width: Fill, height: Fit
                        text: "2/4 success"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 13}
                            color: #5E6C84
                        }
                    }
                }

                actions_label = <Label> {
                    width: Fill, height: Fit
                    text: "operation"
                    draw_text: {
                        text_style: <THEME_FONT_BOLD>{font_size: 12}
                        color: #5E6C84
                    }
                }

                move_card_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8}

                    move_label = <Label> {
                        width: Fill, height: Fit
                        text: "move"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                        }
                    }
                }

                copy_card_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8}

                    copy_label = <Label> {
                        width: Fill, height: Fit
                        text: "copy"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                        }
                    }
                }

                archive_btn = <View> {
                    width: Fill, height: 32
                    show_bg: true
                    draw_bg: { color: #EBECF0 }
                    align: {x: 0.0, y: 0.5}
                    padding: {left: 8}

                    archive_label = <Label> {
                        width: Fill, height: Fit
                        text: "archive"
                        draw_text: {
                            text_style: <THEME_FONT_REGULAR>{font_size: 14}
                            color: #172B4D
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct KanbanCardDetail {
    #[deref]
    view: View,
}

#[derive(Clone, Debug, DefaultNone)]
pub enum KanbanCardDetailAction {
    Close,
    None,
}

impl Widget for KanbanCardDetail {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl KanbanCardDetail {
    pub fn set_detail(
        &mut self,
        cx: &mut Cx,
        title: &str,
        list_name: &str,
        description: &str,
        due_date: &str,
        checklist_text: &str,
    ) {
        self.view
            .label(ids!(main_column.title_section.card_title))
            .set_text(cx, title);
        self.view
            .label(ids!(main_column.title_section.in_list_label))
            .set_text(cx, &format!("inTheList {list_name}"));
        self.view
            .label(ids!(main_column.description_section.desc_content.desc_text))
            .set_text(cx, description);
        self.view
            .label(ids!(sidebar.date_value))
            .set_text(cx, due_date);
        self.view
            .label(ids!(sidebar.checklist_progress.checklist_text))
            .set_text(cx, checklist_text);
    }
}

impl KanbanCardDetailRef {
    pub fn set_detail(
        &self,
        cx: &mut Cx,
        title: &str,
        list_name: &str,
        description: &str,
        due_date: &str,
        checklist_text: &str,
    ) {
        let Some(mut inner) = self.borrow_mut() else {
            return;
        };
        inner.set_detail(cx, title, list_name, description, due_date, checklist_text);
    }
}
