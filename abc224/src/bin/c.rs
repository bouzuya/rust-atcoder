use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut count = 0_usize;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (x_i, y_i) = xy[i];
                let (x_j, y_j) = xy[j];
                let (x_k, y_k) = xy[k];

                if (y_j - y_i) * (x_k - x_i) != (y_k - y_i) * (x_j - x_i) {
                    count += 1;
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
