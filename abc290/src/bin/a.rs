use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [Usize1; m],
    };
    let ans = b.into_iter().map(|b_i| a[b_i]).sum::<usize>();
    println!("{}", ans);
}
