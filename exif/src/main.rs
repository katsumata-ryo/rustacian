use std::env;
use rexiv2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let meta = rexiv2::Metadata::new_from_path(&file_path).unwrap();

    println!("{:?}", meta.get_pixel_width());
    println!("{:?}", meta.get_pixel_height());
    println!("F: {:?}", meta.get_fnumber());
    println!("ISO: {:?}", meta.get_iso_speed());
    println!("{:?}", meta.get_exposure_time());
}
