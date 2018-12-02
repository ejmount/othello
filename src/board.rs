use std::fmt;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Colour {
    Light,
    Dark,
}
impl Colour {
    pub fn opposite(&self) -> Colour {
        match self {
            Colour::Light => Colour::Dark,
            Colour::Dark => Colour::Light,
        }
    }
}

type GridSlot = Option<Colour>;

pub const BOARD_SIZE: usize = 8;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Direction(i8, i8);
const DIRECTIONS: [Direction; 8] = [
    Direction(0, -1),
    Direction(-1, 0),
    Direction(1, 0),
    Direction(0, 1),
    Direction(1, -1),
    Direction(-1, -1),
    Direction(-1, 1),
    Direction(1, 1),
];
impl Mul<i8> for Direction {
    type Output = Direction;
    fn mul(self, f: i8) -> Direction {
        Direction(self.0 * f, self.1 * f)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position(usize, usize);
impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        assert!(x < BOARD_SIZE && y < BOARD_SIZE);
        Position(x, y)
    }
    pub fn get_x(&self) -> usize {
        self.0
    }
    pub fn get_y(&self) -> usize {
        self.1
    }
}
impl Add<Direction> for Position {
    type Output = Position;
    fn add(self, o: Direction) -> Position {
        let x = self.0 as isize + o.0 as isize;
        let y = self.1 as isize + o.1 as isize;
            Position(x as usize,y as usize)
    }
}
lazy_static! {
    static ref POSITIONS: Vec<Position> = (0..BOARD_SIZE)
        .flat_map(|r| (0..BOARD_SIZE).map(move |c| Position(r, c)))
        .collect();
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Move {
    pub origin: Position,
    pub dir: Direction,
    pub team: Colour,
    _private: (), // Stops public construction
}

impl Move {
    fn new(origin: Position, dir: Direction, team: Colour) -> Move {
        Move {
            origin,
            dir,
            team,
            _private: (),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Board {
    grid: [[GridSlot; BOARD_SIZE]; BOARD_SIZE],
}
impl Board {
    pub fn new() -> Board {
        let row = [None; BOARD_SIZE];
        let grid = [row; BOARD_SIZE];
        return Board { grid };
    }

    pub fn get(&self, Position(x, y): Position) -> GridSlot {
        return self.grid[x][y];
    }

    pub fn get_mut(&mut self, Position(x, y): Position) -> &mut GridSlot {
        return &mut self.grid[x][y];
    }

    pub fn set(&mut self, Position(x, y): Position, v: GridSlot) {
        self.grid[x][y] = v;
    }

    const BOARD_SIZE_I32 : i32 = BOARD_SIZE as i32;
    const FAR_AWAY : i32 = Self::BOARD_SIZE_I32*2;

    pub fn get_line(origin: Position, going: Direction) -> impl Iterator<Item = Position> {
        let x_boundary = match going.0 {
            x if x > 0 => Self::BOARD_SIZE_I32,
            x if x < 0 => -1,
            _ => Self::FAR_AWAY
        } as i32;
        let y_boundary = match going.1 {
            y if y > 0 => Self::BOARD_SIZE_I32,
            y if y < 0 => -1,
            _ => Self::FAR_AWAY
        };
        let dist_x = (origin.0 as i32 - x_boundary).abs();
        let dist_y = (origin.1 as i32 - y_boundary).abs();
        let min_steps = std::cmp::min(dist_x, dist_y) as i8;
        (1..min_steps).map(move |s| (origin + (going * s)))
    }

    pub fn get_empty_at_end_of_line(&self, origin: Position, going: Direction) -> Option<Position> {
        let starting_team = self.get(origin)?;
        let mut capturing = false;
        for p in Self::get_line(origin, going) {
            match self.get(p) {
                Some(u) if starting_team == u => return None,
                Some(u) if starting_team != u => {
                    capturing = true;
                }
                None if capturing => return Some(p),
                None => return None,
                _ => {}
            }
        }
        return None;
    }

    pub fn get_moves_from_cell<'a>(
        &'a self,
        t: Colour,
        origin: Position,
    ) -> impl 'a + Iterator<Item = Move> {
        let get_move_in_direction =
            move |d: &Direction| match self.get_empty_at_end_of_line(origin, *d) {
                Some(_) => Some(Move::new(origin, *d, t)),
                None => None,
            };
        DIRECTIONS.iter().flat_map(get_move_in_direction)
    }

    pub fn get_moves<'a>(&'a self, t: Colour) -> impl 'a + Iterator<Item = Move> {
        POSITIONS
            .iter()
            .filter(move |&p| Some(t) == self.get(*p))
            .flat_map(move |&p| self.get_moves_from_cell(t, p))
    }

    pub fn apply_move(&mut self, m: Move) {
        for p in Self::get_line(m.origin, m.dir) {
            let cell = self.get_mut(p);
            match *cell {
                Some(u) if u != m.team => {
                    *cell = Some(m.team);
                }
                None => {
                    *cell = Some(m.team);
                    break;
                }
                _ => {
                    panic!("Invalid move applied");
                }
            }
        }
    }

    pub fn get_score(&self) -> (usize, usize) {
        let mut light = 0;
        let mut dark = 0;
        let all_cells = self.grid.iter().flat_map(|x| x.iter());
        for c in all_cells {
            match &c {
                Some(Colour::Light) => light += 1,
                Some(Colour::Dark) => dark += 1,
                None => {}
            }
        }
        return (dark, light);
    }
}
impl Default for Board {
    fn default() -> Board {
        use self::Colour::*;
        let mut b = Board::new();
        let centre = (BOARD_SIZE - 1) / 2;

        b.set(Position(centre, centre), Some(Light));
        b.set(Position(centre + 1, centre), Some(Dark));
        b.set(Position(centre, centre + 1), Some(Dark));
        b.set(Position(centre + 1, centre + 1), Some(Light));
        return b;
    }
}
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                match self.get(Position(r, c)) {
                    Some(Colour::Light) => write!(f, "L")?,
                    Some(Colour::Dark) => write!(f, "D")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
