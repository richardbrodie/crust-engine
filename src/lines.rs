fn plot_low(x0: isize, y0: isize, x1: isize, y1: isize) {
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi = 0;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut d = (2 * dy) - dx;
    let mut y = y0;

    for x in x0..x1 {
        // plot(x, y)
        if d > 0 {
            y = y + yi;
            d = d + (2 * (dx - dy));
        } else {
            d = d + 2 * dx;
        }
    }
}

fn lerp(a: isize, b: isize, t: f32) -> f32 {
    a as f32 * (1.0 - t) + b as f32 * t
}

// fn lerp_point(p0: Point, p1: Point, t:f32) -> Point {
// Vec2
// }

#[cfg(test)]
mod tests {
    use crate::lines::lerp;

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0, 1, 0.5), 0.5);
        assert_eq!(lerp(0, 100, 0.5), 50.0);
        assert_eq!(lerp(3, 5, 0.5), 4.0);
        assert_eq!(lerp(5, 3, 0.5), 4.0);
    }
}
