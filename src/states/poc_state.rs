use amethyst::{GameData, SimpleState, SimpleTrans, StateData, Trans};

use crate::utilities::geometry::meshdata_helper;
use amethyst::assets::{AssetLoaderSystemData, Handle, Loader};
use amethyst::core::ecs::{Builder, World, WorldExt};
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::renderer::light::{Light, PointLight, SunLight, DirectionalLight};
use amethyst::renderer::palette::{Srgb, Srgba};
use amethyst::renderer::rendy::texture::palette::load_from_srgba;
use amethyst::renderer::{Camera, Material, MaterialDefaults};
use amethyst::ui::TtfFormat;
use crate::utilities::traits::Tickable;
use crate::states::CurrentState;
use crate::engines::time::TimeComponent;
use crate::engines::time::time_resource::TimeResource;
use crate::engines::terrain::data_model::generate_fake_island;
use crate::engines::terrain::bloc::Bloc;

#[derive(Default)]
pub struct PocState;

impl SimpleState for PocState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        *data.world.write_resource::<CurrentState>() = CurrentState::InGame;
        let world = data.world;
        initialise_camera(world);
        initialise_lights(world);
        initialise_ui(world);
        initialise_cubes(world);
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        Trans::None
    }

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        data.world.entry::<TimeResource>().or_insert(TimeResource::new()).tick();
        Trans::None
    }
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(-5., -5., 25.);
    transform.face_towards(Vector3::new(25.0, 25.0, 1.0), Vector3::new(0.0, 0.0, 1.0));

    world
        .create_entity()
        .with(Camera::standard_3d(1920., 1080.))
        .with(transform)
        .build();
}

fn initialise_lights(world: &mut World) {
    let light: Light = DirectionalLight {
        color: Srgb::new(1.0, 1.0, 1.0),
        direction:  [0., 0., -1.0].into(),
        intensity: 1.0,
    }.into();
    let mut transform = Transform::default();

    transform.set_translation_xyz(50., 50.0, 26.);

    world.create_entity().with(light).with(transform).build();
}

fn initialise_ui(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "fonts/ubuntu-regular.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let hour_transform = crate::ui::ui_helper::horizontal_grid_transform(7, 8, String::from("test_hour"));
    let header_transform = crate::ui::ui_helper::header_bar_transform();
    let ui_text = crate::ui::ui_helper::create_header_ui_text(font, String::from(""));
    world
        .create_entity()
        .with(header_transform)
        .with(crate::ui::ui_helper::create_header_image())
        .build();
    world
        .create_entity()
        .with(ui_text)
        .with(hour_transform)
        .with(TimeComponent)
        .build();
}

fn initialise_cubes(world: &mut World) {
    let island = generate_fake_island();
    island.chunks().iter().for_each(|chunk_column| {
        chunk_column.into_iter().for_each(|chunk| {
            chunk.squares().iter().for_each(|square_column|{
                square_column.iter().for_each(|square| {
                    initialize_cube(world, square.blocs().iter().last().unwrap());
                })
            })
        })
    })
}

fn initialize_cube(world: &mut World, bloc: &Bloc) {
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
            load_from_srgba(Srgba::new(0., 0.4, 0., 1.0)).into(),
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
    trans.set_translation_xyz(*(bloc.coordinates().x()) as f32, *(bloc.coordinates().y()) as f32, bloc.coordinates().z().unwrap() as f32);
    world
        .create_entity()
        .with(mesh)
        .with(trans)
        .with(mat)
        .build();
}
