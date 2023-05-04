use crate::cache::manifest::{Manifest, ManifestEntry};
use crate::element_units::ElementUnits;

pub fn from_element_units(element_units: &ElementUnits) -> Manifest {
    Manifest {
        items: element_units
            .items
            .iter()
            .map(ManifestEntry::from_element_unit)
            .collect(),
        index_map: element_units.index_map.clone(),
    }
}
