use std::{
    fmt,
    fs::File,
    io::{BufReader, Read},
};

const POKEMON_LEN: usize = 802;

/*
weight double, height double, captureRate int, isLegendary boolean, captureDate Date.
 */
#[derive(Debug, Clone)]
struct Date {
    day: i32,
    month: i32,
    year: i32,
}
impl Date {
    fn from_str(str: &str) -> Date {
        let date: Vec<&str> = str.split("/").collect();
        Date {
            day: date[0].parse::<i32>().unwrap(),
            month: date[1].parse::<i32>().unwrap(),
            year: date[2].parse::<i32>().unwrap(),
        }
    }
}
#[derive(Debug, Clone)]
struct Pokemon {
    id: i32,
    generation: i32,
    name: String,
    description: String,
    types: Vec<String>,
    abilities: Vec<String>,
    weight: f64,
    height: f64,
    capture_rate: i32,
    is_legendary: bool,
    capture_date: Date,
}

impl Pokemon {
    fn extract_habilities(&mut self, csv_line: &str) -> String {
        let idx = csv_line.find('[').unwrap();
        let final_idx = csv_line.find(']').unwrap();
        let mut new_line = csv_line[..idx].to_string();
        new_line.push_str(&csv_line[final_idx + 1..]);

        let habilitiy_str = csv_line[idx + 1..final_idx - 1].replace("'", "");
        let habilitiy_str = habilitiy_str.split(",");
        for hab in habilitiy_str {
            if hab == "" {
                continue;
            }
            self.abilities.push(hab.trim().to_string());
        }

        new_line
    }

    pub fn new() -> Pokemon {
        Pokemon {
            id: 0,
            generation: 0,
            name: "".to_string(),
            description: "".to_string(),
            types: Vec::new(),
            abilities: Vec::new(),
            weight: 0.0,
            height: 0.0,
            capture_rate: 0,
            is_legendary: false,
            capture_date: Date {
                day: 0,
                month: 0,
                year: 0,
            },
        }
    }

    pub fn from_str(line: &str) -> Pokemon {
        let mut pokemon = Pokemon::new();
        let cleared_line = pokemon.extract_habilities(line);

        let atributes: Vec<&str> = cleared_line.split(",").collect();
        pokemon.id = atributes[0].parse().unwrap();
        pokemon.generation = atributes[1].parse().unwrap();
        pokemon.name = atributes[2].to_string();
        pokemon.description = atributes[3].to_string();
        pokemon.types.push(atributes[4].to_string());
        if atributes[5].len() > 1 {
            pokemon.types.push(atributes[5].to_string());
        }
        if atributes[7].len() > 1 {
            pokemon.weight = atributes[7].parse().unwrap();
        } else {
            pokemon.weight = 0.0;
        }
        if atributes[8].len() > 1 {
            pokemon.height = atributes[8].parse().unwrap();
        } else {
            pokemon.height = 0.0;
        }
        if atributes[9].len() > 1 {
            pokemon.capture_rate = atributes[9].parse().unwrap();
        } else {
            pokemon.capture_rate = 0;
        }
        pokemon.is_legendary = atributes[10] == "1";
        if atributes[11].len() > 1 {
            pokemon.capture_date = Date::from_str(atributes[11]);
        }

        pokemon
    }

    pub fn vec_from_file(file: File) -> Vec<Pokemon> {
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader
            .read_to_string(&mut content)
            .expect("Failed to read file");

        let mut pokemons: Vec<Pokemon> = Vec::with_capacity(POKEMON_LEN);
        let mut lines = content.split("\n");
        lines.next();

        for csv_line in lines {
            pokemons.push(Pokemon::from_str(csv_line));
        }

        pokemons
    }
}

impl fmt::Display for Pokemon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format types and abilities to match the style of the example in Java
        let types_str = self
            .types
            .iter()
            .map(|t| format!("'{}'", t))
            .collect::<Vec<_>>()
            .join(", ");
        let abilities_str = self
            .abilities
            .iter()
            .map(|a| format!("'{}'", a))
            .collect::<Vec<_>>()
            .join(", ");

        // Add leading zero to day and month if necessary
        let day = format!("{:02}", self.capture_date.day);
        let month = format!("{:02}", self.capture_date.month);

        write!(
            f,
            "[#{} -> {}: {} - [{}] - [{}] - {:.1}kg - {:.1}m - {}% - {} - {} gen] - {}/{}/{}",
            self.id,
            self.name,
            self.description,
            types_str,
            abilities_str,
            self.weight,
            self.height,
            self.capture_rate,
            self.is_legendary,
            self.generation,
            day,
            month,
            self.capture_date.year
        )
    }
}

// impl Drop for Pokemon{
//     fn drop(&mut self) {
//         println!("Dropping Pokemon: {}", self.name);
//     }
// }

fn main() {
    let file: File = File::open("pokemon.csv").expect("Arquivo não encontrado!");

    let mut pokemons = Pokemon::vec_from_file(file);
    let mut entrada = String::new();

    let mut use_pokemons: Vec<Pokemon> = Vec::new();
    text_io::scan!("{}", entrada);
    //coletando primeira entrada
    while entrada.trim() != "FIM" {
        let id = entrada.parse::<usize>();
        use_pokemons.push(pokemons[id.unwrap() - 1].clone());
        text_io::scan!("{}", entrada);
    }
    // reutilizando variavel
    let mut pokemons = use_pokemons;

    sort(&mut pokemons); //ordenando

    for pokemon in pokemons {
        println!("{}", pokemon);
    }
}


fn sort(array: &mut Vec<Pokemon>) {

    heapify(array);

    let mut last = array.len();
    while last > 1 {
        last -= 1;
        array.swap(0, last);

        check_position_down(array, 0, last);
    }
}

// Função para ajustar a posição para cima (Heapify Up)
fn check_position_up(array: &mut Vec<Pokemon>, idx: usize) {
    if idx == 0 {
        return;
    }
    let mut current_idx = idx;
    while current_idx > 0 {
        let father_idx = (current_idx - 1) / 2;

        if array[father_idx].height < array[current_idx].height {
            array.swap(father_idx, current_idx);

            current_idx = father_idx;
        } else {
            break;
        }
    }
}

// Função para ajustar a posição para baixo (Heapify Down)
fn check_position_down(array: &mut Vec<Pokemon>, idx: usize, last_idx: usize) {
    let mut current_idx = idx;
    loop {
        let son1_idx = 2 * current_idx + 1;
        let son2_idx = 2 * current_idx + 2;
        let mut max_idx = current_idx;

        if son1_idx < last_idx {
            if array[son1_idx].height > array[max_idx].height {
                max_idx = son1_idx;
            }
        }
        if son2_idx < last_idx {
            if array[son2_idx].height > array[max_idx].height {
                max_idx = son2_idx;
            }
        }

        if max_idx != current_idx {
            array.swap(current_idx, max_idx);
            current_idx = max_idx;
        } else {
            break;
        }
    }
}

// Função para construir o Heap máximo
fn heapify(array: &mut Vec<Pokemon>) {
    for i in 0..array.len() {
        check_position_up(array, i);
    }
}
