use proconio::input;

fn main() {
    input! {
        l: usize,
    };
    let mut x = 1;
    for i in 1..=11 {
        x *= l - i;
        x /= i;
    }
    let ans = x;
    println!("{}", ans);
}
