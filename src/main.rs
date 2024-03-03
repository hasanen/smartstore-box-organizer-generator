use clap::Parser;
mod generator;
use crate::generator::generate_svg;

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
    output_filename: String,
}

fn main() {
    let args = Args::parse();

    println!(
        "So you want to generate organizer with {} rows and {} columns, using {}mm thick material.",
        args.rows, args.columns, args.material_thickness
    );

    let svg = generate_svg(args.rows, args.columns, args.material_thickness);
    let filename = format!("{}.svg", args.output_filename);
    svg::save(&filename, &svg).unwrap();
    println!("Saved to {}", &filename);
}
