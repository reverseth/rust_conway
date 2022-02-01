use std::{thread, time};

const COLUMNS_SIZE: usize = 50;
const LINES_SIZE: usize = 50;

struct World {
    matrice: Vec<Vec<bool>>,
}

impl World {
    fn new() -> World {
        World {
            matrice: vec![vec![false; COLUMNS_SIZE]; LINES_SIZE],
        }
    }

    fn get_cel_value(self: &Self, column: usize, line: usize) -> bool {
        self.matrice[line][column]
    }

    fn set_cel_value(self: &mut Self, column: usize, line: usize) -> () {
        self.matrice[line][column] = !self.get_cel_value(column, line);
    }

    fn display(self: &Self) -> String {
        let mut map = String::from("");
        for vec_lines in self.matrice.iter() {
            map.push_str("\n");
            for cel_val in vec_lines.iter() {
                map.push_str(match cel_val {
                    false => "⬜",
                    true => "⬛",
                });
            }
        }
        map.push_str("\n");
        map
    }

    fn get_amount_of_neighbours(self: &Self, column: usize, line: usize) -> u8 {
        let mut neighbours_counter = 0;
        if line != 0 {
            if column != 0 && self.get_cel_value(column - 1, line - 1) {
                neighbours_counter += 1;
            }
            if self.get_cel_value(column, line - 1) {
                neighbours_counter += 1;
            }
            if column != COLUMNS_SIZE - 1 && self.get_cel_value(column + 1, line - 1) {
                neighbours_counter += 1;
            }
        }
        if column != 0 && self.get_cel_value(column - 1, line) {
            neighbours_counter += 1;
        }
        if column != COLUMNS_SIZE - 1 && self.get_cel_value(column + 1, line) {
            neighbours_counter += 1;
        }
        if line != LINES_SIZE - 1 {
            if column != 0 && self.get_cel_value(column - 1, line + 1) {
                neighbours_counter += 1;
            }
            if self.get_cel_value(column, line + 1) {
                neighbours_counter += 1;
            }
            if column != COLUMNS_SIZE - 1 && self.get_cel_value(column + 1, line + 1) {
                neighbours_counter += 1;
            }
        }
        neighbours_counter
    }

    fn should_switch(self: &Self, column: usize, line: usize) -> bool {
        match self.get_cel_value(column, line) {
            true => match self.get_amount_of_neighbours(column, line) {
                2 | 3 => false,
                _ => true,
            },
            false => match self.get_amount_of_neighbours(column, line) {
                3 => true,
                _ => false,
            },
        }
    }

    fn goto_next_gen(self: &mut Self) -> () {
        let mut new_matrice: World = World::new();

        for (id_line, vec_lines) in self.matrice.iter().enumerate() {
            for (id_column, _) in vec_lines.iter().enumerate() {
                new_matrice.matrice[id_line][id_column] =
                    match self.should_switch(id_column, id_line) {
                        true => !self.get_cel_value(id_column, id_line),
                        false => self.get_cel_value(id_column, id_line),
                    };
            }
        }
        self.matrice = new_matrice.matrice;
    }
}

fn main() {
    let mut world: World = World::new();

    // println!("{:#?}", w.matrice);

    {
        // glider
        world.set_cel_value(25, 13);
        world.set_cel_value(26, 14);
        world.set_cel_value(24, 15);
        world.set_cel_value(25, 15);
        world.set_cel_value(26, 15);
    }

    {
        // pulsar
        world.set_cel_value(28, 22);
        world.set_cel_value(29, 22);
        world.set_cel_value(30, 22);
        world.set_cel_value(34, 22);
        world.set_cel_value(35, 22);
        world.set_cel_value(36, 22);
        world.set_cel_value(26, 24);
        world.set_cel_value(31, 24);
        world.set_cel_value(33, 24);
        world.set_cel_value(38, 24);
        world.set_cel_value(26, 25);
        world.set_cel_value(31, 25);
        world.set_cel_value(33, 25);
        world.set_cel_value(38, 25);
        world.set_cel_value(26, 26);
        world.set_cel_value(31, 26);
        world.set_cel_value(33, 26);
        world.set_cel_value(38, 26);
        world.set_cel_value(28, 27);
        world.set_cel_value(29, 27);
        world.set_cel_value(30, 27);
        world.set_cel_value(34, 27);
        world.set_cel_value(35, 27);
        world.set_cel_value(36, 27);
        world.set_cel_value(28, 29);
        world.set_cel_value(29, 29);
        world.set_cel_value(30, 29);
        world.set_cel_value(34, 29);
        world.set_cel_value(35, 29);
        world.set_cel_value(36, 29);
        world.set_cel_value(26, 30);
        world.set_cel_value(31, 30);
        world.set_cel_value(33, 30);
        world.set_cel_value(38, 30);
        world.set_cel_value(26, 31);
        world.set_cel_value(31, 31);
        world.set_cel_value(33, 31);
        world.set_cel_value(38, 31);
        world.set_cel_value(26, 32);
        world.set_cel_value(31, 32);
        world.set_cel_value(33, 32);
        world.set_cel_value(38, 32);
        world.set_cel_value(28, 34);
        world.set_cel_value(29, 34);
        world.set_cel_value(30, 34);
        world.set_cel_value(34, 34);
        world.set_cel_value(35, 34);
        world.set_cel_value(36, 34);
    }

    let mut generation: u32 = 1;
    loop {
        world.goto_next_gen();

        print!("\x1B[2J\x1B[1;1H");
        println!("\nGeneration {} :", generation);
        print!("{}", world.display());

        generation += 1;
        thread::sleep(time::Duration::from_millis(200));
    }
}
