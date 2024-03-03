use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::{Document, Node};

// All measurements are in mm
const BOX_WIDTH_UNDER_LID: usize = 155;
const BOX_WDITH_WITH_LID: usize = 174;
const BOX_DEPTH: usize = 210;
const BOX_BLOCK_HEIGHT: usize = 55;
const SIDE_WING_FROM_BOX_TOP: usize = 14;
const SIDE_WING_WIDTH: usize = 8;
const EXTRA_SPACE_ON_TOP_OF_TOP_SLOT: usize = 3;
const SIDE_WING_SLOT_FROM_FRONT: usize = 20;
const SIDE_WING_SLOT_WIDTH: usize = 20;
const SIDE_WING_SLOT_SPACING: usize = 15;
const CLEARANCE_BETWEEN_PATHS: usize = 3;
const SIDE_TAP_FROM_FRONT: usize = 30;
const SIDE_TAP_WIDTH: usize = 30;

pub fn generate_svg(
    rows: usize,
    columns: usize,
    material_thickness: f32,
    primary_color: &str,
    secondary_color: &str,
) -> Document {
    let starting_point_x = 0.0;
    let starting_point_y = 0.0;
    let amount_of_boxes = (rows * columns) as usize;
    let height_of_two_side_wings = height_of_two_side_wings(material_thickness);
    let height_of_two_side_wings_with_clearance =
        height_of_two_side_wings + CLEARANCE_BETWEEN_PATHS as f32;

    let total_width = (BOX_DEPTH + (CLEARANCE_BETWEEN_PATHS * 3)) as f32
        + top_width(columns, material_thickness)
        + (BOX_BLOCK_HEIGHT * rows + EXTRA_SPACE_ON_TOP_OF_TOP_SLOT) as f32
        + (2.0 * material_thickness);
    let total_height = vec![
        amount_of_boxes as f32 * height_of_two_side_wings_with_clearance,
        (2 * BOX_DEPTH + CLEARANCE_BETWEEN_PATHS) as f32,
        ((columns + 1) * (BOX_DEPTH + CLEARANCE_BETWEEN_PATHS)) as f32,
    ]
    .iter()
    .cloned()
    .fold(f32::NEG_INFINITY, f32::max);

    let mut document = Document::new()
        .set("viewBox", (0, 0, total_width, total_height))
        .set("width", format!("{}mm", total_width))
        .set("height", format!("{}mm", total_height));

    // Generate side wings
    for i in 0..amount_of_boxes {
        generate_side_wing_pair(
            &mut document,
            starting_point_x,
            starting_point_y + height_of_two_side_wings_with_clearance * i as f32,
            material_thickness,
            secondary_color,
        );
    }

    // Generate top and bottom pieces
    generate_top_and_bottom_pieces(
        &mut document,
        (BOX_DEPTH + CLEARANCE_BETWEEN_PATHS) as f32,
        columns,
        material_thickness,
        primary_color,
        secondary_color,
    );

    // generate side panels
    generate_side_panels(
        &mut document,
        (BOX_DEPTH + CLEARANCE_BETWEEN_PATHS) as f32 //side wings
            + top_width(columns, material_thickness) + CLEARANCE_BETWEEN_PATHS as f32, // top and bottom plates
        rows,
        columns,
        material_thickness,
        primary_color,
        secondary_color,
    );

    document
}

fn generate_side_panels(
    document: &mut Document,
    starting_point_x: f32,
    rows: usize,
    columns: usize,
    material_thickness: f32,
    primary_color: &str,
    secondary_color: &str,
) {
    for i in 0..columns + 1 {
        let y = (i * (BOX_DEPTH + CLEARANCE_BETWEEN_PATHS)) as f32;

        document.append(generate_side_panel_outline_path(
            starting_point_x,
            y,
            rows,
            material_thickness,
            secondary_color,
        ));

        for r in 0..rows {
            let row_x = material_thickness
                + (EXTRA_SPACE_ON_TOP_OF_TOP_SLOT + SIDE_WING_FROM_BOX_TOP + r * BOX_BLOCK_HEIGHT)
                    as f32;

            document.append(generate_side_panel_wing_holes(
                starting_point_x + row_x,
                y + SIDE_WING_SLOT_FROM_FRONT as f32,
                material_thickness,
                primary_color,
            ));

            document.append(generate_side_panel_wing_holes(
                starting_point_x + row_x,
                y + (SIDE_WING_SLOT_FROM_FRONT + SIDE_WING_SLOT_WIDTH + SIDE_WING_SLOT_SPACING)
                    as f32,
                material_thickness,
                primary_color,
            ));

            document.append(generate_side_panel_wing_holes(
                starting_point_x + row_x,
                y + (BOX_DEPTH
                    - SIDE_WING_SLOT_FROM_FRONT
                    - (2 * SIDE_WING_SLOT_WIDTH)
                    - SIDE_WING_SLOT_SPACING) as f32,
                material_thickness,
                primary_color,
            ));
            document.append(generate_side_panel_wing_holes(
                starting_point_x + row_x,
                y + (BOX_DEPTH - SIDE_WING_SLOT_FROM_FRONT - SIDE_WING_SLOT_WIDTH) as f32,
                material_thickness,
                primary_color,
            ));
        }
    }
}

fn generate_side_panel_wing_holes(x: f32, y: f32, material_thickness: f32, color: &str) -> Path {
    let path_data = Data::new()
        .move_to((x, y))
        .vertical_line_to(y + SIDE_WING_SLOT_WIDTH as f32)
        .horizontal_line_to(x + material_thickness)
        .vertical_line_to(y)
        .close();

    Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("d", path_data)
}

fn generate_side_panel_outline_path(
    starting_point_x: f32,
    starting_point_y: f32,
    rows: usize,
    material_thickness: f32,
    color: &str,
) -> Path {
    let panel_inner_height = (EXTRA_SPACE_ON_TOP_OF_TOP_SLOT + (BOX_BLOCK_HEIGHT * rows)) as f32;
    let side_panel_path_data = Data::new()
        .move_to((starting_point_x + material_thickness, starting_point_y))
        .vertical_line_to(starting_point_y + SIDE_TAP_FROM_FRONT as f32)
        .horizontal_line_to(starting_point_x)
        .vertical_line_to(starting_point_y + (SIDE_TAP_FROM_FRONT + SIDE_TAP_WIDTH) as f32)
        .horizontal_line_to(starting_point_x + material_thickness)
        .vertical_line_to(
            starting_point_y + (BOX_DEPTH - SIDE_TAP_FROM_FRONT - SIDE_TAP_WIDTH) as f32,
        )
        .horizontal_line_to(starting_point_x)
        .vertical_line_to(starting_point_y + (BOX_DEPTH - SIDE_TAP_FROM_FRONT) as f32)
        .horizontal_line_to(starting_point_x + material_thickness)
        .vertical_line_to(starting_point_y + BOX_DEPTH as f32)
        .horizontal_line_to(starting_point_x + panel_inner_height + (1.0 * material_thickness))
        .vertical_line_to(starting_point_y + (BOX_DEPTH - SIDE_TAP_FROM_FRONT) as f32)
        .horizontal_line_to(starting_point_x + panel_inner_height + (2.0 * material_thickness))
        .vertical_line_to(
            starting_point_y + (BOX_DEPTH - SIDE_TAP_FROM_FRONT - SIDE_TAP_WIDTH) as f32,
        )
        .horizontal_line_to(starting_point_x + panel_inner_height + (1.0 * material_thickness))
        .vertical_line_to(starting_point_y + (SIDE_TAP_FROM_FRONT + SIDE_TAP_WIDTH) as f32)
        .horizontal_line_to(starting_point_x + panel_inner_height + (2.0 * material_thickness))
        .vertical_line_to(starting_point_y + SIDE_TAP_FROM_FRONT as f32)
        .horizontal_line_to(starting_point_x + panel_inner_height + (1.0 * material_thickness))
        .vertical_line_to(starting_point_y)
        .close();

    Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("d", side_panel_path_data)
}

fn generate_top_and_bottom_pieces(
    document: &mut Document,
    starting_point_x: f32,
    columns: usize,
    material_thickness: f32,
    primary_color: &str,
    secondary_color: &str,
) {
    generate_cover_path(
        document,
        starting_point_x,
        0.0,
        columns,
        material_thickness,
        primary_color,
        secondary_color,
    );

    generate_cover_path(
        document,
        starting_point_x,
        (BOX_DEPTH + CLEARANCE_BETWEEN_PATHS) as f32,
        columns,
        material_thickness,
        primary_color,
        secondary_color,
    );
}

fn generate_cover_path(
    document: &mut Document,
    starting_point_x: f32,
    starting_point_y: f32,
    columns: usize,
    material_thickness: f32,
    primary_color: &str,
    secondary_color: &str,
) {
    // Generate cover
    let top_path_data = generate_top_path(
        starting_point_x,
        starting_point_y,
        columns,
        material_thickness,
    );
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", secondary_color)
        .set("d", top_path_data);
    document.append(path);

    for i in 0..columns - 1 {
        let section_width = BOX_WDITH_WITH_LID as f32 + material_thickness;
        let x = starting_point_x + section_width + (i as f32 * section_width);
        let y = starting_point_y + SIDE_TAP_FROM_FRONT as f32;
        let side_tap_hole_path = generate_side_tap_path(x, y, material_thickness, primary_color);
        document.append(side_tap_hole_path);

        let side_tap_hole_path = generate_side_tap_path(
            x,
            y + (BOX_DEPTH - SIDE_TAP_FROM_FRONT - (SIDE_TAP_WIDTH * 2)) as f32,
            material_thickness,
            primary_color,
        );
        document.append(side_tap_hole_path);
    }

    //Generate side panel taps to middle of cover
}

fn generate_side_tap_path(x: f32, y: f32, material_thickness: f32, color: &str) -> Path {
    let data = Data::new()
        .move_to((x, y))
        .vertical_line_to(y + SIDE_TAP_WIDTH as f32)
        .horizontal_line_to(x + material_thickness as f32)
        .vertical_line_to(y)
        .close();

    Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("d", data)
}

fn generate_top_path(
    starting_point_x: f32,
    starting_point_y: f32,
    columns: usize,
    material_thickness: f32,
) -> Data {
    let top_width = top_width(columns, material_thickness);

    Data::new()
        .move_to((starting_point_x, starting_point_y))
        .vertical_line_to(starting_point_y + SIDE_TAP_FROM_FRONT as f32)
        .horizontal_line_to(starting_point_x + material_thickness)
        .vertical_line_to(starting_point_y + (SIDE_TAP_FROM_FRONT + SIDE_TAP_WIDTH) as f32)
        .horizontal_line_to(starting_point_x)
        .vertical_line_to(
            starting_point_y + (BOX_DEPTH - (SIDE_TAP_FROM_FRONT + SIDE_TAP_WIDTH)) as f32,
        )
        .horizontal_line_to(starting_point_x + material_thickness)
        .vertical_line_to(starting_point_y + (BOX_DEPTH - SIDE_TAP_FROM_FRONT) as f32)
        .horizontal_line_to(starting_point_x)
        .vertical_line_to(starting_point_y + BOX_DEPTH as f32)
        .horizontal_line_to(starting_point_x + top_width)
        .vertical_line_to(starting_point_y + (BOX_DEPTH - SIDE_TAP_FROM_FRONT) as f32)
        .horizontal_line_to(starting_point_x - material_thickness + top_width)
        .vertical_line_to(
            starting_point_y + (BOX_DEPTH - (SIDE_TAP_FROM_FRONT + SIDE_TAP_WIDTH)) as f32,
        )
        .horizontal_line_to(starting_point_x + top_width)
        .vertical_line_to(starting_point_y + (SIDE_TAP_FROM_FRONT + SIDE_TAP_WIDTH) as f32)
        .horizontal_line_to(starting_point_x - material_thickness + top_width)
        .vertical_line_to(starting_point_y + SIDE_TAP_FROM_FRONT as f32)
        .horizontal_line_to(starting_point_x + top_width)
        .vertical_line_to(starting_point_y)
        .close()
}

fn top_width(columns: usize, material_thickness: f32) -> f32 {
    ((material_thickness + BOX_WDITH_WITH_LID as f32) * columns as f32) + material_thickness
}
fn generate_side_wing_pair(
    document: &mut Document,
    starting_point_x: f32,
    starting_point_y: f32,
    material_thickness: f32,
    color: &str,
) {
    let path = generate_side_wing(
        starting_point_x,
        starting_point_y,
        material_thickness,
        false,
        &color,
    );
    document.append(path);
    let path = generate_side_wing(
        starting_point_x,
        starting_point_y + (SIDE_WING_WIDTH + CLEARANCE_BETWEEN_PATHS) as f32,
        material_thickness,
        true,
        &color,
    );
    document.append(path);
}

fn height_of_two_side_wings(material_thickness: f32) -> f32 {
    (SIDE_WING_WIDTH * 2 + CLEARANCE_BETWEEN_PATHS) as f32 + material_thickness
}

fn generate_side_wing(
    starting_point_x: f32,
    starting_point_y: f32,
    material_thickness: f32,
    inverted: bool,
    color: &str,
) -> Path {
    let wing_data = if inverted {
        generate_side_wing_inverted_path(starting_point_x, starting_point_y, material_thickness)
    } else {
        generate_side_wing_path(starting_point_x, starting_point_y, material_thickness)
    };

    svg::node::element::Path::new()
        .set("fill", "none")
        .set("stroke", color)
        .set("d", wing_data)
}

fn generate_side_wing_path(
    starting_point_x: f32,
    starting_point_y: f32,
    material_thickness: f32,
) -> Data {
    Data::new()
        .move_to((starting_point_x, starting_point_y))
        .vertical_line_to(starting_point_y + SIDE_WING_WIDTH as f32)
        .horizontal_line_to(SIDE_WING_SLOT_FROM_FRONT)
        .vertical_line_to(starting_point_y + material_thickness + SIDE_WING_WIDTH as f32)
        .horizontal_line_to(SIDE_WING_SLOT_FROM_FRONT + SIDE_WING_SLOT_WIDTH)
        .vertical_line_to(starting_point_y + SIDE_WING_WIDTH as f32)
        .horizontal_line_to(third_side_wing_tap_position_from_front())
        .vertical_line_to(starting_point_y + SIDE_WING_WIDTH as f32 + material_thickness)
        .horizontal_line_to(third_side_wing_tap_position_from_front() + SIDE_WING_SLOT_WIDTH)
        .vertical_line_to(starting_point_y + SIDE_WING_WIDTH as f32)
        .horizontal_line_to(BOX_DEPTH)
        .vertical_line_to(starting_point_y)
        .close()
}

fn generate_side_wing_inverted_path(
    starting_point_x: f32,
    starting_point_y: f32,
    material_thickness: f32,
) -> Data {
    Data::new()
        .move_to((starting_point_x, starting_point_y + material_thickness))
        .horizontal_line_to(second_side_wing_tap_position_from_front())
        .vertical_line_to(starting_point_y)
        .horizontal_line_to(second_side_wing_tap_position_from_front() + SIDE_WING_SLOT_WIDTH)
        .vertical_line_to(starting_point_y + material_thickness)
        .horizontal_line_to(fourth_side_wing_tap_position_from_front())
        .vertical_line_to(starting_point_y)
        .horizontal_line_to(BOX_DEPTH - SIDE_WING_SLOT_FROM_FRONT)
        .vertical_line_to(starting_point_y + material_thickness)
        .horizontal_line_to(BOX_DEPTH)
        .vertical_line_to(starting_point_y + material_thickness + SIDE_WING_WIDTH as f32)
        .horizontal_line_to(starting_point_x)
        .close()
}

fn third_side_wing_tap_position_from_front() -> usize {
    BOX_DEPTH
        - (SIDE_WING_SLOT_FROM_FRONT
            + SIDE_WING_SLOT_WIDTH
            + SIDE_WING_SLOT_SPACING
            + SIDE_WING_SLOT_WIDTH)
}
fn second_side_wing_tap_position_from_front() -> usize {
    SIDE_WING_SLOT_FROM_FRONT + SIDE_WING_SLOT_WIDTH + SIDE_WING_SLOT_SPACING
}

fn fourth_side_wing_tap_position_from_front() -> usize {
    BOX_DEPTH - (SIDE_WING_SLOT_FROM_FRONT + SIDE_WING_SLOT_WIDTH)
}
