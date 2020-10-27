use amethyst::renderer::rendy::mesh::{MeshBuilder, Normal, Position, TexCoord};
use amethyst::renderer::types::MeshData;
use amethyst::Error;
use amethyst::core::math::Vector3;

#[derive(Debug)]
pub struct Modifier {
    sizes: Option<(f32, f32, f32)>
}

impl Modifier {
    pub fn of(size_modifiers: Option<(f32, f32, f32)>) -> Modifier {
        Modifier {
            sizes: size_modifiers
        }
    }
}

#[derive(Debug)]
pub enum FaceType {
    Left(Modifier), Right(Modifier),
    Top(Modifier), Bottom(Modifier),
    Front(Modifier), Back(Modifier),
}

pub enum CubeVertices {
    FrontTopLeft,
    FrontTopRight,
    FrontBottomLeft,
    FrontBottomRight,
    BackTopLeft,
    BackTopRight,
    BackBottomLeft,
    BackBottomRight,
}

static FRONT_FACE_VERTICES: &[CubeVertices] = &[CubeVertices::FrontTopLeft, CubeVertices::FrontBottomLeft, CubeVertices::FrontTopRight,
    CubeVertices::FrontTopRight, CubeVertices::FrontBottomLeft, CubeVertices::FrontBottomRight];

static BACK_FACE_VERTICES: &[CubeVertices] = &[CubeVertices::BackBottomLeft, CubeVertices::BackTopLeft, CubeVertices::BackTopRight,
    CubeVertices::BackTopRight,CubeVertices::BackBottomRight,CubeVertices::BackBottomLeft ];

static LEFT_FACE_VERTICES: &[CubeVertices] = &[CubeVertices::FrontTopLeft, CubeVertices::BackTopLeft, CubeVertices::FrontBottomLeft,
    CubeVertices::FrontBottomLeft, CubeVertices::BackTopLeft, CubeVertices::BackBottomLeft];

static RIGHT_FACE_VERTICES: &[CubeVertices] = &[CubeVertices::FrontBottomRight, CubeVertices::BackTopRight, CubeVertices::FrontTopRight,
    CubeVertices::BackTopRight, CubeVertices::FrontBottomRight, CubeVertices::BackBottomRight];

static TOP_FACE_VERTICES: &[CubeVertices] = &[CubeVertices::FrontTopLeft, CubeVertices::FrontTopRight, CubeVertices::BackTopLeft,
    CubeVertices::BackTopLeft, CubeVertices::FrontTopRight, CubeVertices::BackTopRight];

static BOTTOM_FACE_VERTICES: &[CubeVertices] = &[CubeVertices::FrontBottomLeft, CubeVertices::BackBottomLeft, CubeVertices::FrontBottomRight,
CubeVertices::FrontBottomRight, CubeVertices::BackBottomLeft, CubeVertices::BackBottomRight];

pub fn create_cuboid(size_modifier: Option<(f32, f32, f32)>) -> Result<MeshData, Error> {
    create_multiple_quads(vec![
        FaceType::Back(Modifier::of(size_modifier)),
        FaceType::Front(Modifier::of(size_modifier)),
        FaceType::Left(Modifier::of(size_modifier)),
        FaceType::Right(Modifier::of(size_modifier)),
        FaceType::Top(Modifier::of(size_modifier)),
        FaceType::Bottom(Modifier::of(size_modifier)),
    ])
}//Right back top

pub fn create_multiple_quads(sides: Vec<FaceType>) -> Result<MeshData, Error> {
    let mut result = (
        Vec::<Position>::new(),
        Vec::<Normal>::new(),
        Vec::<TexCoord>::new(),
    );
    sides.into_iter().for_each(|side| {
        let mut tris = create_quad_tris(side);
        result.0.append(&mut tris.0);
        result.1.append(&mut tris.1);
        result.2.append(&mut tris.2);
    });
    let res = MeshBuilder::new()
        .with_vertices(result.0)
        .with_vertices(result.1)
        .with_vertices(result.2)
        .into();
    Ok(res)
}

fn create_quad_tris(cube_side: FaceType) -> (Vec<Position>, Vec<Normal>, Vec<TexCoord>) {
    let result = match cube_side {
        FaceType::Front(modifier) => (
            FRONT_FACE_VERTICES.iter().map(|v| vertice_of(v, &modifier)).collect(),
            vec![Normal([0., 0., 1.]); 6],
            vec![TexCoord([0., 0.]); 6]),
        FaceType::Back(modifier) => (
            BACK_FACE_VERTICES.iter().map(|v| vertice_of(v, &modifier)).collect(),
            vec![Normal([0., 0., -1.]); 6],
            vec![TexCoord([0., 0.]); 6]),
        FaceType::Right(modifier) => (
            RIGHT_FACE_VERTICES.iter().map(|v| vertice_of(v, &modifier)).collect(),
            vec![Normal([0., 1., 0.]); 6],
            vec![TexCoord([0., 0.]); 6]),
        FaceType::Left(modifier) => (
            LEFT_FACE_VERTICES.iter().map(|v| vertice_of(v, &modifier)).collect(),
            vec![Normal([0., -1., 0.]); 6],
            vec![TexCoord([0., 0.]); 6]),
        FaceType::Top(modifier) => (
            TOP_FACE_VERTICES.iter().map(|v| vertice_of(v, &modifier)).collect(),
            vec![Normal([0., 0., 1.]); 6],
            vec![TexCoord([0., 0.]); 6]),
        FaceType::Bottom(modifier) => (
            BOTTOM_FACE_VERTICES.iter().map(|v| vertice_of(v, &modifier)).collect(),
            vec![ Normal([-1., 0., 0.]); 6],
            vec![TexCoord([0., 0.]); 6]),
    };
    result
}

fn vertice_of(cube_vertice: &CubeVertices, modifier: &Modifier) -> Position {
    let size_modifier = modifier.sizes.unwrap_or((1.,1.,1.));
    let result = match cube_vertice {
        CubeVertices::FrontTopLeft => (0., 0., 1.),
        CubeVertices::FrontTopRight => (1., 0., 1.),
        CubeVertices::FrontBottomRight => (1., 0., 0.),
        CubeVertices::FrontBottomLeft => (0., 0., 0.),
        CubeVertices::BackTopLeft => (0., 1., 1.),
        CubeVertices::BackTopRight => (1., 1., 1.),
        CubeVertices::BackBottomRight => (1., 1., 0.),
        CubeVertices::BackBottomLeft => (0., 1., 0.),
    };
    Position([result.0 * size_modifier.0, result.1 * size_modifier.1, result.2 * size_modifier.2])
}
