pub const SMALL_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

#[derive(Debug)]
enum Symbol {
    Void,
    Splitter,
    Beam(usize),
}

#[derive(Debug)]
struct Teleporter {
    manifold_diagram: Vec<Vec<Symbol>>,
    beam_pos: usize,
    split_counter: usize,
}

impl Teleporter {
    fn import(input: &str) -> Teleporter {
        let lines = input.lines();
        let mut diagram = Vec::<Vec<Symbol>>::new();

        for line in lines {
            let mut diagram_line: Vec<Symbol> = Vec::new();

            for char in line.chars() {
                match char {
                    '.' => diagram_line.push(Symbol::Void),
                    'S' => diagram_line.push(Symbol::Beam(1)),
                    '^' => diagram_line.push(Symbol::Splitter),
                    _ => {}
                }
            }

            diagram.push(diagram_line);
        }

        Teleporter { manifold_diagram: diagram, beam_pos: 0, split_counter: 0 }
    }

    fn print_diagram(&self) {
        for row in &self.manifold_diagram {
            for symbol in row {
                match symbol {
                    Symbol::Void => print!("."),
                    Symbol::Beam(_) => print!("|"),
                    Symbol::Splitter => print!("^"),
                }
            }
            println!();
        }
    }

    fn get_beams_at_line(&self, line_no: usize) -> Vec<(usize, usize)> {
        let mut beam_positions: Vec<(usize, usize)> = Vec::new();

        for (pos, symbol) in self.manifold_diagram.get(line_no).unwrap().iter().enumerate() {
            match symbol {
                Symbol::Void => {}
                Symbol::Splitter => {}
                Symbol::Beam(count) => {
                    beam_positions.push((pos, *count));
                }
            }
        }

        beam_positions
    }

    fn next_step(&mut self) {
        let beam_positions = self.get_beams_at_line(self.beam_pos);
        //println!("beam_positions: {:?}", beam_positions);

        for (beam_position, beam_count) in beam_positions {
            match self.manifold_diagram.get(self.beam_pos+1).unwrap().get(beam_position) {
                None => {println!("problem")}
                Some(Symbol::Void) => {
                    self.manifold_diagram[self.beam_pos+1][beam_position] = Symbol::Beam(beam_count);
                }
                Some(Symbol::Splitter) => {
                    match self.manifold_diagram[self.beam_pos+1][beam_position-1] {
                        Symbol::Void => {self.manifold_diagram[self.beam_pos+1][beam_position-1] = Symbol::Beam(beam_count);}
                        Symbol::Splitter => {}
                        Symbol::Beam(count) => {self.manifold_diagram[self.beam_pos+1][beam_position-1] = Symbol::Beam(beam_count+count);}
                    }

                    match self.manifold_diagram[self.beam_pos+1][beam_position+1] {
                        Symbol::Void => {self.manifold_diagram[self.beam_pos+1][beam_position+1] = Symbol::Beam(beam_count);}
                        Symbol::Splitter => {}
                        Symbol::Beam(count) => {self.manifold_diagram[self.beam_pos+1][beam_position+1] = Symbol::Beam(beam_count+count);}
                    }

                    self.split_counter += 1;
                }
                Some(Symbol::Beam(count)) => {
                    self.manifold_diagram[self.beam_pos+1][beam_position] = Symbol::Beam(beam_count+count);
                }
            }
        }

        self.beam_pos += 1;


    }

    fn solve(&mut self) {
        while self.beam_pos < self.manifold_diagram.len() - 1 {
            self.next_step();
        }
    }

    fn part2_result(&self) -> usize {
        let mut result = 0;
        if let Some(row) = self.manifold_diagram.last() {
            for symbol in row {
                match symbol {
                    Symbol::Void => {}
                    Symbol::Splitter => {}
                    Symbol::Beam(count) => {result+=count}
                }
            }
        }
        result
    }
}

pub fn part1(input: &str) -> String {
    let mut teleporter = Teleporter::import(input);
    //teleporter.print_diagram();

    teleporter.solve();

    format!("grand total: {}", teleporter.split_counter)
}

pub fn part2(input: &str) -> String {
    let mut teleporter = Teleporter::import(input);
    teleporter.solve();

    format!("different timelines: {}", teleporter.part2_result())
}