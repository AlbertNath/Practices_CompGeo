// Module tree declaration
mod i_o;
mod geo_structs;
mod algorithmia;

// Imports
use std::env;
use algorithmia::{algorithms::Algorithm, extreme_points, extreme_segments, gift_wrapping};

use crate::i_o::{reader, writter};
use crate::geo_structs::point::Point;

/// # Geometría computacional, semestre 2023-2
/// ## Practice 2: Extreme Points.
/// ### Author: [Medel Piña Alberto Natanael](https://github.com/AlbertNath)
///
/// Implementation of Extreme Segments algorithm.
fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() - 1 == 0{
        reader::err("Too few arguments.");
    }

    let points: Vec<Point> = reader::parse_points(&args);
    let mut algorithm: Algorithm = Algorithm::ExtremePoints;

    if let Some(a) = reader::get_algorithm(&args) {
        algorithm = a;
    } else {
        reader::err("Invalid algorithm flag detected.");
    }

    let mut _result: Vec<Point> = vec![];
    match algorithm {
        Algorithm::ExtremePoints => {
            println!("Extreme!");
            _result = extreme_points::extreme_points(&points, points.len());
        }

        Algorithm::ExtremeSegments => {
            println!("Segments");
            _result = extreme_segments::extreme_segments(&points, points.len());
        }

        Algorithm::GiftWrapping => {
            println!("Wrapping up!");
            _result = gift_wrapping::jarvis_march(&points, points.len());
        }
    }

    writter::write_result(&_result)
}
