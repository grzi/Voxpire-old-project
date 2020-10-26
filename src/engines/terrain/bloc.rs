use crate::utilities::geometry::coordinates::Coordinates;
use crate::engines::blocs::bloc_type::BlocType;

pub const BLOC_HEIGHT : f32 = 1.0;
pub const BLOC_WIDTH : f32 = 1.0;

#[derive(Debug)]
pub struct Bloc {
    coordinates: Coordinates,
    bloc_type: BlocType,
}

impl Bloc {
    pub fn new(coordinates : Coordinates, bloc_type: BlocType) -> Bloc {
        Bloc{ coordinates, bloc_type }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}