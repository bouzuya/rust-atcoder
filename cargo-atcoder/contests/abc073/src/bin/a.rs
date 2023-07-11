use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = n.to_string().contains('9');
    println!("{}", if ans { "Yes" } else { "No" });
}
