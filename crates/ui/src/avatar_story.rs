use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    avatar::{Avatar, AvatarGroup},
    h_flex,
    scroll::ScrollableElement as _,
    v_flex, ActiveTheme as _, IconName, Sizable as _,
};

pub struct AvatarStory;

impl AvatarStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for AvatarStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Avatar and Avatar Group"))
                    .child("User and organization identity representations."),
            )
            .child(section(
                "Avatar with text",
                h_flex()
                    .flex_wrap()
                    .items_center()
                    .gap_3()
                    .child(Avatar::new().name("Jason Lee").large())
                    .child(Avatar::new().name("Floyd Wang"))
                    .child(Avatar::new().name("xda").small())
                    .child(Avatar::new().name("ihavecoke").xsmall()),
            ))
            .child(section(
                "Placeholder",
                h_flex()
                    .flex_wrap()
                    .items_center()
                    .gap_3()
                    .child(Avatar::new().large())
                    .child(Avatar::new())
                    .child(Avatar::new().small())
                    .child(Avatar::new().xsmall())
                    .child(Avatar::new().placeholder(IconName::Building2)),
            ))
            .child(section(
                "Avatar Group",
                v_flex()
                    .gap_4()
                    .child(
                        AvatarGroup::new()
                            .child(Avatar::new().name("Alice"))
                            .child(Avatar::new().name("Bob"))
                            .child(Avatar::new().name("Charlie"))
                            .child(Avatar::new().name("David")),
                    )
                    .child(
                        AvatarGroup::new()
                            .small()
                            .limit(3)
                            .child(Avatar::new().name("Alice"))
                            .child(Avatar::new().name("Bob"))
                            .child(Avatar::new().name("Charlie"))
                            .child(Avatar::new().name("David"))
                            .child(Avatar::new().name("Eve")),
                    )
                    .child(
                        AvatarGroup::new()
                            .xsmall()
                            .limit(3)
                            .ellipsis()
                            .child(Avatar::new().name("Alice"))
                            .child(Avatar::new().name("Bob"))
                            .child(Avatar::new().name("Charlie"))
                            .child(Avatar::new().name("David"))
                            .child(Avatar::new().name("Eve")),
                    ),
            ))
            .child(section(
                "Custom Styling",
                h_flex()
                    .items_center()
                    .gap_4()
                    .child(
                        Avatar::new()
                            .name("Rounded")
                            .with_size(px(72.))
                            .rounded(px(16.)),
                    )
                    .child(
                        Avatar::new()
                            .name("Bordered")
                            .with_size(px(72.))
                            .border_3()
                            .border_color(cx.theme().foreground),
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
