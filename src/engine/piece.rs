use cgmath::Vector2;
// ( VIDEO 1 - 32:16 MIN )
pub(super) struct Piece {
    pub kind: Kind,
    pub position: Vector2<usize>,
    pub rotation: Rotation,
}

impl Piece {
    
}

// AQUI SÃO OS TIPOS DE BLOCOS DO JOGOS
pub enum Kind { Square, Line, T, L, J, S, Z }

impl Kind {    
}

pub enum Rotation { N, S, E, W }
