use amethyst::ui::{UiTransform, Anchor, ScaleMode, UiText, TextEditing, LineMode, FontHandle, FontAsset};
use crate::utilities::developer::developer_console::DeveloperConsoleResource;
use amethyst::assets::{Loader, AssetStorage};
use amethyst::core::ecs::{Write, Read, ReadExpect};

pub fn create_input_transform () -> UiTransform{
    let mut t = UiTransform::new(
        String::from("developer-console-input-transform"),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.,
        -0.20,
        0.,
        1.0,
        0.03,
    );
    t.scale_mode = ScaleMode::Percent;
    t
}

pub fn create_output_transform () -> UiTransform {
    let mut t = UiTransform::new(
        String::from("developer-console-output-transform"),
        Anchor::TopLeft,
        Anchor::TopLeft,
        0.,
        0.,
        0.,
        1.0,
        0.2,
    );
    t.scale_mode = ScaleMode::Percent;
    t
}

pub fn create_ui_text(developer_resource: &Write<DeveloperConsoleResource>,
                      loader: &ReadExpect<Loader>,
                      font_asset: &Read<AssetStorage<FontAsset>>) -> UiText {
    let font = developer_resource.font_handle.clone();
    let font = if let Some(f) = font {
        f
    }else{
        amethyst::ui::get_default_font(loader, font_asset)
    };

  UiText::new(font,
        String::from(""),
        [1.0, 1.0, 1.0, 1.],
        25.,
        LineMode::Single,
        Anchor::MiddleLeft,
    )
}

pub fn create_output_text(developer_resource: &Write<DeveloperConsoleResource>,
                      loader: &ReadExpect<Loader>,
                      font_asset: &Read<AssetStorage<FontAsset>>) -> UiText {
    let font = developer_resource.font_handle.clone();
    let font = if let Some(f) = font {
        f
    }else{
        amethyst::ui::get_default_font(loader, font_asset)
    };

    UiText::new(font,
                String::from(""),
                [1.0, 1.0, 1.0, 1.],
                25.,
                LineMode::Wrap,
                Anchor::BottomLeft,
    )
}
pub fn create_text_editing() -> TextEditing {
    TextEditing::new(100, [0., 0.5, 0.2, 1.], [0., 1., 1., 1.], true)
}
