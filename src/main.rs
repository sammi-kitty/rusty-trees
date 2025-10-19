mod structs;
mod tree_organiser;

fn main() {
    let value = tree_organiser::tree_organiser::read_trees();
    println!("{:?}", value)
}