use proconio::input;

fn main() {
    input! {
        deg: usize,
        dis: usize,
    };
    let mut dir = "N".to_string();
    for (r, d) in vec![
        (1125..3375, "NNE"),
        (3375..5625, "NE"),
        (5625..7875, "ENE"),
        (7875..10125, "E"),
        (10125..12375, "ESE"),
        (12375..14625, "SE"),
        (14625..16875, "SSE"),
        (16875..19125, "S"),
        (19125..21375, "SSW"),
        (21375..23625, "SW"),
        (23625..25875, "WSW"),
        (25875..28125, "W"),
        (28125..30375, "WNW"),
        (30375..32625, "NW"),
        (32625..34875, "NNW"),
    ] {
        if r.contains(&(deg * 10)) {
            dir = d.to_string();
            break;
        }
    }
    let mut w = 12;
    for (i, x) in vec![2, 15, 33, 54, 79, 107, 138, 171, 207, 244, 284, 326]
        .into_iter()
        .map(|x| (x * 10 + 4) * 6 / 10)
        .enumerate()
    {
        if dis <= x {
            w = i;
            break;
        }
    }
    if w == 0 {
        dir = "C".to_string();
    }
    println!("{} {}", dir, w);
}
