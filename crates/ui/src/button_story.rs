use crate::StorySection;
use gpui::{div, prelude::*, px, App, Axis, ClickEvent, Context, IntoElement, Render, Window};
use gpui_component::{
    button::{Button as ComponentButton, ButtonCustomVariant, ButtonGroup, ButtonVariants as _},
    checkbox::Checkbox,
    h_flex,
    scroll::ScrollableElement as _,
    v_flex, ActiveTheme as _, Colorize as _, Disableable as _, IconName, Selectable as _,
    Sizable as _,
};

pub struct ButtonStory {
    disabled: bool,
    loading: bool,
    selected: bool,
    compact: bool,
    multiple: bool,
    horizontal_selection: Vec<usize>,
    vertical_selection: Vec<usize>,
    toggle_selection: Vec<usize>,
}

impl ButtonStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            disabled: false,
            loading: false,
            selected: false,
            compact: false,
            multiple: false,
            horizontal_selection: Vec::new(),
            vertical_selection: Vec::new(),
            toggle_selection: Vec::new(),
        }
    }

    fn log_click(_: &ClickEvent, _: &mut Window, _: &mut App) {
        println!("Longbridge button clicked");
    }

    fn apply_state(&self, button: ComponentButton) -> ComponentButton {
        button
            .disabled(self.disabled)
            .selected(self.selected)
            .loading(self.loading)
            .when(self.compact, |this| this.compact())
            .on_click(Self::log_click)
    }

    fn state_controls(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .flex_wrap()
            .gap_3()
            .child(
                Checkbox::new("button-story-disabled")
                    .label("Disabled")
                    .checked(self.disabled)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.disabled = !this.disabled;
                        cx.notify();
                    })),
            )
            .child(
                Checkbox::new("button-story-loading")
                    .label("Loading")
                    .checked(self.loading)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.loading = !this.loading;
                        cx.notify();
                    })),
            )
            .child(
                Checkbox::new("button-story-selected")
                    .label("Selected")
                    .checked(self.selected)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.selected = !this.selected;
                        cx.notify();
                    })),
            )
            .child(
                Checkbox::new("button-story-compact")
                    .label("Compact")
                    .checked(self.compact)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.compact = !this.compact;
                        cx.notify();
                    })),
            )
    }

    fn standard_variants(&self) -> impl IntoElement {
        StorySection::new(
            "Standard Variants",
            h_flex()
                .flex_wrap()
                .gap_2()
                .child(self.apply_state(ComponentButton::new("button-default").label("Default")))
                .child(
                    self.apply_state(
                        ComponentButton::new("button-primary")
                            .primary()
                            .label("Primary"),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("button-secondary")
                            .secondary()
                            .label("Secondary"),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("button-danger")
                            .danger()
                            .label("Danger"),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("button-warning")
                            .warning()
                            .label("Warning"),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("button-success")
                            .success()
                            .label("Success"),
                    ),
                )
                .child(self.apply_state(ComponentButton::new("button-info").info().label("Info")))
                .child(
                    self.apply_state(ComponentButton::new("button-ghost").ghost().label("Ghost")),
                )
                .child(self.apply_state(ComponentButton::new("button-link").link().label("Link")))
                .child(self.apply_state(ComponentButton::new("button-text").text().label("Text"))),
        )
    }

    fn outline_variants(&self) -> impl IntoElement {
        StorySection::new(
            "Outline Variants",
            h_flex()
                .flex_wrap()
                .gap_2()
                .child(
                    self.apply_state(
                        ComponentButton::new("outline-primary")
                            .primary()
                            .outline()
                            .label("Primary"),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("outline-secondary")
                            .secondary()
                            .outline()
                            .label("Secondary"),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("outline-danger")
                            .danger()
                            .outline()
                            .label("Danger"),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("outline-ghost")
                            .ghost()
                            .outline()
                            .label("Ghost"),
                    ),
                ),
        )
    }

    fn sizes(&self) -> impl IntoElement {
        StorySection::new(
            "Sizes",
            h_flex()
                .items_center()
                .flex_wrap()
                .gap_2()
                .child(
                    self.apply_state(
                        ComponentButton::new("button-large")
                            .primary()
                            .large()
                            .label("Large"),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("button-medium")
                            .primary()
                            .label("Medium (Default)"),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("button-small")
                            .secondary()
                            .small()
                            .label("Small"),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("button-xsmall")
                            .danger()
                            .xsmall()
                            .label("XSmall"),
                    ),
                ),
        )
    }

    fn content_examples(&self) -> impl IntoElement {
        StorySection::new(
            "Icons and Content",
            h_flex()
                .flex_wrap()
                .gap_2()
                .child(
                    self.apply_state(
                        ComponentButton::new("button-icon-label")
                            .primary()
                            .icon(IconName::Check)
                            .label("Confirm"),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("button-icon-only")
                            .secondary()
                            .icon(IconName::Search)
                            .tooltip("Search"),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("button-custom-content").child(
                            h_flex()
                                .gap_2()
                                .child("Custom")
                                .child(IconName::ChevronDown),
                        ),
                    ),
                )
                .child(
                    self.apply_state(
                        ComponentButton::new("button-dropdown")
                            .primary()
                            .dropdown_caret(true)
                            .label("Menu"),
                    ),
                ),
        )
    }

    fn custom_variant(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let magenta = cx.theme().magenta;
        let background = magenta.mix_oklab(cx.theme().transparent, 0.2);
        let custom = ButtonCustomVariant::new(cx)
            .color(magenta)
            .foreground(magenta)
            .hover(magenta.opacity(0.4))
            .active(background)
            .shadow(true);

        StorySection::new(
            "Custom Variant",
            h_flex().child(
                self.apply_state(
                    ComponentButton::new("button-custom")
                        .custom(custom)
                        .when(self.selected, |this| this.border_1().border_color(magenta))
                        .label("Custom Magenta"),
                ),
            ),
        )
    }

    fn horizontal_group(&self, cx: &mut Context<Self>) -> impl IntoElement {
        StorySection::new(
            "Horizontal Button Group",
            h_flex().child(
                ButtonGroup::new("button-group-horizontal")
                    .outline()
                    .disabled(self.disabled)
                    .child(
                        ComponentButton::new("group-one")
                            .label("One")
                            .selected(self.horizontal_selection.contains(&0)),
                    )
                    .child(
                        ComponentButton::new("group-two")
                            .label("Two")
                            .selected(self.horizontal_selection.contains(&1)),
                    )
                    .child(
                        ComponentButton::new("group-three")
                            .label("Three")
                            .selected(self.horizontal_selection.contains(&2)),
                    )
                    .on_click(cx.listener(|this, selected: &Vec<usize>, _, cx| {
                        this.horizontal_selection = selected.clone();
                        cx.notify();
                    })),
            ),
        )
    }

    fn vertical_group(&self, cx: &mut Context<Self>) -> impl IntoElement {
        StorySection::new(
            "Vertical Button Group",
            h_flex().child(
                ButtonGroup::new("button-group-vertical")
                    .outline()
                    .layout(Axis::Vertical)
                    .disabled(self.disabled)
                    .child(
                        ComponentButton::new("vertical-one")
                            .label("One")
                            .selected(self.vertical_selection.contains(&0)),
                    )
                    .child(
                        ComponentButton::new("vertical-two")
                            .label("Two")
                            .selected(self.vertical_selection.contains(&1)),
                    )
                    .child(
                        ComponentButton::new("vertical-three")
                            .label("Three")
                            .selected(self.vertical_selection.contains(&2)),
                    )
                    .on_click(cx.listener(|this, selected: &Vec<usize>, _, cx| {
                        this.vertical_selection = selected.clone();
                        cx.notify();
                    })),
            ),
        )
    }

    fn toggle_group(&self, cx: &mut Context<Self>) -> impl IntoElement {
        StorySection::new(
            "Toggle Button Group",
            v_flex()
                .gap_2()
                .child(
                    Checkbox::new("button-group-multiple")
                        .label("Allow multiple selection")
                        .checked(self.multiple)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.multiple = !this.multiple;
                            this.toggle_selection.clear();
                            cx.notify();
                        })),
                )
                .child(
                    ButtonGroup::new("button-group-toggle")
                        .outline()
                        .compact()
                        .multiple(self.multiple)
                        .child(
                            ComponentButton::new("toggle-bold")
                                .label("Bold")
                                .selected(self.toggle_selection.contains(&0)),
                        )
                        .child(
                            ComponentButton::new("toggle-italic")
                                .label("Italic")
                                .selected(self.toggle_selection.contains(&1)),
                        )
                        .child(
                            ComponentButton::new("toggle-underline")
                                .label("Underline")
                                .selected(self.toggle_selection.contains(&2)),
                        )
                        .on_click(cx.listener(|this, selected: &Vec<usize>, _, cx| {
                            this.toggle_selection = selected.clone();
                            cx.notify();
                        })),
                ),
        )
    }
}

impl Render for ButtonStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Button and Button Group"))
                    .child("Buttons communicate actions, states, and selection."),
            )
            .child(self.state_controls(cx))
            .child(self.standard_variants())
            .child(self.outline_variants())
            .child(self.sizes())
            .child(self.content_examples())
            .child(self.custom_variant(cx))
            .child(self.horizontal_group(cx))
            .child(self.vertical_group(cx))
            .child(self.toggle_group(cx))
    }
}
