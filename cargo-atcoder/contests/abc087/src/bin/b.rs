use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        x: i64,
    };
    let mut count = 0;
    for a_i in 0..=a {
        for b_j in 0..=b {
            for c_k in 0..=c {
                if 500 * a_i + 100 * b_j + 50 * c_k == x {
                    count += 1;
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
