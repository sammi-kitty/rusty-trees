pub mod tree_organiser {
    use std::fs;
    use text_io::read;

    use crate::structs::{PlantationType, Species, Tree, TreeType, Coordinates};

    pub fn read_trees() -> Vec<Tree> {
        let filename: String = get_filename("Input file name (WITH relative path): ");

        let file = read_file(&filename);

        let trees: Vec<Tree> = format_trees(&file);

        return trees;
    }

    fn format_trees(file: &Option<String>) -> Vec<Tree> {
        let file = match file {
            None => return Vec::new(),
            Some(file) => file,
        };

        let mut trees: Vec<Tree> = Vec::new();

        let iter_file = file.split("\n");
        for line in iter_file {
            if line == "Geo Point;Löv- eller barrträd;Trädart vetenskapligt namn;Trädart svenskt namn;Gatu- eller parkträd;Planteringsdatum" {
                
            }
            else {
                trees.push(line_to_tree(line))
            }
        }

        return trees;
    }

    fn line_to_tree(line: &str) -> Tree {
        /*
        Take in a line on form
        "coordinates";"tree_type";"species_latin";"species_swedish";"plantation_type";"date"
        and return Tree object 
        */ 

        let vec_elements = line.split(";").collect::<Vec<&str>>();

        Tree {
            coordinates: {

            },
            tree_type: {
                if vec_elements[1].to_lowercase() == "lövträd" {
                    TreeType::Decidious
                } else {
                    TreeType::Conifer
                }
            },
            species: {
                Species {
                    latin: String::from(vec_elements[2]),
                    swedish: String::from(vec_elements[3]),
                }
            },
            plantation_type: {
                if vec_elements[4].to_lowercase() == "parkträd" {
                    PlantationType::Park
                } else {
                    PlantationType::Street
                }
            },
            date: vec_elements[5],
        }
    }

    fn read_file(filename: &str) -> Option<String> {
        let file: Option<String> = match fs::read_to_string(filename) {
            Err(why) => {
                println!("{why}");
                None
            }
            Ok(file) => Some(file),
        };
        return file;
    }

    fn get_filename(text: &str) -> String {
        let mut input: String = String::new();
        while !is_valid_filename(&input) {
            println!("{text}");
            input = read!("{}\n");
        }

        return input;
    }

    fn is_valid_filename(filename: &str) -> bool {
        println!("{}", filename);

        match fs::read_to_string(&filename) {
            Err(why) => {
                println!("{why}");
                return false;
            }
            Ok(_file) => {
                //println!("{_file}");
                return true;
            }
        };
    }
}
