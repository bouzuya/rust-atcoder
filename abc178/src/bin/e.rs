use proconio::input;

#[derive(Clone, Copy)]
struct P {
    x: i64,
    y: i64,
}

fn distance(p1: P, p2: P) -> i64 {
    (p2.x - p1.x).abs() + (p2.y - p1.y).abs()
}

fn closest(p0: P, max: P, xy: &Vec<(i64, i64)>) -> P {
    let mut p_n = max;
    let mut d_n = distance(p0, max);
    for &(x_i, y_i) in xy.iter() {
        let p = P { x: x_i, y: y_i };
        let d = distance(p0, p);
        if d < d_n {
            p_n = p;
            d_n = d;
        }
    }
    p_n
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let max = 1_000_000_000;
    let bl = P { x: 0, y: 0 };
    let br = P { x: max, y: 0 };
    let tl = P { x: 0, y: max };
    let tr = P { x: max, y: max };
    let p_bl = closest(bl, tr, &xy);
    let p_br = closest(br, tl, &xy);
    let p_tl = closest(tl, br, &xy);
    let p_tr = closest(tr, bl, &xy);
    let ans = std::cmp::max(distance(p_bl, p_tr), distance(p_br, p_tl));
    println!("{}", ans);
}
