use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let max = a.max(b);
    let min = a.min(b);
    let ans = (16 - min * 2) >= (max - min) * 2;
    println!("{}", if ans { "Yay!" } else { ":(" });
}
