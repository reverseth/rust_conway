use serde::{Deserialize, Serialize};
use std::{thread, time};

const COLUMNS_SIZE: usize = 100;
const LINES_SIZE: usize = 75;
const SLEEP_TIME: u64 = 80;

#[derive(Debug, Serialize, Deserialize)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Spaceship {
    name: String,
    points: Vec<Point>,
    starting_point: Point,
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
            self.set_cel_value(spaceship.starting_point.x + point.x, spaceship.starting_point.y + point.y);
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

    print!("{}", world.display());

    loop {
        world.goto_next_gen();

        print!("\x1B[2J\x1B[1;1H");
        println!("\nGeneration {} :", generation);
        print!("{}", world.display());

        generation += 1;
        thread::sleep(time::Duration::from_millis(SLEEP_TIME));
    }
}

fn spaceship_serializator() -> Result<String, serde_json::Error> {
    let glider: Spaceship = Spaceship {
        name: String::from("Glider"),
        points: vec![
            Point { x: 1, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
        ],
        starting_point: Point { x: 18, y: 12 },
    };

    let pulsar: Spaceship = Spaceship {
        name: String::from("Pulsar"),
        points: vec![
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 8, y: 0 },
            Point { x: 9, y: 0 },
            Point { x: 10, y: 0 },
            Point { x: 0, y: 2 },
            Point { x: 5, y: 2 },
            Point { x: 7, y: 2 },
            Point { x: 12, y: 2 },
            Point { x: 0, y: 3 },
            Point { x: 5, y: 3 },
            Point { x: 7, y: 3 },
            Point { x: 12, y: 3 },
            Point { x: 0, y: 4 },
            Point { x: 5, y: 4 },
            Point { x: 7, y: 4 },
            Point { x: 12, y: 4 },
            Point { x: 2, y: 5 },
            Point { x: 3, y: 5 },
            Point { x: 4, y: 5 },
            Point { x: 8, y: 5 },
            Point { x: 9, y: 5 },
            Point { x: 10, y: 5 },
            Point { x: 2, y: 7 },
            Point { x: 3, y: 7 },
            Point { x: 4, y: 7 },
            Point { x: 8, y: 7 },
            Point { x: 9, y: 7 },
            Point { x: 10, y: 7 },
            Point { x: 0, y: 8 },
            Point { x: 5, y: 8 },
            Point { x: 7, y: 8 },
            Point { x: 12, y: 8 },
            Point { x: 0, y: 9 },
            Point { x: 5, y: 9 },
            Point { x: 7, y: 9 },
            Point { x: 12, y: 9 },
            Point { x: 0, y: 10 },
            Point { x: 5, y: 10 },
            Point { x: 7, y: 10 },
            Point { x: 12, y: 10 },
            Point { x: 2, y: 12 },
            Point { x: 3, y: 12 },
            Point { x: 4, y: 12 },
            Point { x: 8, y: 12 },
            Point { x: 9, y: 12 },
            Point { x: 10, y: 12 },
        ],
        starting_point: Point { x: 38, y: 35 },
    };

    let pentadecathlon: Spaceship = Spaceship {
        name: String::from("Pentadecathlon"),
        points: vec![
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 1 },
            Point { x: 4, y: 1 },
            Point { x: 5, y: 1 },
            Point { x: 6, y: 1 },
            Point { x: 7, y: 0 },
            Point { x: 7, y: 2 },
            Point { x: 8, y: 1 },
            Point { x: 9, y: 1 },
        ],
        starting_point: Point { x: 31, y: 60 },
    };

    let spaceships_json: Result<String, serde_json::Error> =
        serde_json::to_string(&vec![glider, pulsar, pentadecathlon]);
    spaceships_json
}
