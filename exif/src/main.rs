use rexiv2;

fn main() {
    let file = "/Users/ryo/Downloads/Photos/DSC04117.jpg";
    let meta = rexiv2::Metadata::new_from_path(&file).unwrap();

    println!("{:?}", meta.get_pixel_width());
    println!("{:?}", meta.get_pixel_height());
    println!("F: {:?}", meta.get_fnumber());
    println!("ISO: {:?}", meta.get_iso_speed());
    println!("{:?}", meta.get_exposure_time());
}
