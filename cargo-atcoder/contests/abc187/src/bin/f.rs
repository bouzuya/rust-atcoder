use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let ans = n - a.len();
    println!("{}", ans);
    todo!()
}
