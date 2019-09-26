use std::error::Error;
use std::fmt;
use std::thread;
use std::sync::{Mutex, Arc};

#[derive(Clone, PartialEq, Debug)]
enum TerrainGround {
    Soil,
    Stone
}

#[derive(Clone, PartialEq, Debug)]
enum TerrainBlock {
    Tree,
    Soil,
    Stone
}

#[derive(Clone, PartialEq, Debug)]
enum Being {
    Orc,
    Human
}

enum Direction {
    West,
    East,
    North,
    South
}

#[derive(Debug, PartialEq)]
enum MovementError {
    NoBeingInSquare,
    OutOfGridBounds,
    AnotherBeingInSquare,
    TerrainIsStone
}

#[derive(Clone)]
struct Square {
    ground: TerrainGround,
    block: Option<TerrainBlock>,
    being: Option<Being>
}

struct Grid {
    size: (usize, usize),
    squares: Vec<Square>
}

impl fmt::Display for MovementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Movement Error")
    }
}

impl Error for MovementError {
    fn description(&self) -> &str {
        match self {
            NoBeingInSquare => "No Being in Square",
            AnotherBeingInSquare => "Another Being in Square",
            OutOfGridBounds => "Fell of the Grid",
            TerrainIsStone => "Terrain is Stone"
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl Grid {
    fn move_being_in_coord(&mut self, coord: (usize, usize), dir: Direction) -> Result<(usize, usize), MovementError> {
        let squares = self.squares.clone();
        let square = squares.get(coord.0 * self.size.0 + coord.1).expect("Index of out map bounds");

        if square.being == None {
            return Err(MovementError::NoBeingInSquare);
        }

        let destination_coord = match dir {
            Direction::West => (coord.0, coord.1 - 1),
            Direction::East => (coord.0, coord.1 + 1),
            Direction::South => (coord.0 + 1, coord.1),
            Direction::North => (coord.0 - 1, coord.1)
        };

        if destination_coord.0 >= self.size.0 || destination_coord.1 >= self.size.1 {
            return Err(MovementError::OutOfGridBounds);
        }

        let destination_square = squares.get(destination_coord.0  * self.size.0 + destination_coord.1).unwrap();

        if destination_square.being != None {
            return Err(MovementError::AnotherBeingInSquare);
        }

        if destination_square.block == Some(TerrainBlock::Stone) {
            return Err(MovementError::TerrainIsStone);
        }

        self.squares[destination_coord.0  * self.size.0 + destination_coord.1] = Square {
            being: square.being.clone(),
            block: destination_square.block.clone(),
            ground: destination_square.ground.clone()
        };

        self.squares[coord.0 * self.size.0 + coord.1] = Square {
            being: None,
            block: square.block.clone(),
            ground: square.ground.clone()
        };

        return Ok(destination_coord);

    }

    fn generate_empty(size_x: usize, size_y: usize) -> Grid {
        let number_of_squares = size_x * size_y;
        let mut squares: Vec<Square> = Vec::with_capacity(number_of_squares);

        for _ in 0..number_of_squares {
            squares.push(Square{ground: TerrainGround::Stone, block: None, being: None});
        }

        Grid {
            size: (size_x, size_y),
            squares: squares
        }
    }
}

fn main() {
    println!("Creating the grid..");
    let mut initial_grid = Grid::generate_empty(3, 3);

    println!("..and the humans");
    let homer = Being::Human;
    let lisa = Being::Human;

    initial_grid.squares[0].being = Some(homer);
    initial_grid.squares[4].being = Some(lisa);

    let grid = Arc::new(Mutex::new(initial_grid));

    let grid_homer = grid.clone();
    let thread_homer = thread::spawn(move || {
        println!("Moving homer");
        let mut internal_grid = grid_homer.lock().unwrap();
        match internal_grid.move_being_in_coord((0, 0), Direction::East) {
            Ok(position) => println!("homer successfully moved to {}", position.0 + position.1),
            Err(reason) => println!("homer movement error {}", reason.description())
        }
    });

    let grid_lisa = grid.clone();
    let thread_lisa = thread::spawn(move || {
        println!("Moving lisa");
        let mut internal_grid = grid_lisa.lock().unwrap();
        match internal_grid.move_being_in_coord((1, 1), Direction::West) {
            Ok(position) => println!("lisa successfully moved to {}", position.0 + position.1),
            Err(reason) => println!("lisa movement error {}", reason.description())
        }
    });

    thread_homer.join().unwrap();
    thread_lisa.join().unwrap();

    println!("Everyone is done");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_empty_grid() {
        let grid = super::Grid::generate_empty(5, 13);
        assert_eq!(grid.size, (5, 13));
        let mut number_of_squares = 0;

        for square in &grid.squares {
            assert_eq!(square.ground, super::TerrainGround::Stone);
            assert_eq!(square.block, None);
            assert_eq!(square.being, None);
            number_of_squares += 1;
        }

        assert_eq!(grid.squares.len(), (5*13));
        assert_eq!(number_of_squares, 5*13);
    }

    #[test]
    fn test_move_without_being_in_square() {
        let mut grid = super::Grid::generate_empty(3, 3);
        assert_eq!(grid.move_being_in_coord((0, 1), super::Direction::West), Err(super::MovementError::NoBeingInSquare));
    }

    #[test]
    fn test_move_out_of_grid() {
        let mut grid = super::Grid::generate_empty(3, 3);
        let human = super::Being::Human;

        grid.squares[8].being = Some(human);
        assert_eq!(grid.move_being_in_coord((2, 2), super::Direction::East), Err(super::MovementError::OutOfGridBounds));
    }

    #[test]
    fn test_move_in_busy_square() {
        let mut grid = super::Grid::generate_empty(3, 3);
        let human = super::Being::Human;
        let orc = super::Being::Orc;

        grid.squares[0].being = Some(human);
        grid.squares[1].being = Some(orc);
        assert_eq!(grid.move_being_in_coord((0, 0), super::Direction::East), Err(super::MovementError::AnotherBeingInSquare));
    }

    #[test]
    fn test_move_in_terrain_stone() {
        let mut grid = super::Grid::generate_empty(2, 2);
        let human = super::Being::Human;

        grid.squares[0].being = Some(human);
        grid.squares[1].block = Some(super::TerrainBlock::Stone);
        assert_eq!(grid.move_being_in_coord((0, 0), super::Direction::East), Err(super::MovementError::TerrainIsStone));
    }

    #[test]
    fn test_move_success() {
        let mut grid = super::Grid::generate_empty(2, 2);
        let human = super::Being::Human;

        grid.squares[0].being = Some(human);
        assert_eq!(grid.move_being_in_coord((0, 0), super::Direction::East), Ok((0, 1)));
        assert_eq!(grid.squares[0].being, None);
        assert!(grid.squares[1].being != None);
    }
}