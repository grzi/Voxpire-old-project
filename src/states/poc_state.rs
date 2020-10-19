use amethyst::{GameData, SimpleState, SimpleTrans, StateData, Trans};

use crate::utilities::geometry::meshdata_helper;
use amethyst::assets::{AssetLoaderSystemData, Handle, Loader};
use amethyst::core::ecs::{Builder, World, WorldExt};
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::renderer::light::{Light, PointLight};
use amethyst::renderer::palette::{Srgb, Srgba};
use amethyst::renderer::rendy::texture::palette::load_from_srgba;
use amethyst::renderer::{Camera, Material, MaterialDefaults};
use amethyst::ui::TtfFormat;
use log::info;

#[derive(Default)]
pub struct PocState;

impl SimpleState for PocState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialise_camera(world);
        initialise_lights(world);
        initialise_ui(world);
        initialise_cube(world);
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(-5., -5.0, -10.);
    transform.face_towards(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));

    world
        .create_entity()
        .with(Camera::standard_3d(100., 100.))
        .with(transform)
        .build();
}

fn initialise_lights(world: &mut World) {
    let light: Light = PointLight {
        intensity: 100.0,
        radius: 10.0,
        color: Srgb::new(1.0, 1.0, 1.0),
        ..Default::default()
    }
    .into();

    let mut transform = Transform::default();

    transform.set_translation_xyz(-10., -4.0, -10.0);
    world.create_entity().with(light).with(transform).build();
}

fn initialise_ui(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "fonts/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let hour_transform = crate::utilities::ui::ui_helper::horizontal_grid_transform(7, 8, String::from("test_hour"));
    let header_transform = crate::utilities::ui::ui_helper::header_bar_transform();
    let ui_text = crate::utilities::ui::ui_helper::create_header_ui_text(font, String::from("23:17 "));
    world
        .create_entity()
        .with(header_transform)
        .with(crate::utilities::ui::ui_helper::create_header_image())
        .build();
    world
        .create_entity()
        .with(ui_text)
        .with(hour_transform)
        .build();
}

fn initialise_cube(world: &mut World) {
    let (mesh, mat) = {
        let mesh = world.exec(
            |loader: AssetLoaderSystemData<amethyst::renderer::types::Mesh>| {
                loader.load_from_data(meshdata_helper::create_cuboid(None).unwrap(), ())
            },
        );

        let textures = &world.read_resource();
        let mat_defaults = &world.read_resource::<MaterialDefaults>();
        let materials = &world.read_resource();
        let loader = &world.read_resource::<Loader>();

        let albedo = loader.load_from_data(
            load_from_srgba(Srgba::new(0.4, 0.5, 0.3, 1.0)).into(),
            (),
            textures,
        );
        let mat: Handle<Material> = loader.load_from_data(
            Material {
                albedo,
                ..mat_defaults.0.clone()
            },
            (),
            materials,
        );
        (mesh, mat)
    };
    let mut trans = Transform::default();
    trans.set_translation_xyz(-0.5, 0.0, -0.5);
    world
        .create_entity()
        .with(mesh)
        .with(trans)
        .with(mat)
        .build();
}
