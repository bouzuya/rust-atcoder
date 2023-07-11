use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = a.to_string().len() * b.to_string().len();
    println!("{}", ans);
}
