use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let mut abc = vec![a, b, c];
    abc.sort();
    let ans = abc[1] == b;
    println!("{}", if ans { "Yes" } else { "No" });
}
