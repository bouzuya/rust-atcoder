use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        a: [i64; 4]
    };
    let ans = a.iter().min().unwrap();
    println!("{}", ans);
}
