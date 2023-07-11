use proconio::input;

fn main() {
    input! {
        k: u32,
        a: String,
        b: String,
    };
    let a = usize::from_str_radix(&a, k).unwrap();
    let b = usize::from_str_radix(&b, k).unwrap();
    let ans = a * b;
    println!("{}", ans);
}
