use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize,
    };
    let mut ans = 1_usize;
    for i in 0..r {
        ans *= n - i;
        ans /= i + 1;
    }
    println!("{}", ans);
}
