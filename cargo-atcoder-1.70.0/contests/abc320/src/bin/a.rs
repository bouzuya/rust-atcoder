use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    };
    let ans = a.pow(b) + b.pow(a);
    println!("{}", ans);
}
