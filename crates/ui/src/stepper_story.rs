use crate::StorySection;
use gpui::{div, prelude::*, px, Context, IntoElement, Render, Window};
use gpui_component::{
    button::{Button, ButtonGroup},
    checkbox::Checkbox,
    h_flex,
    scroll::ScrollableElement as _,
    stepper::{Stepper, StepperItem},
    v_flex, IconName, Selectable as _, Sizable as _, Size,
};

pub struct StepperStory {
    size: Size,
    selected: usize,
    disabled: bool,
}

impl StepperStory {
    pub fn new(_: &mut Window, _: &mut Context<Self>) -> Self {
        Self {
            size: Size::Medium,
            selected: 1,
            disabled: false,
        }
    }
}

impl Render for StepperStory {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_y_scrollbar()
            .gap_6()
            .p_6()
            .child(
                v_flex()
                    .gap_1()
                    .child(div().text_size(px(24.)).child("Stepper"))
                    .child("Step-by-step progress through a workflow."),
            )
            .child(
                h_flex()
                    .items_center()
                    .gap_3()
                    .child(
                        ButtonGroup::new("stepper-size")
                            .outline()
                            .compact()
                            .child(
                                Button::new("stepper-small")
                                    .label("Small")
                                    .selected(self.size == Size::Small),
                            )
                            .child(
                                Button::new("stepper-medium")
                                    .label("Medium")
                                    .selected(self.size == Size::Medium),
                            )
                            .child(
                                Button::new("stepper-large")
                                    .label("Large")
                                    .selected(self.size == Size::Large),
                            )
                            .on_click(cx.listener(|this, selected: &Vec<usize>, _, cx| {
                                if let Some(index) = selected.first() {
                                    this.size = match index {
                                        0 => Size::Small,
                                        1 => Size::Medium,
                                        2 => Size::Large,
                                        _ => return,
                                    };
                                    cx.notify();
                                }
                            })),
                    )
                    .child(
                        Checkbox::new("stepper-disabled")
                            .label("Disabled")
                            .checked(self.disabled)
                            .on_click(cx.listener(|this, checked, _, cx| {
                                this.disabled = *checked;
                                cx.notify();
                            })),
                    ),
            )
            .child(StorySection::new(
                "Horizontal",
                Stepper::new("stepper-horizontal")
                    .with_size(self.size)
                    .disabled(self.disabled)
                    .selected_index(self.selected)
                    .items([
                        StepperItem::new().child("Cart"),
                        StepperItem::new().child("Shipping"),
                        StepperItem::new().child("Payment"),
                        StepperItem::new().child("Complete"),
                    ])
                    .on_click(cx.listener(|this, step, _, cx| {
                        this.selected = *step;
                        cx.notify();
                    })),
            ))
            .child(StorySection::new(
                "With Icons",
                Stepper::new("stepper-icons")
                    .with_size(self.size)
                    .selected_index(self.selected)
                    .items([
                        StepperItem::new().icon(IconName::Calendar).child("Details"),
                        StepperItem::new().icon(IconName::Inbox).child("Shipping"),
                        StepperItem::new()
                            .icon(IconName::CircleCheck)
                            .child("Finish"),
                    ])
                    .on_click(cx.listener(|this, step, _, cx| {
                        this.selected = *step;
                        cx.notify();
                    })),
            ))
            .child(StorySection::new(
                "Vertical",
                Stepper::new("stepper-vertical")
                    .vertical()
                    .with_size(self.size)
                    .selected_index(self.selected)
                    .items([
                        StepperItem::new()
                            .icon(IconName::Building2)
                            .child(v_flex().child("Account").child("Set up your profile.")),
                        StepperItem::new()
                            .icon(IconName::Folder)
                            .child(v_flex().child("Workspace").child("Choose a workspace.")),
                        StepperItem::new()
                            .icon(IconName::CircleCheck)
                            .child(v_flex().child("Done").child("You are ready to go.")),
                    ])
                    .on_click(cx.listener(|this, step, _, cx| {
                        this.selected = *step;
                        cx.notify();
                    })),
            ))
    }
}
