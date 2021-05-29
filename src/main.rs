use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    collections::HashMap,
    env,
};
use rand::Rng;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn read_wordlist (filepath: String) -> HashMap<String, String> {
    let lines = lines_from_file(filepath).expect("Couldn't load Wordlist file");

    let mut map = HashMap::new();
    for line in lines {


        let iterator: Vec<&str> = line.split_whitespace().collect();
        let number = iterator[0].to_string();
        let word = iterator[1].to_string();
        map.insert(String::from(number).to_string(), String::from(word).to_string());
    }

    return map;
}

fn random_rolldice() -> u32 {
    return rand::thread_rng().gen_range(1..7)
}

fn generate_dicenumbers() -> String {
    let mut index = 0;
    let mut overallroll = String::new();
    while index<5 {
        let newroll = random_rolldice();
        overallroll.push_str(&newroll.to_string());
        index += 1;
    }
    overallroll
}

fn generate_password(n : u32, map: HashMap<String,String>) -> Vec<String> {
    let mut vec = Vec::new();
    for _i in 1..n+1 {
        let diceware = generate_dicenumbers();
        let resultword = map.get(&diceware).unwrap().to_string();
        vec.push(resultword);
    }
    vec
}

fn main() {
    let mut directory = env::current_exe().unwrap();
    directory.pop();
    env::set_current_dir(directory).expect("Couldn't set working directory!");


    let args: Vec<String> = env::args().collect();
    let number : u32;
    let filepath ;

    match args.len() {

        1 => {
            number = 6;
            filepath = "./assets/eff_large_wordlist.txt".to_string();
        },

        2 => {
            number = match &args[1].parse() {
                Ok(num) => {
                    *num
                },
                Err(_) => {
                    6
                }
            };
            filepath = "./assets/eff_large_wordlist.txt".to_string();
        },

        3 => {
            number = match &args[1].parse() {
                Ok(num) => {
                    *num
                },
                Err(_) => {
                    6                }
            };
            filepath = args[2].clone();
        },

        _ => {
            number =6;
            filepath =  "./assets/eff_large_wordlist.txt".to_string();
        }
    }


    println!("Generating {} words, from file: {}", number, filepath);
    let map = read_wordlist(filepath);
    println!("Diceware Password:\n{}", generate_password(number, map).join("-"));
}
