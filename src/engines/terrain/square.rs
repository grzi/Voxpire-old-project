use crate::utilities::geometry::coordinates::Coordinates;
use crate::engines::terrain::bloc::Bloc;

#[derive(Debug)]
pub struct Square {
    in_chunk_coordinates: Coordinates,
    global_coordinates: Coordinates,
    blocs: Vec<Bloc>,
    is_fertile: bool,
    has_structure_on_top: bool
}

impl Square {
    pub fn new (in_chunk_coordinates: Coordinates,
                global_coordinates: Coordinates,
                is_fertile: bool,
                has_structure_on_top: bool) -> Square {
        Square{
            in_chunk_coordinates,
            global_coordinates,
            blocs: Vec::new(),
            is_fertile,
            has_structure_on_top
        }
    }

    pub fn blocs(&self) -> &Vec<Bloc> {
        &self.blocs
    }

    pub fn push_bloc(&mut self, bloc:Bloc){
        self.blocs.push(bloc);
    }
}