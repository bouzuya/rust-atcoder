use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(Usize1, Usize1); m],
    };
    let mut count = vec![0_i64; n + 1];
    for (l, r) in lr {
        count[l] += 1;
        count[r + 1] -= 1;
    }
    for i in 0..n {
        count[i + 1] += count[i];
    }

    let ans = count.iter().filter(|&&c| c >= (m as i64)).count();
    println!("{}", ans);
}
