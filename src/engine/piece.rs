use super::{Coordinate, Offset};

pub(super) struct Piece {
    pub kind: Kind,
    pub position: Coordinate,
    pub rotation: Rotation,
}

impl Piece {
    const CELL_COUNT: usize = 4;

    pub fn cells(&self) -> Option<impl Iterator<Item=Coordinate>> {
        self.kind.cells()
            .map(|cell|cell * self.rotation)
    }

    fn rotator(&self) -> impl Fn(Coordinate) -> Coordinate {
        // VIDEO 1 ( 1:41:45 )
        |
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind { O, I, T, L, J, S, Z }

impl Kind {
    pub const  ALL: [Self; 7] = [Self::O, Self::I, Self::T, Self::L, Self::J, Self::S, Self::Z];
    pub fn cells(&self) -> impl Iterator<Item=&'static Vector2<isize>> {
        match self {
            Kind::O => &[( 0,0), ( 0,1), (1,0), (1,1)],
            Kind::I => &[(-1,0), ( 0,0), (1,0), (2,0)],
            Kind::T => &[(-1,0), ( 0,0), (1,0), (0,1)],
            Kind::L => &[(-1,0), ( 0,0), (1,0), (1,1)],
            Kind::J => &[(-1,1), (-1,0), (0,0), (1,0)],
            Kind::S => &[(-1,0), ( 0,0), (0,1), (1,1)],
            Kind::Z => &[(-1,1), ( 0,1), (0,0), (1,0)],
        }.iter().map(From::from)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rotation { N, S, E, W }

impl std::ops::Mul<Rotation> for Offset {
    type Output = Self;
    fn mul(self, rotation: Rotation) -> Self::Output {
        match rotation {
            Rotation::N => self,
            Rotation::S => Offset::new(-self.x, -self.y),
            Rotation::E => Offset::new(self.x, -self.y),
            Rotation::W => Offset::new(-self.y, self.x),
        }
    }
}
