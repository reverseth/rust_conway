use serde::{Deserialize, Serialize};
use std::{thread, time};

const COLUMNS_SIZE: usize = 100;
const LINES_SIZE: usize = 70;

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Spaceship {
    name: String,
    points: Vec<Point>,
}

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

    fn build_something(self: &mut Self, spaceship: &Spaceship) -> () {
        for point in spaceship.points.iter() {
            self.set_cel_value(point.x, point.y);
        }
    }
}

fn main() {
    let mut world: World = World::new();

    let mut generation: u32 = 1;

    let spaceships_json = spaceship_serializator().unwrap();
    let spaceships: Vec<Spaceship> = serde_json::from_str(&spaceships_json[..]).unwrap();

    for spaceship in spaceships.iter() {
        world.build_something(spaceship);
    }

    loop {
        world.goto_next_gen();

        print!("\x1B[2J\x1B[1;1H");
        println!("\nGeneration {} :", generation);
        print!("{}", world.display());

        generation += 1;
        thread::sleep(time::Duration::from_millis(50));
    }
}

fn spaceship_serializator() -> Result<String, serde_json::Error> {
    let glider: Spaceship = Spaceship {
        name: String::from("Glider"),
        points: vec![
            Point { x: 25, y: 10 },
            Point { x: 26, y: 11 },
            Point { x: 24, y: 12 },
            Point { x: 25, y: 12 },
            Point { x: 26, y: 12 },
        ],
    };

    let pulsar: Spaceship = Spaceship {
        name: String::from("Pulsar"),
        points: vec![
            Point { x: 38, y: 22 },
            Point { x: 39, y: 22 },
            Point { x: 40, y: 22 },
            Point { x: 44, y: 22 },
            Point { x: 45, y: 22 },
            Point { x: 46, y: 22 },
            Point { x: 36, y: 24 },
            Point { x: 41, y: 24 },
            Point { x: 43, y: 24 },
            Point { x: 48, y: 24 },
            Point { x: 36, y: 25 },
            Point { x: 41, y: 25 },
            Point { x: 43, y: 25 },
            Point { x: 48, y: 25 },
            Point { x: 36, y: 26 },
            Point { x: 41, y: 26 },
            Point { x: 43, y: 26 },
            Point { x: 48, y: 26 },
            Point { x: 38, y: 27 },
            Point { x: 39, y: 27 },
            Point { x: 40, y: 27 },
            Point { x: 44, y: 27 },
            Point { x: 45, y: 27 },
            Point { x: 46, y: 27 },
            Point { x: 38, y: 29 },
            Point { x: 39, y: 29 },
            Point { x: 40, y: 29 },
            Point { x: 44, y: 29 },
            Point { x: 45, y: 29 },
            Point { x: 46, y: 29 },
            Point { x: 36, y: 30 },
            Point { x: 41, y: 30 },
            Point { x: 43, y: 30 },
            Point { x: 48, y: 30 },
            Point { x: 36, y: 31 },
            Point { x: 41, y: 31 },
            Point { x: 43, y: 31 },
            Point { x: 48, y: 31 },
            Point { x: 36, y: 32 },
            Point { x: 41, y: 32 },
            Point { x: 43, y: 32 },
            Point { x: 48, y: 32 },
            Point { x: 38, y: 34 },
            Point { x: 39, y: 34 },
            Point { x: 40, y: 34 },
            Point { x: 44, y: 34 },
            Point { x: 45, y: 34 },
            Point { x: 46, y: 34 },
        ],
    };

    let pentadecathlon: Spaceship = Spaceship {
        name: String::from("Pentadecathlon"),
        points: vec![
            Point { x: 21, y: 42 },
            Point { x: 22, y: 42 },
            Point { x: 23, y: 41 },
            Point { x: 23, y: 43 },
            Point { x: 24, y: 42 },
            Point { x: 25, y: 42 },
            Point { x: 26, y: 42 },
            Point { x: 27, y: 42 },
            Point { x: 28, y: 41 },
            Point { x: 28, y: 43 },
            Point { x: 29, y: 42 },
            Point { x: 30, y: 42 },
        ],
    };

    let spaceships_json: Result<String, serde_json::Error> =
        serde_json::to_string(&vec![glider, pulsar, pentadecathlon]);
    spaceships_json
}
