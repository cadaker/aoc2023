use crate::grid::Point;

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

pub fn cw(dir: Dir) -> Dir {
    use Dir::*;
    match dir {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up,
    }
}

pub fn ccw(dir: Dir) -> Dir {
    use Dir::*;
    match dir {
        Up => Left,
        Right => Up,
        Down => Right,
        Left => Down,
    }
}

pub fn step(p: Point, dir: Dir) -> Point {
    use Dir::*;
    match dir {
        Up => Point{ row: p.row - 1, col: p.col },
        Right => Point{ row: p.row, col: p.col + 1 },
        Down => Point{ row: p.row + 1, col: p.col },
        Left => Point{ row: p.row, col: p.col - 1 },
    }
}

pub fn cart_neighbours(p: &Point) -> Vec<Point> {
    vec![
        Point{ row: p.row - 1, col: p.col },
        Point{ row: p.row + 1, col: p.col },
        Point{ row: p.row, col: p.col - 1 },
        Point{ row: p.row, col: p.col + 1 },
    ]
}
