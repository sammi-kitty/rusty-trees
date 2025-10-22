#[derive(Debug)]
pub enum TreeType {
    Decidious,
    Conifer
}

#[derive(Debug)]
pub enum PlantationType {
    Park,
    Street
}

#[derive(PartialEq, Debug)]
pub struct Species {
    pub latin: String,
    pub swedish: String
}

#[derive(Debug)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64
}

#[derive(Debug)]
pub struct Tree {
    pub coordinates: Coordinates,
    pub tree_type: TreeType,
    pub species: Species,
    pub plantation_type: PlantationType,
    pub date: &'static str
}

impl Tree {
    fn is_species(&self, species: Species) -> bool {
        if self.species == species {
            return true
        }
        else {
            return false
        }
    }
}