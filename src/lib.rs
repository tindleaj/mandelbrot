extern crate image;
extern crate num;

use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::fs::File;
use std::str::FromStr;

pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;

    Ok(())
}

pub fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { im: 0f64, re: 0f64 };

    for i in 0..limit {
        z = z * z + c;

        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

pub fn parse_pair<T: FromStr>(input: &str, separator: char) -> Option<(T, T)> {
    match input.find(separator) {
        None => None,
        Some(index) => match (
            T::from_str(&input[..index]),
            T::from_str(&input[index + 1..]),
        ) {
            (Ok(i), Ok(j)) => Some((i, j)),
            _ => None,
        },
    }
}

pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        _ => None,
    }
}

/// Given the row and col on the output images, returns the
/// corresponding point on the complex plane
pub fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

pub fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zero_case() {
        assert_eq!(escape_time(Complex { im: 0.0, re: 0.0 }, 1000), None)
    }

    #[test]
    fn test_real_number_escape() {
        assert_eq!(escape_time(Complex { im: 0.0, re: 2.0 }, 1000), Some(1))
    }

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<f64>("0.5x1.0", 'x'), Some((0.5, 1.0)));
        assert_eq!(parse_pair::<f64>("0.5,1.0", 'x'), None);
        assert_eq!(parse_pair::<i32>("5x1", 'x'), Some((5, 1)));
        assert_eq!(parse_pair::<f64>("    ,      ", ','), None);
        assert_eq!(parse_pair::<f64>("1,2xy", ','), None);
    }

    #[test]
    fn test_parse_complex() {
        assert_eq!(
            parse_complex("0.25,-1.222"),
            Some(Complex {
                re: 0.25,
                im: -1.222
            })
        );
        assert_eq!(parse_complex(",-1.222"), None);
    }

    #[test]
    fn test_pixel_to_point() {
        assert_eq!(
            pixel_to_point(
                (100, 100),
                (25, 75),
                Complex { re: -1.0, im: 1.0 },
                Complex { re: 1.0, im: -1.0 }
            ),
            Complex { re: -0.5, im: -0.5 }
        )
    }
}
