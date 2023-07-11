use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n],
    };
    let mut up = true;
    let mut max_score = 0;
    let mut s = 0;
    for (i, &h_i) in h.iter().enumerate().skip(1) {
        let h_p = h[i - 1];
        if up {
            if h_p < h_i {
            } else if h_p > h_i {
                up = false;
                max_score = std::cmp::max(max_score, i + 1 - s);
            } else {
                unreachable!();
            }
        } else {
            if h_p < h_i {
                up = true;
                s = i - 1;
            } else if h_p > h_i {
                max_score = std::cmp::max(max_score, i + 1 - s);
            } else {
                unreachable!();
            }
        }
    }
    max_score = std::cmp::max(max_score, n - s);
    println!("{}", max_score);
}
