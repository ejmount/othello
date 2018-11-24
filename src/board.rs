use arrayvec::ArrayVec;
use std::fmt;
use std::ops::Add;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Colour {
    Light,
    Dark,
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position(usize, usize);
impl Position {
    pub fn new(x: usize, y: usize) -> Option<Position> {
        if x < BOARD_SIZE && y < BOARD_SIZE {
            Some(Position(x, y))
        } else {
            None
        }
    }
    pub fn get_x(&self) -> usize {
        self.0
    }
    pub fn get_y(&self) -> usize {
        self.1
    }
}
impl Add<Direction> for Position {
    type Output = Option<Position>;
    fn add(self, o: Direction) -> Option<Position> {
        let x = self.0 as isize + o.0 as isize;
        let y = self.1 as isize + o.1 as isize;
        if 0 <= x && x < BOARD_SIZE as isize && 0 <= y && y < BOARD_SIZE as isize {
            Position::new(x as usize, y as usize)
        } else {
            None
        }
    }
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

#[derive(PartialEq, Eq)]
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

    pub fn get_line(&self, origin: Position, going: Direction) -> ArrayVec<[Position; BOARD_SIZE]> {
        let mut current_position = origin;
        let mut result = ArrayVec::new();
        loop {
            let next_position = current_position + going;
            if let Some(p) = next_position {
                result.push(p);
                current_position = p;
            } else {
                break;
            }
        }
        return result;
    }

    pub fn get_empty_at_end_of_line(&self, origin: Position, going: Direction) -> Option<Position> {
        let starting_team = self.get(origin)?;
        let mut capturing = false;
        for p in self.get_line(origin, going) {
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
        let mut positions = ArrayVec::<[_; BOARD_SIZE * BOARD_SIZE]>::new();
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                positions.push(Position(r, c));
            }
        }
        positions
            .into_iter()
            .filter(move |p| Some(t) == self.get(*p))
            .flat_map(move |p| self.get_moves_from_cell(t, p))
    }

    pub fn apply_move(&mut self, m: Move) {
        for p in self.get_line(m.origin, m.dir) {
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
