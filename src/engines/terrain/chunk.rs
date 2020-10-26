use crate::utilities::geometry::coordinates::Coordinates;
use crate::engines::terrain::square::Square;

pub const CHUNK_SIZE: u16 = 5;

#[derive(Debug)]
pub struct Chunk {
    squares: Vec<Vec<Square>>,
    coordinates: Coordinates
}

impl Chunk {
    pub fn empty_with_coordinates(coordinates: Coordinates) -> Chunk {
        Chunk{
            squares: Vec::new(),
            coordinates
        }
    }

    pub fn squares(&self) -> &Vec<Vec<Square>> {
        &self.squares
    }

    pub fn add_square_column(&mut self, column: Vec<Square>) {
        self.squares.push(column);
    }
}