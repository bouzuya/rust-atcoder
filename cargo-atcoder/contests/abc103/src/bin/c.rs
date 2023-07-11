use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let ans = a.into_iter().sum::<usize>();
    println!("{}", ans);
}
