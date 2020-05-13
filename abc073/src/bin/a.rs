use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = if n / 10 == 9 || n % 10 == 9 {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
