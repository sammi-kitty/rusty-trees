pub mod tree_organiser {
    use text_io::read;
    use std::{fs, path::Path};

    use crate::structs::Tree;

    pub fn read_trees() -> Option<bool> {
        let filename: String = get_filename(
            "Input file name (WITH relative path): "
        );
        return None
    }

    fn read_file(filename: &str) -> Option<bool> {
        
        return None
    }

    fn get_filename(text: &str) -> String {
        let mut input: String = String::new();
        while !is_valid_filename(&input) {
            println!("{text}");
            input = read!("{}\n");
        }
        
        return input
    }

    fn is_valid_filename(filename: &str) -> bool {
        let filepath = Path::new(filename);
        println!("{}", filepath.display());

        let mut _file = match fs::File::open(&filepath){
            Err(why) => {
                println!("{why}");
                return false
            },
            Ok(_file) => return true,
        };
    }
}