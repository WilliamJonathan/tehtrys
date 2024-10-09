use cgmath::Vector2;
// ( VIDEO 1 - 32:16 MIN )
pub(super) struct Piece {
    pub kind: Kind,
    pub position: Vector2<usize>,
    pub rotation: Rotation,
}

impl Piece {
    const CELL_COUNT: usize = 4;

    pub fn cells(&self) -> impl Option<Iterator<Item=Vector2<usize>>> {
        todo!()
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
// AQUI SÃO OS TIPOS DE BLOCOS DO JOGOS
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

pub enum Rotation { N, S, E, W }
// video 1 1:26:20
impl<S> std::ops::Mul<Rotation> for Vector2<S> {
    fn matrix() {
        
    }
}
