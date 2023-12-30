pub fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> (u8, u8, u8) {
    check_bounds(hue, saturation, value);

    let c = value * saturation;
    let h = hue / 60.0;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let m = value - c;

    let (r, g, b): (f64, f64, f64) = if (0.0..1.0).contains(&h) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&h) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&h) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&h) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&h) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn check_bounds(hue: f64, saturation: f64, value: f64) {
    macro_rules! bp {
        ($n:literal, $f:literal, $t:literal, $g:expr) => {
            panic!(
                "param {} must be between {} and {} inclusive; was: {}",
                $n, $f, $t, $g
            )
        };
    }

    if !(0.0..=360.0).contains(&hue) {
        bp!("hue", "0", "360", hue)
    } else if !(0.0..=1.0).contains(&saturation) {
        bp!("saturation", "0", "1", saturation)
    } else if !(0.0..=1.0).contains(&value) {
        bp!("value", "0", "1", value)
    }
}
