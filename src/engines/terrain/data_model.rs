use rand::Rng;

use crate::utilities::geometry::coordinates::Coordinates;
use crate::engines::terrain::chunk::{Chunk, CHUNK_SIZE};
use crate::engines::terrain::square::Square;
use crate::engines::terrain::bloc::Bloc;
use crate::engines::blocs::bloc_type::BlocType;

#[derive(Debug)]
pub struct IslandTerrain {
    seed: u32,
    chunks: Vec<Vec<Chunk>>
}

impl IslandTerrain {
    pub fn chunks(&self) -> &Vec<Vec<Chunk>> {
        &self.chunks
    }
}

pub fn generate_fake_island() -> IslandTerrain{
    let mut chunks = Vec::<Vec<Chunk>>::new();
    for chunk_width in 0..4 {
        let mut chunk_line = Vec::<Chunk>::new();
        for chunk_height in 0..4 {
            let mut chunk = Chunk::empty_with_coordinates( Coordinates::new(chunk_width, chunk_height, None));
            for width in 0..CHUNK_SIZE {
                let mut square_column = Vec::<Square>::new();
                for height in 0..CHUNK_SIZE {
                    let mut square = Square::new(
                        Coordinates::new(width, height, None),
                        Coordinates::new(CHUNK_SIZE * chunk_width + width, CHUNK_SIZE * chunk_height + height, None),
                        false,
                        false
                    );
                    for square_height in 0..(rand::thread_rng().gen_range(3, 12)){
                        square.push_bloc(
                            Bloc::new(Coordinates::new(CHUNK_SIZE * chunk_width + width, CHUNK_SIZE * chunk_height + height, Some(square_height)), BlocType::Dirt)
                        );
                    }
                    square_column.push(square);
                }
                chunk.add_square_column(square_column);
            }
            chunk_line.push(chunk);
        }
        chunks.push(chunk_line);
    }
    IslandTerrain{ seed: 0, chunks }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_fake_island() {
        let island = generate_fake_island();
        println!("coucou {:?}", island);
    }
}