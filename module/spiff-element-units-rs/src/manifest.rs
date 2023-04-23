use crate::domain::{ElementUnits, Manifest};

pub fn from_element_units(element_units: &ElementUnits) -> Manifest {
    Manifest {
        items: element_units.items.iter().map(|el| el.id()).collect(),
        index_map: element_units.index_map.clone(),
    }
}
