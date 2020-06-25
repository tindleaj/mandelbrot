extern crate num;

use num::Complex;
use std::str::FromStr;

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

fn parse_pair<T: FromStr>(input: &str, separator: char) -> Option<(T, T)> {
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
}
