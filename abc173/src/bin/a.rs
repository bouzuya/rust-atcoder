use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut m = 0;
    for _ in 0.. {
        if m >= n {
            break;
        }
        m += 1000;
    }
    let ans = m - n;
    println!("{}", ans);
}
