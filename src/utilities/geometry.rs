use amethyst::renderer::rendy::mesh::{MeshBuilder, Normal, Position, TexCoord};
use amethyst::renderer::types::MeshData;
use amethyst::Error;

pub enum CubeSide {
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
    FRONT,
    BACK,
}
enum CubeVertices {
    FRONT_TOP_LEFT,
    FRONT_TOP_RIGHT,
    FRONT_BOTTOM_LEFT,
    FRONT_BOTTOM_RIGHT,
    BACK_TOP_LEFT,
    BACK_TOP_RIGHT,
    BACK_BOTTOM_LEFT,
    BACK_BOTTOM_RIGHT,
}
pub fn create_cube() -> Result<MeshData, Error> {
    create_multiple_quads(vec![
        CubeSide::LEFT,
        CubeSide::TOP,
        CubeSide::RIGHT,
        CubeSide::BOTTOM,
        CubeSide::FRONT,
        CubeSide::BACK,
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

pub fn create_quad_mesh(side: CubeSide) -> Result<MeshData, Error> {
    let tris = create_quad_tris(side);
    Ok(MeshBuilder::new()
        .with_vertices(tris.0)
        .with_vertices(tris.1)
        .with_vertices(tris.2)
        .into())
}

fn create_quad_tris(cube_side: CubeSide) -> (Vec<Position>, Vec<Normal>, Vec<TexCoord>) {
    match cube_side {
        CubeSide::FRONT => (
            vec![
                vertice_of(CubeVertices::FRONT_TOP_LEFT),
                vertice_of(CubeVertices::FRONT_BOTTOM_LEFT),
                vertice_of(CubeVertices::FRONT_TOP_RIGHT),
                vertice_of(CubeVertices::FRONT_TOP_RIGHT),
                vertice_of(CubeVertices::FRONT_BOTTOM_LEFT),
                vertice_of(CubeVertices::FRONT_BOTTOM_RIGHT),
            ],
            vec![normal_of(CubeSide::FRONT); 6],
            vec![TexCoord([0., 0.]); 6],
        ),
        CubeSide::TOP => (
            vec![
                vertice_of(CubeVertices::FRONT_TOP_LEFT),
                vertice_of(CubeVertices::BACK_TOP_RIGHT),
                vertice_of(CubeVertices::BACK_TOP_LEFT),
                vertice_of(CubeVertices::BACK_TOP_RIGHT),
                vertice_of(CubeVertices::FRONT_TOP_LEFT),
                vertice_of(CubeVertices::FRONT_TOP_RIGHT),
            ],
            vec![normal_of(CubeSide::TOP); 6],
            vec![TexCoord([0., 0.]); 6],
        ),
        CubeSide::RIGHT => (
            vec![
                vertice_of(CubeVertices::FRONT_TOP_RIGHT),
                vertice_of(CubeVertices::BACK_TOP_RIGHT),
                vertice_of(CubeVertices::FRONT_BOTTOM_RIGHT),
                vertice_of(CubeVertices::FRONT_BOTTOM_RIGHT),
                vertice_of(CubeVertices::BACK_TOP_RIGHT),
                vertice_of(CubeVertices::BACK_BOTTOM_RIGHT),
            ],
            vec![normal_of(CubeSide::RIGHT); 6],
            vec![TexCoord([0., 0.]); 6],
        ),
        CubeSide::BOTTOM => (
            vec![
                vertice_of(CubeVertices::FRONT_BOTTOM_LEFT),
                vertice_of(CubeVertices::BACK_BOTTOM_LEFT),
                vertice_of(CubeVertices::FRONT_BOTTOM_RIGHT),
                vertice_of(CubeVertices::FRONT_BOTTOM_RIGHT),
                vertice_of(CubeVertices::BACK_BOTTOM_LEFT),
                vertice_of(CubeVertices::BACK_BOTTOM_RIGHT),
            ],
            vec![normal_of(CubeSide::BOTTOM); 6],
            vec![TexCoord([0., 0.]); 6],
        ),
        CubeSide::LEFT => (
            vec![
                vertice_of(CubeVertices::FRONT_TOP_LEFT),
                vertice_of(CubeVertices::BACK_TOP_LEFT),
                vertice_of(CubeVertices::FRONT_BOTTOM_LEFT),
                vertice_of(CubeVertices::FRONT_BOTTOM_LEFT),
                vertice_of(CubeVertices::BACK_TOP_LEFT),
                vertice_of(CubeVertices::BACK_BOTTOM_LEFT),
            ],
            vec![normal_of(CubeSide::LEFT); 6],
            vec![TexCoord([0., 0.]); 6],
        ),
        CubeSide::BACK => (
            vec![
                vertice_of(CubeVertices::BACK_TOP_LEFT),
                vertice_of(CubeVertices::BACK_BOTTOM_LEFT),
                vertice_of(CubeVertices::BACK_TOP_RIGHT),
                vertice_of(CubeVertices::BACK_TOP_RIGHT),
                vertice_of(CubeVertices::BACK_BOTTOM_LEFT),
                vertice_of(CubeVertices::BACK_BOTTOM_RIGHT),
            ],
            vec![normal_of(CubeSide::BACK); 6],
            vec![TexCoord([0., 0.]); 6],
        ),
        _ => (vec![], vec![], vec![]),
    }
}

fn vertice_of(cube_vertice: CubeVertices) -> Position {
    match cube_vertice {
        CubeVertices::FRONT_TOP_LEFT => return Position([0., 0., 1.]),
        CubeVertices::FRONT_TOP_RIGHT => return Position([1., 0., 1.]),
        CubeVertices::FRONT_BOTTOM_RIGHT => return Position([1., 0., 0.]),
        CubeVertices::FRONT_BOTTOM_LEFT => return Position([0., 0., 0.]),
        CubeVertices::BACK_TOP_LEFT => return Position([0., 1., 1.]),
        CubeVertices::BACK_TOP_RIGHT => return Position([1., 1., 1.]),
        CubeVertices::BACK_BOTTOM_RIGHT => return Position([1., 1., 0.]),
        CubeVertices::BACK_BOTTOM_LEFT => return Position([0., 1., 0.]),
    };
}

fn normal_of(cube_side: CubeSide) -> Normal {
    match cube_side {
        CubeSide::BACK => Normal([0., 0., 1.]),
        CubeSide::FRONT => Normal([0., 0., -1.]),
        CubeSide::LEFT => Normal([0., -1., 0.]),
        CubeSide::RIGHT => Normal([0., 1., 0.]),
        CubeSide::TOP => Normal([1., 0., 0.]),
        CubeSide::BOTTOM => Normal([-1., 0., 0.]),
    }
}
