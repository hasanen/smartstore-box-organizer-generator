mod rack;
use std::vec;

use rack::{Container, Dimensions};

pub use crate::rack::generate_svg;

pub fn supported_containers() -> Vec<Container> {
    vec![Container {
        vendor: "SmartStore".to_string(),
        model: "Classic 1".to_string(),
        description: "Rack generated with this container can also have classic 2 and classic 3 containers. And 'Home' versions of same containers.".to_string(),
        links: vec![
          "https://www.orthexgroup.com/smartstore-classic/909-smartstore-classic-1-7310543520075.html".to_string(),
          "https://www.orthexgroup.com/smartstore-classic/918-smartstore-classic-2-7310543521072.html".to_string(),
          "https://www.orthexgroup.com/smartstore-classic/919-smartstore-classic-3-7310543522079.html".to_string(),
          ],
          dimensions: Dimensions {
            width: 170,
            depth: 210,
            height: 56,
            side_wing_from_box_top: 15,
            side_wing_width: 8,
          }
    }]
}
