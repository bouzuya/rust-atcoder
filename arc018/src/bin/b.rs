use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (x1, y1) = xy[i];
                let (x2, y2) = xy[j];
                let (x3, y3) = xy[k];

                let a1 = x2 - x1;
                let a2 = y2 - y1;
                let b1 = x3 - x1;
                let b2 = y3 - y1;

                if a2 * b1 == a1 * b2 {
                    continue;
                }

                if (a1 * b2 - a2 * b1).abs() % 2 != 0 {
                    continue;
                }
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
