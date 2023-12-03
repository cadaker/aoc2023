pub struct Point {
    pub row: i32,
    pub col: i32,
}

pub struct Grid<T> {
    width: i32,
    data: Vec<T>,
}

pub struct GridBuilder<T> {
    width: i32,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn width(&self) -> i32 { return self.width; }
    pub fn height(&self) -> i32 { return (self.data.len() as i32) / self.width(); }
    pub fn get(&self, row: i32, col: i32) -> &T { return &self.data[(row * self.width() + col) as usize]; }
    pub fn getp(&self, p: &Point) -> &T { return self.get(p.row, p.col); }
    pub fn contains(&self, row: i32, col: i32) -> bool { return row >= 0 && row < self.height() && col >= 0 && col < self.width(); }
    pub fn containsp(&self, p: &Point) -> bool { return self.contains(p.row, p.col); }
}

impl<T> GridBuilder<T> {
    pub fn new() -> GridBuilder<T> {
        return GridBuilder{ width: -1, data: Vec::new()};
    }

    pub fn add(&mut self, val: T) {
        self.data.push(val);
    }

    pub fn eol(&mut self) {
        if self.width < 0 {
            self.width = self.data.len() as i32;
        } else if self.width != 0 && (self.data.len() as i32) % self.width != 0 {
            panic!("Mismatched line lengths in grid");
        }
    }

    pub fn finish(self) -> Grid<T> {
        return Grid {width: self.width, data: self.data};
    }
}