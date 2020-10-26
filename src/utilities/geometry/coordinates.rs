#[derive(Debug)]
pub struct Coordinates {
    x: u16,
    y: u16,
    z: Option<u16>
}

impl Coordinates{
    pub fn new(x:u16, y:u16, z:Option<u16>) -> Coordinates{
        Coordinates{x,y,z}
    }
    pub fn x(&self) -> &u16 {&self.x}
    pub fn y(&self) -> &u16 {&self.y}
    pub fn z(&self) -> &Option<u16> {&self.z}
}