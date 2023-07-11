use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        t: [usize; n],
        m: usize,
        px: [(Usize1, usize); m],
    };
    let sum = t.iter().sum::<usize>();
    for (p, x) in px {
        let ans = sum - t[p] + x;
        println!("{}", ans);
    }
}
