use amethyst::ui::{UiTransform, Anchor, ScaleMode, UiText, FontHandle, LineMode, UiImage};



pub fn header_bar_transform() -> UiTransform {
    let mut t = UiTransform::new(
        String::from("voxpire-header"),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.,
        0.,
        0.,
        1.0,
        0.03,
    );
    t.scale_mode = ScaleMode::Percent;
    t
}

pub fn horizontal_grid_transform(index: u8, total_slots: u8, name: String) -> UiTransform {
    let mut t = UiTransform::new(
        name,
        Anchor::TopLeft,
        Anchor::TopLeft,
        index as f32 /total_slots as f32,
        0.,
        0.,
        1.0 / total_slots as f32,
        0.03,
    );
    t.scale_mode = ScaleMode::Percent;
    t
}

pub fn create_header_ui_text(font: FontHandle, text: String) -> UiText {
    UiText::new(font,
                text,
                [0.5, 0.5, 0.5, 1.],
                30.,
                LineMode::Wrap,
                Anchor::MiddleRight,
    )
}

pub fn create_header_image() -> UiImage {
    UiImage::SolidColor([0.1, 0.1, 0.1, 0.5])
}