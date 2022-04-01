use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (x1, y1) = xy[i];
                let (x2, y2) = xy[j];
                let (x3, y3) = xy[k];
                let ok = (x2 - x1 == 0 && x1 == x3)
                    || ((y2 - y1) * x3 - (x2 - x1) * y3 + (x2 * y1 - x1 * y2) == 0);
                if ok {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
