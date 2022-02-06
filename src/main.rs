use configparser::ini::Ini;
use serde::{Deserialize, Serialize};
use std::{fs, thread, time};

#[derive(Debug, Serialize, Deserialize)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Spaceship {
    name: String,
    coords: Vec<Coord>,
    starting_coord: Coord,
}

struct World {
    matrice: Vec<Vec<bool>>,
    spaceships: Vec<Spaceship>,
}

impl World {
    fn new(size: usize) -> World {
        World {
            matrice: vec![vec![false; size]; size],
            spaceships: vec![],
        }
    }

    fn get_cel_value(self: &Self, column: usize, line: usize) -> bool {
        self.matrice[line][column]
    }

    fn set_cel_value(self: &mut Self, column: usize, line: usize) {
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

    fn add_spaceship(self: &mut Self, spaceship: Spaceship) {
        self.draw_spaceship(&spaceship);
        self.spaceships.push(spaceship);
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
            if column != self.matrice.len() - 1  && self.get_cel_value(column + 1, line - 1) {
                neighbours_counter += 1;
            }
        }
        if column != 0 && self.get_cel_value(column - 1, line) {
            neighbours_counter += 1;
        }
        if column != self.matrice.len() - 1 && self.get_cel_value(column + 1, line) {
            neighbours_counter += 1;
        }
        if line != self.matrice.len() - 1 {
            if column != 0 && self.get_cel_value(column - 1, line + 1) {
                neighbours_counter += 1;
            }
            if self.get_cel_value(column, line + 1) {
                neighbours_counter += 1;
            }
            if column != self.matrice.len() - 1 && self.get_cel_value(column + 1, line + 1) {
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

    fn goto_next_gen(self: &mut Self) {
        let mut new_matrice: World = World::new(self.matrice.len());

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

    fn draw_spaceship(self: &mut Self, spaceship: &Spaceship) -> () {
        for coord in spaceship.coords.iter() {
            self.set_cel_value(
                spaceship.starting_coord.x + coord.x,
                spaceship.starting_coord.y + coord.y,
            );
        }
    }
}

fn get_file_content(file_name: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_name)
}

fn main() {
    let mut configparser = Ini::new();
    let config = match configparser.load("config.ini") {
        Ok(config) => config,
        Err(error) => panic!("{}", error),
    };

    let size = config["global"]["matrice_size"]
        .clone()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let sleep_time: u64 = config["global"]["sleep_time"].clone().unwrap().parse().unwrap();

    let mut world: World = World::new(size);

    let mut generation: u32 = 1;

    for spaceship_config in config["spaceships"].iter() {
        match spaceship_config.1.clone().unwrap().as_str() {
            "On" => {
                let spaceship_json =
                    &get_file_content(&format!("{}{}", &spaceship_config.0[..], ".json"))
                        .expect("erreur lor de l'accès au fichier.")[..];
                let spaceship: Spaceship = serde_json::from_str(spaceship_json)
                    .expect("Erreur lor de la désérialisation.");
                world.add_spaceship(spaceship);
            }
            _ => (),
        }
    }

    print!("{}", world.display());

    loop {
        world.goto_next_gen();
        print!(
            "\x1B[2J\x1B[1;1H\nGénération {}:\n{}",
            generation,
            world.display()
        );
        thread::sleep(time::Duration::from_millis(sleep_time));
        generation += 1;
    }
}
