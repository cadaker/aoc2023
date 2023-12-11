use aoc2023::grid::{Grid, GridBuilder, Point};
use aoc2023::utils::stdio_lines;

fn parse_input() -> Grid<char> {
    let lines = stdio_lines();
    let mut builder = GridBuilder::new();

    for line in lines {
        for ch in line.chars() {
            builder.add(ch);
        }
        builder.eol();
    }

    builder.finish()
}

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

fn delta(dir: Dir) -> (i32, i32) {
    match dir {
        Dir::Up => (-1, 0),
        Dir::Right => (0, 1),
        Dir::Down => (1, 0),
        Dir::Left => (0, -1),
    }
}

fn step(p: &Point, d: Dir) -> Point {
    let (dr, dc) = delta(d);
    Point{row: p.row + dr, col: p.col + dc}
}

fn enterable_via(pipe: char, dir: Dir) -> bool {
    match dir {
        Dir::Up => pipe == '|' || pipe == 'F' || pipe == '7',
        Dir::Right => pipe == '-' || pipe == 'J' || pipe == '7',
        Dir::Down => pipe == '|' || pipe == 'J' || pipe == 'L',
        Dir::Left => pipe == '-' || pipe == 'F' || pipe == 'L',
    }
}

fn exit_of(pipe: char, in_dir: Dir) -> Dir {
    match (pipe, in_dir) {
        ('|', Dir::Up) => Dir::Up,
        ('|', Dir::Down) => Dir::Down,
        ('-', Dir::Left) => Dir::Left,
        ('-', Dir::Right) => Dir::Right,
        ('J', Dir::Down) => Dir::Left,
        ('J', Dir::Right) => Dir::Up,
        ('L', Dir::Left) => Dir::Up,
        ('L', Dir::Down) => Dir::Right,
        ('7', Dir::Right) => Dir::Down,
        ('7', Dir::Up) => Dir::Left,
        ('F', Dir::Left) => Dir::Down,
        ('F', Dir::Up) => Dir::Right,
        _ => panic!("Mismatch: {} {:?}", pipe, in_dir)
    }
}

fn find_start(grid: &Grid<char>) -> Option<Point> {
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            if *grid.get(row, col) == 'S' {
                return Some(Point{row, col});
            }
        }
    }
    None
}

fn fix_start_pos(grid: &mut Grid<char>) -> Point {
    let start = find_start(grid).unwrap();

    let matches = |dir: Dir| {
        let next = step(&start, dir);
        grid.containsp(&next) && enterable_via(*grid.getp(&next), dir)
    };

    let tiles = [
        (Dir::Up, Dir::Down, '|'),
        (Dir::Up, Dir::Right, 'L'),
        (Dir::Up, Dir::Left, 'J'),
        (Dir::Right, Dir::Left, '-'),
        (Dir::Down, Dir::Right, 'F'),
        (Dir::Down, Dir::Left, '7'),
    ];

    for (dir1, dir2, ch) in tiles {
        if matches(dir1) && matches(dir2) {
            *grid.mutgetp(&start) = ch;
            return start
        }
    }
    panic!("Could not find a way to replace starting tile");
}

fn trace_loop(grid: &Grid<char>, start: &Point) -> Vec<(Point, Dir)> {
    let mut dir = match *grid.getp(start) {
        '|' => Dir::Down,
        'F' => Dir::Down,
        '7' => Dir::Down,
        '-' => Dir::Right,
        'L' => Dir::Right,
        'J' => Dir::Left,
        _ => panic!("Start square bad"),
    };

    let mut path = Vec::new();
    path.push((*start, dir));
    let mut next = step(start, dir);

    while next != *start {
        let out_dir = exit_of(*grid.getp(&next), dir);
        path.push((next, out_dir));
        dir = out_dir;
        next = step(&next, dir);
    }

    path
}

fn inner_area(grid: &Grid<char>, path: &[(Point, Dir)]) -> i64 {
    // Green's theorem gives Area = sum_(x,y) in C x * dy
    // That includes area on each tile, though, so we have to correct for that.

    // This counts the total area, including all the space "on" the pipes
    let mut full_area = 0;
    for i in 0..path.len() {
        let dy = (path[(i+1) % path.len()].0.row - path[i].0.row) as i64;
        let x = path[i].0.col as i64;
        full_area += x * dy;
    }

    // This counts quarter-areas for the loop squares
    let mut area_compensation_quarters = 0i64;
    for (p, dir) in path {
          let diff = match (grid.getp(p), *dir) {
              ('|', _) => 2,
              ('-', _) => 2,
              ('F', Dir::Down) => 3,
              ('F', Dir::Right) => 1,
              ('L', Dir::Right) => 3,
              ('L', Dir::Up) => 1,
              ('J', Dir::Up) => 3,
              ('J', Dir::Left) => 1,
              ('7', Dir::Left) => 3,
              ('7', Dir::Down) => 1,
              _ => panic!("No matching tile in path")
          };
        area_compensation_quarters += diff
    }

    // If we counted a positive area, the compensation above counts the inside area properly.
    // If we counted a negative area, we've counted the outside of every pipe square, so correct
    //  with 1 unit per pipe square, i.e. the length of the path.
    if full_area >= 0 {
        full_area - area_compensation_quarters / 4
    } else {
        (-full_area) - (path.len() as i64 - area_compensation_quarters / 4)
    }
}

fn main() {
    let mut grid = parse_input();
    let start = fix_start_pos(&mut grid);

    let path = trace_loop(&grid, &start);

    println!("{}", path.len() / 2);
    println!("{}", inner_area(&grid, &path));
}
