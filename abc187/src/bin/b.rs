use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            let is_ok = {
                let (x_i, y_i) = xy[i];
                let (x_j, y_j) = xy[j];
                (y_j - y_i).abs() <= (x_j - x_i).abs()
            };
            if is_ok {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
