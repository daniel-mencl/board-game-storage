use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
}

#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] disabled: bool,
    on_click: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let variant_class = match variant {
        ButtonVariant::Primary => "primary",
        ButtonVariant::Secondary => "secondary",
        ButtonVariant::Danger => "danger",
    };

    rsx! {
        button {
            class: "btn {variant_class}",
            disabled: disabled,
            onclick: move |evt| on_click.call(evt),
            {children}
        }
    }
}
