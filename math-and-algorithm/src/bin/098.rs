use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
        ab: (i64, i64)
    };
    xy.push(xy[0]);
    let mut inside = false;
    for i in 0..n {
        let (x, y) = ab;
        let ((x1, y1), (x2, y2)) = if xy[i].1 < xy[i + 1].1 {
            (xy[i], xy[i + 1])
        } else {
            (xy[i + 1], xy[i])
        };
        if (y1..y2).contains(&y) && (x - x1) * (y2 - y1) < (y - y1) * (x2 - x1) {
            inside = !inside;
        }
    }
    let ans = inside;
    println!("{}", if ans { "INSIDE" } else { "OUTSIDE" });
}
