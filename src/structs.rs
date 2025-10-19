enum TreeType {
    Decidious,
    Conifer
}

enum PlantationType {
    Park,
    Street
}

pub struct Tree{
    coordinates: [f64; 2],
    tree_type: TreeType,
    species_latin: &'static str,
    species_swedish: &'static str,
    plantation_type: PlantationType,
    date: &'static str
}