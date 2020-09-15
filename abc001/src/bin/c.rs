use proconio::input;

fn main() {
    input! {
        deg: i64,
        dis: i64,
    };
    let dir = if deg <= 112 {
        "N"
    } else if deg <= 337 {
        "NNE"
    } else if deg <= 562 {
        "NE"
    } else if deg <= 787 {
        "ENE"
    } else if deg <= 1012 {
        "E"
    } else if deg <= 1237 {
        "ESE"
    } else if deg <= 1462 {
        "SE"
    } else if deg <= 1687 {
        "SSE"
    } else if deg <= 1912 {
        "S"
    } else if deg <= 2137 {
        "SSW"
    } else if deg <= 2362 {
        "SW"
    } else if deg <= 2587 {
        "WSW"
    } else if deg <= 2812 {
        "W"
    } else if deg <= 3037 {
        "WNW"
    } else if deg <= 3262 {
        "NW"
    } else if deg <= 3487 {
        "NNW"
    } else {
        "N"
    };
    let d = ((dis as f64 * 100_f64 / 60_f64 + 5_f64) / 10_f64) as i64;
    let w = if d <= 2 {
        0
    } else if d <= 15 {
        1
    } else if d <= 33 {
        2
    } else if d <= 54 {
        3
    } else if d <= 79 {
        4
    } else if d <= 107 {
        5
    } else if d <= 138 {
        6
    } else if d <= 171 {
        7
    } else if d <= 207 {
        8
    } else if d <= 244 {
        9
    } else if d <= 284 {
        10
    } else if d <= 326 {
        11
    } else {
        12
    };
    println!("{} {}", if w == 0 { "C" } else { dir }, w);
}
