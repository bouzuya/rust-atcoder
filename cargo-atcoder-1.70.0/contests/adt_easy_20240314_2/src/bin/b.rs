use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = a.pow(b as u32);
    println!("{}", ans);
}
