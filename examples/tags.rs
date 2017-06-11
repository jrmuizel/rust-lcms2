extern crate lcms2;
use lcms2::*;

use std::env;

fn main() {
    let profile = if let Some(path) = env::args().nth(1) {
        Profile::new_file(path).unwrap()
    } else {
        Profile::new_srgb()
    };

    for &info in &[InfoType::Description, InfoType::Manufacturer, InfoType::Model, InfoType::Copyright] {
        let data = profile.info(info, Locale::none());
        println!("{:?} = {:?}", info, data);
    }

    for sig in profile.tag_signatures() {
        let tag = profile.read_tag(sig);
        println!("{:?} = {:?}", sig, tag);
    }
}