use proconio::input;

fn main() {
    input! {
        k: u32,
        a: String,
        b: String,
    };
    let ans = usize::from_str_radix(&a, k).unwrap() * usize::from_str_radix(&b, k).unwrap();
    println!("{}", ans);
}
