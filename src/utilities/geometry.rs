use amethyst::renderer::rendy::mesh::{MeshBuilder, Normal, Position, TexCoord};
use amethyst::renderer::types::MeshData;
use amethyst::Error;

pub enum CubeSide {
    Left,
    Right,
    Top,
    Bottom,
    Front,
    Back,
}
enum CubeVertices {
    FrontTopLeft,
    FrontTopRight,
    FrontBottomLeft,
    FrontBottomRight,
    BackTopLeft,
    BackTopRight,
    BackBottomLeft,
    BackBottomRight,
}
pub fn create_cube() -> Result<MeshData, Error> {
    create_multiple_quads(vec![
        CubeSide::Left,
        CubeSide::Top,
        CubeSide::Right,
        CubeSide::Bottom,
        CubeSide::Front,
        CubeSide::Back,
    ])
}

pub fn create_multiple_quads(sides: Vec<CubeSide>) -> Result<MeshData, Error> {
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
    Ok(MeshBuilder::new()
        .with_vertices(result.0)
        .with_vertices(result.1)
        .with_vertices(result.2)
        .into())
}

fn create_quad_tris(cube_side: CubeSide) -> (Vec<Position>, Vec<Normal>, Vec<TexCoord>) {
    match cube_side {
        CubeSide::Front => (
            vec![
                vertice_of(CubeVertices::FrontTopLeft),
                vertice_of(CubeVertices::FrontBottomLeft),
                vertice_of(CubeVertices::FrontTopRight),
                vertice_of(CubeVertices::FrontTopRight),
                vertice_of(CubeVertices::FrontBottomLeft),
                vertice_of(CubeVertices::FrontBottomRight),
            ],
            vec![normal_of(CubeSide::Front); 6],
            vec![TexCoord([0., 0.]); 6],
        ),
        CubeSide::Top => (
            vec![
                vertice_of(CubeVertices::FrontTopLeft),
                vertice_of(CubeVertices::BackTopRight),
                vertice_of(CubeVertices::BackTopLeft),
                vertice_of(CubeVertices::BackTopRight),
                vertice_of(CubeVertices::FrontTopLeft),
                vertice_of(CubeVertices::FrontTopRight),
            ],
            vec![normal_of(CubeSide::Top); 6],
            vec![TexCoord([0., 0.]); 6],
        ),
        CubeSide::Right => (
            vec![
                vertice_of(CubeVertices::FrontTopRight),
                vertice_of(CubeVertices::BackTopRight),
                vertice_of(CubeVertices::FrontBottomRight),
                vertice_of(CubeVertices::FrontBottomRight),
                vertice_of(CubeVertices::BackTopRight),
                vertice_of(CubeVertices::BackBottomRight),
            ],
            vec![normal_of(CubeSide::Right); 6],
            vec![TexCoord([0., 0.]); 6],
        ),
        CubeSide::Bottom => (
            vec![
                vertice_of(CubeVertices::FrontBottomLeft),
                vertice_of(CubeVertices::BackBottomLeft),
                vertice_of(CubeVertices::FrontBottomRight),
                vertice_of(CubeVertices::FrontBottomRight),
                vertice_of(CubeVertices::BackBottomLeft),
                vertice_of(CubeVertices::BackBottomRight),
            ],
            vec![normal_of(CubeSide::Bottom); 6],
            vec![TexCoord([0., 0.]); 6],
        ),
        CubeSide::Left => (
            vec![
                vertice_of(CubeVertices::FrontTopLeft),
                vertice_of(CubeVertices::BackTopLeft),
                vertice_of(CubeVertices::FrontBottomLeft),
                vertice_of(CubeVertices::FrontBottomLeft),
                vertice_of(CubeVertices::BackTopLeft),
                vertice_of(CubeVertices::BackBottomLeft),
            ],
            vec![normal_of(CubeSide::Left); 6],
            vec![TexCoord([0., 0.]); 6],
        ),
        CubeSide::Back => (
            vec![
                vertice_of(CubeVertices::BackTopLeft),
                vertice_of(CubeVertices::BackBottomLeft),
                vertice_of(CubeVertices::BackTopRight),
                vertice_of(CubeVertices::BackTopRight),
                vertice_of(CubeVertices::BackBottomLeft),
                vertice_of(CubeVertices::BackBottomRight),
            ],
            vec![normal_of(CubeSide::Back); 6],
            vec![TexCoord([0., 0.]); 6],
        ),
    }
}

fn vertice_of(cube_vertice: CubeVertices) -> Position {
    match cube_vertice {
        CubeVertices::FrontTopLeft => return Position([0., 0., 1.]),
        CubeVertices::FrontTopRight => return Position([1., 0., 1.]),
        CubeVertices::FrontBottomRight => return Position([1., 0., 0.]),
        CubeVertices::FrontBottomLeft => return Position([0., 0., 0.]),
        CubeVertices::BackTopLeft => return Position([0., 1., 1.]),
        CubeVertices::BackTopRight => return Position([1., 1., 1.]),
        CubeVertices::BackBottomRight => return Position([1., 1., 0.]),
        CubeVertices::BackBottomLeft => return Position([0., 1., 0.]),
    };
}

fn normal_of(cube_side: CubeSide) -> Normal {
    match cube_side {
        CubeSide::Back => Normal([0., 0., 1.]),
        CubeSide::Front => Normal([0., 0., -1.]),
        CubeSide::Left => Normal([0., -1., 0.]),
        CubeSide::Right => Normal([0., 1., 0.]),
        CubeSide::Top => Normal([1., 0., 0.]),
        CubeSide::Bottom => Normal([-1., 0., 0.]),
    }
}
