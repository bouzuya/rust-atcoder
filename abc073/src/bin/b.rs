use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        lr: [(Usize1, Usize1); n],
    };
    let mut seats = vec![0_i64; 100_000 + 1];
    for (l, r) in lr {
        seats[l] += 1;
        seats[r + 1] -= 1;
    }
    for i in 0..100_000 {
        seats[i + 1] += seats[i];
    }
    let ans = seats.iter().sum::<i64>();
    println!("{}", ans);
}
