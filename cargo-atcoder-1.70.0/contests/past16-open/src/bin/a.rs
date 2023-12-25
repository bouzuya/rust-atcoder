use proconio::input;

fn main() {
    input! {
        m: usize,
    };
    let ans = (4..=9).contains(&m);
    println!("{}", if ans { "Yes" } else { "No" });
}
