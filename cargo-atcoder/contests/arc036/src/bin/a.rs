use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        t: [i64; n],
    };
    for (i, &t_i) in t.iter().enumerate().skip(2) {
        if t[i - 2] + t[i - 1] + t_i < k {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", -1);
}
