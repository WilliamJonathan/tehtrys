use cgmath::Vector2;
// ( VIDEO 1 - 32:16 MIN )
pub(super) struct Piece {
    pub kind: Kind,
    pub position: Vector2<usize>,
    pub rotation: Rotation,
}

impl Piece {
    const CELL_COUNT: usize = 4;
}
#[derive(Clone, Copy, Debug, PartialEq)]
// AQUI SÃO OS TIPOS DE BLOCOS DO JOGOS
pub enum Kind { O, I, T, L, J, S, Z }

impl Kind {
    pub const  ALL: [Self; 7] = [Self::O, Self::I, Self::T, Self::L, Self::J, Self::S, Self::Z];
    // VIDEO 1 ( 1:06:33 )
    pub fn cells(&self) -> [Vector2<usize>;Piece::CELL_COUNT] {
        match self {
            Kind::O => todo!(),
            Kind::I => todo!(),
            Kind::T => todo!(),
            Kind::L => todo!(),
            Kind::J => todo!(),
            Kind::S => todo!(),
            Kind::Z => todo!(),
        }
    }
}

pub enum Rotation { N, S, E, W }
