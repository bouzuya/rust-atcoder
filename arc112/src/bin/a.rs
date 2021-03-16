use proconio::input;

fn main() {
    input! {
        t: usize,
        lr: [(i64, i64); t],
    };
    for (l_i, r_i) in lr {
        let x = if r_i < l_i * 2 {
            0
        } else {
            let n = (r_i - l_i) - l_i + 1;
            (n + 1) * n / 2
        };
        println!("{}", x);
    }
}
