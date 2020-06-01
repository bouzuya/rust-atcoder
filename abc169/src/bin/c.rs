use proconio::input;

fn main() {
    input! {
        a: u64,
        mut b: String,
    };
    b.retain(|c| c != '.');
    let d = b.parse::<u64>().unwrap();
    let ans = a * d / 100_u64;
    println!("{}", ans);
}
