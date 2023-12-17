use std::{ fs, collections::HashSet };
use colored::Colorize;

struct Tile {
    code: char,
    coords: (usize, usize),
    map_height: usize,
    map_width: usize,
}

impl Tile {
    fn new(code: char, coords: (usize, usize), height: usize, width: usize) -> Tile {
        return Tile {
            code: code,
            coords: coords,
            map_height: height,
            map_width: width,
        };
    }

    fn get_next(&self, direction: char) -> Option<(usize, usize)> {
        let r = self.coords.0 as i32;
        let c = self.coords.1 as i32;
        let r2: i32;
        let c2: i32;

        match direction {
            'N' => {
                r2 = r - 1;
                c2 = c;
            }
            'S' => {
                r2 = r + 1;
                c2 = c;
            }
            'E' => {
                r2 = r;
                c2 = c + 1;
            }
            'W' => {
                r2 = r;
                c2 = c - 1;
            }
            _ => panic!("Direction not recognized: {}", direction),
        }

        if r2 >= 0 && c2 >= 0 && r2 < (self.map_height as i32) && c2 < (self.map_width as i32) {
            return Some((r2 as usize, c2 as usize));
        } else {
            return None;
        }
    }

    fn get_connected_coords(&self) -> Option<((usize, usize), (usize, usize))> {
        let cell1;
        let cell2;

        match self.code {
            '|' => {
                cell1 = self.get_next('N');
                cell2 = self.get_next('S');
            }
            '-' => {
                cell1 = self.get_next('E');
                cell2 = self.get_next('W');
            }
            'L' => {
                cell1 = self.get_next('N');
                cell2 = self.get_next('E');
            }
            'J' => {
                cell1 = self.get_next('N');
                cell2 = self.get_next('W');
            }
            '7' => {
                cell1 = self.get_next('S');
                cell2 = self.get_next('W');
            }
            'F' => {
                cell1 = self.get_next('S');
                cell2 = self.get_next('E');
            }
            '.' => {
                return None;
            }
            'S' => {
                return None;
            }
            _ => panic!("Unknown tile pattern: {}", &self.code),
        }

        if cell1.is_none() || cell2.is_none() {
            return None;
        }

        return Some((cell1.unwrap(), cell2.unwrap()));
    }

    // We will determine if the next cell is inside the loop.
    // For horizontal pipes we need to remember the shape of the tile before the horizontal pipe
    // For F--J, we will switch from outside to inside (or vice versa)
    // For F--7, we will keep the same state (i.e. outside stays outside, etc)
    fn is_east_inside(
        &self,
        current_inside: bool,
        current_horiz: Option<char>
    ) -> (bool, Option<char>) {
        match self.code {
            '|' => {
                return (!current_inside, None);
            }
            '-' => {
                return (current_inside, current_horiz);
            }
            'L' => {
                return (current_inside, Some('L'));
            }
            'F' => {
                return (current_inside, Some('F'));
            }
            'J' => {
                if let Some(c) = current_horiz {
                    if c == 'F' {
                        return (!current_inside, None);
                    } else if c == 'L' {
                        return (current_inside, None);
                    }
                }
                panic!("Weird situation with a J {},{}", self.coords.0, self.coords.1);
            }
            '7' => {
                if let Some(c) = current_horiz {
                    if c == 'F' {
                        return (current_inside, None);
                    } else if c == 'L' {
                        return (!current_inside, None);
                    }
                }
                panic!("Weird situation with a J {},{}", self.coords.0, self.coords.1);
            }
            '.' => {
                return (current_inside, None);
            }
            'S' => {
                return (!current_inside, None);
            }
            _ => panic!("Unknown tile pattern: {}", &self.code),
        }
    }
}

#[test]
fn test_tile_functions() {
    let mut t = Tile::new('F', (3, 3), 5, 5);
    assert_eq!(t.get_next('N'), Some((2, 3)));
    assert_eq!(t.get_next('S'), Some((4, 3)));
    assert_eq!(t.get_next('E'), Some((3, 4)));
    assert_eq!(t.get_next('W'), Some((3, 2)));
    // checking connections
    assert_eq!(t.get_connected_coords(), Some(((4, 3), (3, 4))));
    t.code = 'L';
    assert_eq!(t.get_connected_coords(), Some(((2, 3), (3, 4))));
    t.code = 'J';
    assert_eq!(t.get_connected_coords(), Some(((2, 3), (3, 2))));
    t.code = '7';
    assert_eq!(t.get_connected_coords(), Some(((4, 3), (3, 2))));
    t.code = '|';
    assert_eq!(t.get_connected_coords(), Some(((2, 3), (4, 3))));
}

pub fn solve(problem_input: &str) {
    let contents = fs::read_to_string(problem_input).expect("Should have been able to read the file");

    let line_array: Vec<&str> = contents.split("\n").collect();
    let mut tiles_matrix: Vec<Vec<Tile>> = vec![];

    let mut start_row = 0;
    let mut start_col = 0;
    let map_width: usize = line_array[0].chars().count();
    let map_height: usize = line_array.len();

    let mut row = 0;
    for line in line_array {
        let mut tile_row: Vec<Tile> = vec![];
        let mut col = 0;
        for c in line.chars() {
            if c == 'S' {
                start_row = row;
                start_col = col;
            }
            tile_row.push(Tile::new(c, (row, col), map_height, map_width));
            col = col + 1;
        }
        row = row + 1;
        tiles_matrix.push(tile_row);
    }

    // First, let's look at all around the start cell
    let mut starting_cells: Vec<(usize, usize)> = vec![];
    for i in 0..9 {
        let r: i32 = (start_row as i32) + i / 3 - 1;
        let c: i32 = (start_col as i32) + (i % 3) - 1;
        println!("Checking cell {},{}", r, c);
        if r >= 0 && r < (map_height as i32) && c >= 0 && c < (map_width as i32) {
            if let Some((a, b)) = tiles_matrix[r as usize][c as usize].get_connected_coords() {
                println!("-=-=-=-> {:?},{:?}", a, b);
                if a.0 == start_row && a.1 == start_col {
                    println!("Found S neighbour at ({},{})", r, c);
                    starting_cells.push((r as usize, c as usize));
                }
                if b.0 == start_row && b.1 == start_col {
                    println!("Found S neighbour at ({},{})", r, c);
                    starting_cells.push((r as usize, c as usize));
                }
            }
        }
    }
    // Starting the loop from a random cell and increasing distance little by little
    if starting_cells.len() == 0 {
        panic!("No starting cells are found!");
    }
    let mut current_cell = starting_cells.first().unwrap().clone();
    let mut visited_cells: HashSet<(usize, usize)> = HashSet::new();
    visited_cells.insert(current_cell);
    let mut first_cell = true;

    while current_cell != (start_row, start_col) {
        let (a, b) = tiles_matrix[current_cell.0][current_cell.1].get_connected_coords().unwrap();
        println!(
            "Checking ({},{}) - neighbours are ({},{}) and ({},{})",
            current_cell.0,
            current_cell.1,
            a.0,
            a.1,
            b.0,
            b.1
        );
        if first_cell {
            // special case of first cell: we test wether we come back to start or not
            if a.0 == start_row && a.1 == start_col {
                visited_cells.insert(current_cell);
                current_cell = b;
            } else {
                visited_cells.insert(current_cell);
                current_cell = a;
            }
            first_cell = false;
        } else {
            if visited_cells.contains(&a) {
                // then we visit b
                visited_cells.insert(current_cell);
                current_cell = b;
            } else {
                visited_cells.insert(current_cell);
                current_cell = a;
            }
        }
    }
    visited_cells.insert((start_row, start_col));

    let mut count_inside = 0;
    for r in 0..map_height {
        let mut is_inside = false;
        let mut is_horiz: Option<char> = None;
        for c in 0..map_width {
            if visited_cells.contains(&(r, c)) {
                print!("{}", tiles_matrix[r][c].code);
                (is_inside, is_horiz) = tiles_matrix[r][c].is_east_inside(is_inside, is_horiz);
            } else {
                if is_inside {
                    print!("{}", "I".yellow());
                    count_inside = count_inside + 1;
                } else {
                    print!(" ");
                }
            }
        }
        println!("");
    }
    println!("Number of cells inside: {}", count_inside);
}
