use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = a < c.pow(b as u32);
    println!("{}", if ans { "Yes" } else { "No" });
}
