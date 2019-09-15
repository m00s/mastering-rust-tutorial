use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum TerrainGround {
    Soil,
    Stone
}

#[derive(PartialEq, Debug)]
enum TerrainBlock {
    Tree,
    Soil,
    Stone
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

#[derive(PartialEq, Debug)]
enum Being {
    Orc,
    Human
}

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
    fn move_being_in_coord(&self, coord: (usize, usize), dir: Direction) -> Result<(usize, usize), MovementError> {
        let square = self.squares.get(coord.0 * self.size.0 + coord.1).expect("Index of out map bounds");

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

        let destination_square = self.squares.get(destination_coord.0  * self.size.0 + destination_coord.1).unwrap();

        if destination_square.being != None {
            return Err(MovementError::AnotherBeingInSquare);
        }

        if destination_square.block == Some(TerrainBlock::Stone) {
            return Err(MovementError::TerrainIsStone);
        }

        return Ok((destination_coord.0, destination_coord.1));

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
        let grid = super::Grid::generate_empty(3, 3);
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
    }
}