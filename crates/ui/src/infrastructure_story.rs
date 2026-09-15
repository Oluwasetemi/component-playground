use crate::{StorySection, Theme};
use gpui::{
    div, prelude::*, px, Animation, AnimationExt as _, Context, Hsla, IntoElement, Render, Window,
};
use gpui_component::{
    button::Button,
    clipboard::Clipboard,
    h_flex,
    input::{Input, InputState},
    scroll::ScrollableElement as _,
    v_flex, window_border, ActiveTheme as _, TitleBar, WindowExt as _,
};
use std::time::Duration;

#[derive(Clone, Copy)]
pub enum InfrastructureKind {
    Theme,
    AnimationAndTransitions,
    WindowBorder,
    TitleBar,
    Clipboard,
    History,
    ComponentRoot,
}

pub struct InfrastructureStory {
    kind: InfrastructureKind,
    foreground: Hsla,
    clipboard_input: gpui::Entity<InputState>,
    history: Vec<&'static str>,
}

impl InfrastructureStory {
    pub fn new(kind: InfrastructureKind, window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            kind,
            foreground: Theme::groknight().foreground,
            clipboard_input: cx
                .new(|cx| InputState::new(window, cx).default_value("component-playground")),
            history: vec![
                "Opened Button story",
                "Changed theme",
                "Copied project name",
            ],
        }
    }

    fn shell(
        &self,
        title: &str,
        description: &str,
    ) -> gpui_component::scroll::Scrollable<gpui::Div> {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .text_color(self.foreground)
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child(title.to_string()))
                    .child(description.to_string()),
            )
    }

    fn panel(&self, content: impl IntoElement) -> impl IntoElement {
        div()
            .border_1()
            .border_color(gpui::rgba(0x2a2a2a99))
            .rounded_md()
            .p_4()
            .text_color(self.foreground)
            .child(content)
    }
}

impl Render for InfrastructureStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        match self.kind {
            InfrastructureKind::Theme => {
                let theme = Theme::groknight();
                let swatch = |name: &str, color: Hsla| {
                    h_flex()
                        .gap_3()
                        .items_center()
                        .child(div().size_6().rounded_sm().bg(color))
                        .child(name.to_string())
                };
                self.shell(
                    "Theme",
                    "Runtime tokens keep the playground palette consistent.",
                )
                .child(StorySection::new(
                    "Groknight palette",
                    self.panel(
                        v_flex()
                            .gap_3()
                            .child(swatch("Foreground", theme.foreground))
                            .child(swatch("Accent", theme.accent))
                            .child(swatch("Success", theme.green))
                            .child(swatch("Warning", theme.yellow))
                            .child(swatch("Danger", theme.red)),
                    ),
                ))
            }
            InfrastructureKind::AnimationAndTransitions => self
                .shell(
                    "Animation and Transitions",
                    "Declarative animations can be applied directly to styled elements.",
                )
                .child(StorySection::new(
                    "Repeating animation",
                    self.panel(
                        div()
                            .id("infrastructure-animation")
                            .h(px(100.))
                            .w(px(160.))
                            .bg(cx.theme().primary)
                            .with_animation(
                                "infrastructure-pulse",
                                Animation::new(Duration::from_secs(2)).repeat(),
                                |this, delta| this.opacity(0.35 + delta * 0.65),
                            ),
                    ),
                )),
            InfrastructureKind::WindowBorder => self
                .shell(
                    "Window Border",
                    "The component root can provide platform-aware border and resize behavior.",
                )
                .child(StorySection::new(
                    "Border preview",
                    self.panel(
                        window_border().shadow_size(px(12.)).child(
                            v_flex()
                                .h(px(140.))
                                .items_center()
                                .justify_center()
                                .child("Window border content"),
                        ),
                    ),
                )),
            InfrastructureKind::TitleBar => self
                .shell(
                    "Title Bar",
                    "A custom title bar can host app identity and window controls.",
                )
                .child(StorySection::new(
                    "Title bar preview",
                    self.panel(
                        TitleBar::new().child(
                            h_flex()
                                .gap_2()
                                .child("Component Playground")
                                .child("Infrastructure"),
                        ),
                    ),
                )),
            InfrastructureKind::Clipboard => self
                .shell(
                    "Clipboard",
                    "Copy values without duplicating clipboard state in each view.",
                )
                .child(StorySection::new(
                    "Copy from input",
                    self.panel(
                        Input::new(&self.clipboard_input).suffix(
                            Clipboard::new("infrastructure-clipboard")
                                .value_fn({
                                    let state = self.clipboard_input.clone();
                                    move |_, cx| state.read(cx).value()
                                })
                                .tooltip("Copy value")
                                .on_copied(|value, window, cx| {
                                    window.push_notification(format!("Copied: {value}"), cx);
                                }),
                        ),
                    ),
                )),
            InfrastructureKind::History => self
                .shell(
                    "History",
                    "A compact activity trail for recent application actions.",
                )
                .child(StorySection::new(
                    "Recent activity",
                    self.panel(
                        v_flex()
                            .gap_2()
                            .children(self.history.iter().enumerate().map(|(index, entry)| {
                                h_flex()
                                    .gap_3()
                                    .child(
                                        div()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(format!("{}.", index + 1)),
                                    )
                                    .child(*entry)
                            }))
                            .child(
                                Button::new("clear-history")
                                    .label("Clear history")
                                    .outline()
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.history.clear();
                                        cx.notify();
                                    })),
                            ),
                    ),
                )),
            InfrastructureKind::ComponentRoot => self
                .shell(
                    "Component Root",
                    "The top-level root owns overlays, notifications, focus, and theme state.",
                )
                .child(StorySection::new(
                    "Application composition",
                    self.panel(
                        v_flex()
                            .gap_2()
                            .child("gpui_component::Root")
                            .child("  └─ RootView")
                            .child("      ├─ Sidebar")
                            .child("      ├─ Story content")
                            .child("      └─ Dialog, sheet, notification layers"),
                    ),
                )),
        }
    }
}
