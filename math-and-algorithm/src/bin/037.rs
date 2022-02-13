use proconio::input;

type P = (i64, i64);

fn main() {
    input! {
        xy: [(i64, i64); 4],
    };
    let f = |(x1, y1): P, (x2, y2): P, (x, y): P| -> i64 {
        ((x2 - x1) * (y - y1) - (y2 - y1) * (x - x1)).signum()
        // ((x1 - x2) * (y - y1) + (y1 - y2) * (x1 - x)).signum()
    };
    let g = |(x1, y1): P, (x2, y2): P, (x, y): P| -> bool {
        ((x1..=x2).contains(&x) || (x2..=x1).contains(&x))
            && ((y1..=y2).contains(&y) || (y2..=y1).contains(&y))
            && ((y * (x1 - x2)) + (y1 * (x2 - x)) + (y2 * (x - x1))) == 0
    };

    let f1 = f(xy[0], xy[1], xy[2]);
    let f2 = f(xy[0], xy[1], xy[3]);
    let f3 = f(xy[2], xy[3], xy[0]);
    let f4 = f(xy[2], xy[3], xy[1]);
    let ans = (f1 * f2 < 0 && f3 * f4 < 0) || {
        g(xy[0], xy[1], xy[2])
            || g(xy[0], xy[1], xy[3])
            || g(xy[2], xy[3], xy[0])
            || g(xy[2], xy[3], xy[1])
    };

    println!("{}", if ans { "Yes" } else { "No" });
}
