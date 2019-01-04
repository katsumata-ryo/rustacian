use std::env;
use rexiv2;

fn main() {
    let arguments = env::args().skip(1);
    for argument in arguments {
        let meta = rexiv2::Metadata::new_from_path(&argument).unwrap();

        println!("[{}]", argument);
        println!(
            "Body: {:?} / Lens: {:?} / F: {:?} / ISO: {:?} / Shutter: {:?}/{:?}",
            meta.get_tag_string("Exif.Image.Model").unwrap(),
            meta.get_tag_string("Exif.Photo.LensModel").unwrap(),
            meta.get_fnumber().unwrap(),
            meta.get_iso_speed().unwrap(),
            meta.get_exposure_time().unwrap().numer(), meta.get_exposure_time().unwrap().denom(),
        );
    }
}
