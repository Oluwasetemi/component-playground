use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    avatar::Avatar, badge::Badge, h_flex, scroll::ScrollableElement as _, v_flex, ActiveTheme as _,
    Icon, IconName, Sizable as _,
};

pub struct BadgeStory;

impl BadgeStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for BadgeStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Badge"))
                    .child("Counts, dots, and status icons attached to content."),
            )
            .child(section(
                "Badge on Icon",
                h_flex()
                    .items_center()
                    .gap_5()
                    .child(
                        Badge::new()
                            .count(3)
                            .child(Icon::new(IconName::Bell).large()),
                    )
                    .child(
                        Badge::new()
                            .count(103)
                            .max(99)
                            .child(Icon::new(IconName::Inbox).large()),
                    ),
            ))
            .child(section(
                "Badge with Count",
                h_flex()
                    .items_center()
                    .gap_5()
                    .child(
                        Badge::new()
                            .count(3)
                            .child(Avatar::new().name("Alice").large()),
                    )
                    .child(
                        Badge::new()
                            .count(103)
                            .child(Avatar::new().name("Bob").large()),
                    ),
            ))
            .child(section(
                "Badge with Icon",
                h_flex()
                    .items_center()
                    .gap_5()
                    .child(
                        Badge::new()
                            .icon(IconName::Check)
                            .color(cx.theme().green)
                            .child(Avatar::new().name("Ready").large()),
                    )
                    .child(
                        Badge::new()
                            .icon(IconName::Star)
                            .color(cx.theme().yellow)
                            .child(Avatar::new().name("Starred").large()),
                    ),
            ))
            .child(section(
                "Badge with Dot",
                h_flex()
                    .items_center()
                    .gap_5()
                    .child(
                        Badge::new()
                            .dot()
                            .child(Avatar::new().name("Online").large()),
                    )
                    .child(
                        Badge::new()
                            .dot()
                            .count(1)
                            .color(cx.theme().green)
                            .child(Avatar::new().name("Active").large()),
                    ),
            ))
            .child(section(
                "Nested Badges",
                h_flex()
                    .items_center()
                    .gap_5()
                    .child(
                        Badge::new().count(212).large().child(
                            Badge::new()
                                .icon(IconName::Check)
                                .large()
                                .color(cx.theme().cyan)
                                .child(Avatar::new().name("Done").large()),
                        ),
                    )
                    .child(
                        Badge::new().dot().child(
                            Badge::new()
                                .icon(IconName::Sun)
                                .small()
                                .color(cx.theme().red)
                                .child(Avatar::new().name("Alert").small()),
                        ),
                    ),
            ))
    }
}

fn section(title: &'static str, content: impl IntoElement) -> impl IntoElement {
    v_flex()
        .gap_3()
        .child(div().text_size(px(16.)).child(title))
        .child(content)
}
