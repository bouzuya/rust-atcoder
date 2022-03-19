use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xy: [(i64, i64); n],
        ab: (i64, i64)
    };
    xy.push(xy[0]);
    let mut count = 0_i64;
    for i in 0..n {
        let (x, y) = ab;
        let (x1, y1) = xy[i];
        let (x2, y2) = xy[i + 1];
        if (y1..y2).contains(&y) {
            // ↑
            if (x - x1) * (y2 - y1) < (y - y1) * (x2 - x1) {
                count += 1;
            }
        } else if (y2..y1).contains(&y) {
            // ↓
            if (x - x2) * (y1 - y2) < (y - y2) * (x1 - x2) {
                count -= 1;
            }
        }
    }
    let ans = count % 2 != 0;
    println!("{}", if ans { "INSIDE" } else { "OUTSIDE" });
}
