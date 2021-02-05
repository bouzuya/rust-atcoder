use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = a <= 8 && b <= 8;
    println!("{}", if ans { "Yay!" } else { ":(" });
}
