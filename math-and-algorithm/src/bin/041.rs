use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    };
    let mut count = vec![0_i64; t + 1];
    for (l, r) in lr {
        count[l] += 1;
        count[r] -= 1;
    }
    let mut p = count[0];
    for i in (0..=t).skip(1) {
        count[i] += p;
        p = count[i];
    }
    for i in 0..t {
        println!("{}", count[i]);
    }
}
