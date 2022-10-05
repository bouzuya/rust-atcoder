use proconio::input;

fn main() {
    input! {
        n: String,
    };
    let ans = usize::from_str_radix(&n, 2).unwrap();
    println!("{}", ans);
}
