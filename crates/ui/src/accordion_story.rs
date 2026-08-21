use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    accordion::Accordion,
    button::{Button, ButtonGroup},
    checkbox::Checkbox,
    h_flex,
    scroll::ScrollableElement as _,
    switch::Switch,
    v_flex, IconName, Selectable as _, Sizable as _, Size,
};

pub struct AccordionStory {
    open_items: Vec<usize>,
    size: Size,
    bordered: bool,
    disabled: bool,
    multiple: bool,
    show_icons: bool,
}

impl AccordionStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            open_items: vec![0],
            size: Size::Medium,
            bordered: true,
            disabled: false,
            multiple: false,
            show_icons: false,
        }
    }

    fn controls(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(
                h_flex()
                    .flex_wrap()
                    .items_center()
                    .gap_3()
                    .child(div().text_size(px(14.)).child("Size"))
                    .child(
                        ButtonGroup::new("accordion-size")
                            .outline()
                            .compact()
                            .child(
                                Button::new("accordion-size-xsmall")
                                    .label("XSmall")
                                    .selected(self.size == Size::XSmall),
                            )
                            .child(
                                Button::new("accordion-size-small")
                                    .label("Small")
                                    .selected(self.size == Size::Small),
                            )
                            .child(
                                Button::new("accordion-size-medium")
                                    .label("Medium")
                                    .selected(self.size == Size::Medium),
                            )
                            .child(
                                Button::new("accordion-size-large")
                                    .label("Large")
                                    .selected(self.size == Size::Large),
                            )
                            .on_click(cx.listener(|this, selected: &Vec<usize>, _, cx| {
                                if let Some(index) = selected.first() {
                                    this.size = match index {
                                        0 => Size::XSmall,
                                        1 => Size::Small,
                                        2 => Size::Medium,
                                        3 => Size::Large,
                                        _ => return,
                                    };
                                    cx.notify();
                                }
                            })),
                    ),
            )
            .child(
                h_flex()
                    .flex_wrap()
                    .gap_3()
                    .child(
                        Checkbox::new("accordion-multiple")
                            .label("Multiple")
                            .checked(self.multiple)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.multiple = *checked;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("accordion-icons")
                            .label("Icons")
                            .checked(self.show_icons)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.show_icons = *checked;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("accordion-disabled")
                            .label("Disabled")
                            .checked(self.disabled)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.disabled = *checked;
                                cx.notify();
                            })),
                    )
                    .child(
                        Checkbox::new("accordion-bordered")
                            .label("Bordered")
                            .checked(self.bordered)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.bordered = *checked;
                                cx.notify();
                            })),
                    ),
            )
    }

    fn accordion(&self, cx: &mut Context<Self>) -> Accordion {
        Accordion::new("accordion-story")
            .bordered(self.bordered)
            .with_size(self.size)
            .disabled(self.disabled)
            .multiple(self.multiple)
            .item(|item| {
                item.open(self.open_items.contains(&0))
                    .when(self.show_icons, |item| item.icon(IconName::Info))
                    .title("Is it accessible?")
                    .child("Yes. It follows the WAI-ARIA accordion pattern.")
            })
            .item(|item| {
                item.open(self.open_items.contains(&1))
                    .when(self.show_icons, |item| item.icon(IconName::Inbox))
                    .title("Can it contain complex content?")
                    .child(
                        v_flex()
                            .gap_3()
                            .child("Accordion content can contain any GPUI element.")
                            .child(
                                h_flex()
                                    .gap_3()
                                    .child(Switch::new("accordion-switch").label("Switch"))
                                    .child(Checkbox::new("accordion-checkbox").label("Checkbox")),
                            ),
                    )
            })
            .item(|item| {
                item.open(self.open_items.contains(&2))
                    .when(self.show_icons, |item| item.icon(IconName::Moon))
                    .title("Can it be used without borders?")
                    .child("Yes. Disable Bordered above to see the borderless presentation.")
            })
            .on_toggle_click(cx.listener(|this, open_items: &[usize], _, cx| {
                this.open_items = open_items.to_vec();
                cx.notify();
            }))
    }
}

impl Render for AccordionStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Accordion"))
                    .child("Expandable sections with controlled open state."),
            )
            .child(self.controls(cx))
            .child(StorySection::new(
                "Interactive Accordion",
                self.accordion(cx),
            ))
    }
}
