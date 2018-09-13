extern crate average;
extern crate float_cmp;

use average::Mean;
use float_cmp::ApproxEqRatio;
use std::cmp::Ordering;

struct Point(pub f64, pub f64);

fn get_middle(Point(x1, y1): &Point, Point(x2, y2): &Point) -> Option<f64> {
    if !y1.approx_eq_ratio(&y2, 0.01) {
        None
    } else {
        Some((x1 + x2) / 2.0)
    }
}

fn is_symmetric(points: &[Point]) -> bool {
    if points.len() < 2 {
        return true;
    }

    let mut points = points.iter().collect::<Vec<_>>();
    let middle: Mean = points.iter().map(|Point(x, ..)| *x).collect();
    let middle = middle.mean();

    points.sort_by(|Point(x1, y1), Point(x2, y2)| {
        let x_diff = x1.partial_cmp(x2).unwrap();
        match x_diff {
            Ordering::Equal => {
                let (y1, y2) = if x1 < &middle { (y1, y2) } else { (y2, y1) };
                y1.partial_cmp(y2).unwrap()
            }
            other => other,
        }
    });
    for i in 1..points.len() / 2 {
        match get_middle(points[i], points[points.len() - 1 - i]) {
            Some(new_middle) => if !middle.approx_eq_ratio(&new_middle, 0.01) {
                return false;
            },
            None => return false,
        };
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let points = [
            Point(-10.0, 20.0),
            Point(-10.0, 15.0),
            Point(0.0, -10.0),
            Point(10.0, 15.0),
            Point(10.0, 20.0),
        ];
        assert!(is_symmetric(&points));
    }

    #[test]
    fn it_works2() {
        let points = [
            Point(-10.0, 20.0),
            Point(-10.0, 15.0),
            Point(-5.0, -10.0),
            Point(10.0, 15.0),
            Point(10.0, 20.0),
        ];
        assert!(!is_symmetric(&points));
    }
}
