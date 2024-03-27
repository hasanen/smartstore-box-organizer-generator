mod generator;
use std::vec;

use generator::Container;

pub use crate::generator::generate_svg;

pub fn supported_containers() -> Vec<Container> {
    vec![Container {
        vendor: "SmartStore".to_string(),
        model: "Classic 1".to_string(),
        description: "Rack generated with this model also supports classic 2 and classic 3 containers. And 'Home' versions of same containers.".to_string(),
        width_with_lid: 174,
        depth: 210,
        block_height: 55,
        side_wing_from_box_top: 14,
        side_wing_width: 8,
    }]
}
