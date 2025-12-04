pub const SMALL_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";


struct Diagram {
    diagram: Vec<Vec<bool>>,
    dimensions: (usize, usize),
}

impl Diagram {
    fn new(input: &str) -> Self {
        let mut diagram: Vec<Vec<bool>> = Vec::new();

        let mut diagram_iter = input.lines();
        while let Some(line) = diagram_iter.next() {
            if !line.is_empty() {
                let mut row: Vec<bool> = Vec::new();
                for c in line.chars() {
                    if c == '.' {
                        row.push(false);
                    } else if c == '@' {
                        row.push(true);
                    }
                }
                diagram.push(row);
            }
        }

        let dimensions = (diagram.len(), diagram[0].len());

        Self {
            diagram,
            dimensions,
        }
    }

    fn is_roll(&self, x: usize, y: usize) -> bool {
        self.diagram[x][y]
    }

    fn is_roll_accessible(&self, x: usize, y: usize) -> bool {
        if !self.is_roll(x, y) {
            return false;
        }

        // find number of adjacent rolls (adjacent cells that are rolls)
        // if number < 4: true

        let mut adjacent_rolls = 0;

        for i in 0..=2 {
            for j in 0..=2 {
                // the roll we're testing
                if i == 1 && j == 1 {
                    continue;
                }

                // out of bounds
                if x + i < 1 || y + j < 1 || x + i > self.dimensions.0 || y + j > self.dimensions.1
                {
                    continue;
                }

                if self.is_roll(x + i - 1, y + j - 1) {
                    adjacent_rolls += 1;
                }
            }
        }

        adjacent_rolls < 4
    }

    fn count_accessible_rolls(&self) -> i32 {
        let mut counter = 0;
        for i in 0..self.dimensions.0 {
            for j in 0..self.dimensions.1 {
                if !self.is_roll(i, j) {
                    continue;
                } else if self.is_roll_accessible(i, j) {
                    counter += 1;
                }
            }
        }
        counter
    }

    fn remove_roll(&mut self, x: usize, y: usize) {
        if self.is_roll(x,y) {
            self.diagram[x][y] = false;
        }
    }

    fn list_accessible_rolls(&self) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = Vec::new();

        for i in 0..self.dimensions.0 {
            for j in 0..self.dimensions.1 {
                if !self.is_roll(i, j) {
                    continue;
                } else if self.is_roll_accessible(i, j) {
                    positions.push((i, j))
                }
            }
        }

        positions
    }

    fn print_diagram(&self) {
        for i in 0..self.dimensions.0 {
            for j in 0..self.dimensions.1 {
                if !self.is_roll(i, j) {
                    print!(".");
                } else {
                    if self.is_roll_accessible(i, j) {
                        print!("X");
                    } else {
                        print!("@");
                    }
                }
            }
            println!();
        }
    }
}

pub fn part1(input: &str) -> String {
    let diag = Diagram::new(input);

    //diag.print_diagram();

    format!("accessible rolls: {}", diag.count_accessible_rolls())
}

pub fn part2(input: &str) -> String {
    let mut diag = Diagram::new(input);

    let mut counter = 0;

    loop {
        let accessible_rolls = diag.list_accessible_rolls();

        if accessible_rolls.is_empty() {
            break;
        }

        counter += accessible_rolls.len();

        /*println!("\naccessible rolls: {}", diag.count_accessible_rolls());
        diag.print_diagram();*/

        for (x, y) in accessible_rolls {
            diag.remove_roll(x, y);
        }
    }

    format!("removable rolls: {}", counter)
}
