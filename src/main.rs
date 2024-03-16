use clap::Parser;
use container_rack_lib::generate_svg;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of rows of boxes
    #[arg(short, long)]
    rows: usize,

    /// Number columns of boxes
    #[arg(short, long)]
    columns: usize,

    /// Thickness of the plywood or other material
    #[arg(short, long)]
    material_thickness: f32,

    /// Name of the file to save the SVG to
    #[arg(short, long)]
    output_filename: Option<String>,

    /// Primary color of the line that will be cut first
    #[clap(short, long, default_value = "black")]
    primary_color: String,

    /// Primary color of the line that will be cut first
    #[clap(short, long, default_value = "blue")]
    secondary_color: String,
}

fn main() {
    let args = Args::parse();

    println!(
        "So you want to generate organizer with {} rows and {} columns, using {}mm thick material.",
        args.rows, args.columns, args.material_thickness
    );

    let svg = generate_svg(
        args.rows,
        args.columns,
        args.material_thickness,
        &args.primary_color,
        &args.secondary_color,
    );
    let filename = match args.output_filename {
        Some(name) => name,
        None => format!(
            "organizer_{}_rows_{}_columns_{}mm_thick",
            args.rows, args.columns, args.material_thickness
        ),
    };
    let filename_with_extension = format!("{}.svg", filename);
    svg::save(&filename_with_extension, &svg).unwrap();
    println!("Saved to {}", &filename_with_extension);
}
