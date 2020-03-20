extern crate clap;
extern crate fraction;
extern crate image;

use clap::App;
use std::path::Path;
use std::ffi::OsStr;

type F = fraction::Fraction;

fn limit_fraction(numerator: u64, denominator: u64) -> (u64, u64) {
    // Euclid's two-thousand-year-old algorithm for finding the greatest common divisor.
    let mut x = numerator;
    let mut y = denominator;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }

    (numerator / x, denominator / x)
}

fn calculate_aspect_ratio(width: u32, height: u32) -> String {
    let fraction = F::from(width) / F::from(height);

    let (mut numerator, mut denominator) =
        limit_fraction(*fraction.numer().unwrap(), *fraction.denom().unwrap());

    // Approximate ratio for common standards.
    if numerator == 64 && denominator == 27 {
        numerator = 29;
        denominator = 9;
    } else if numerator == 43 && denominator == 18 {
        numerator = 29;
        denominator = 9;
    } else if numerator == 8 && denominator == 5 {
        numerator = 16;
        denominator = 10;
    }

    format!("{}:{}", numerator, denominator)
}

fn main() {
    // Set up clap
    let matches = App::new("hiritsu")
        .version("0.1.0")
        .author("Tryton Van Meer <trytonvanmeer@protonmail.com>")
        .about("Gets the aspect ratio and resolution of images.")
        .args_from_usage(
            "-r, --rename 'Rename the image'
            <FILE>      'Sets the image to use'"
        )
        .get_matches();

    // Check if file exists
    let filepath = Path::new(matches.value_of("FILE").unwrap());
    if !filepath.exists() {
        println!("{}: No such file", filepath.display());
        return;
    }

    let (width, height) = image::image_dimensions(filepath).unwrap();
    let ratio = calculate_aspect_ratio(width, height);

    if matches.occurrences_of("rename") == 1 {
        let extension = filepath.extension().and_then(OsStr::to_str).unwrap();

        println!("{}", extension);
    } else {
        println!("Width:  {}", width);
        println!("Height: {}", height);
        println!("Ratio:  {}", ratio);
    }
}
