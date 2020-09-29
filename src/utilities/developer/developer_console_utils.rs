use amethyst::ui::{UiTransform, Anchor, ScaleMode, UiText, TextEditing};

pub fn create_input_transform () -> UiTransform{
    let mut t = UiTransform::new(
        String::from("developer-console-input-transform"),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.,
        0.0,
        0.,
        1.0,
        0.03,
    );
    t.scale_mode = ScaleMode::Percent;
    t
}

pub fn create_ui_text() -> UiText {
    unimplemented!()
}

pub fn create_text_editing() -> TextEditing {
    unimplemented!()
}
