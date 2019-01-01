use std::env;
use rexiv2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let meta = rexiv2::Metadata::new_from_path(&file_path).unwrap();

    println!("Width: {:?}", meta.get_pixel_width());
    println!("Height: {:?}", meta.get_pixel_height());
    println!("F: {:?}", meta.get_fnumber().unwrap());
    println!("ISO: {:?}", meta.get_iso_speed().unwrap());
    println!("Shutter: {:?}/{:?}", meta.get_exposure_time().unwrap().numer(), meta.get_exposure_time().unwrap().denom());
}
