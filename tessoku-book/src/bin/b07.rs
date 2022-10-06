use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n]
    };
    let mut count = vec![0_i64; t + 1];
    for (l, r) in lr {
        count[l] += 1;
        count[r] -= 1;
    }
    for i in 0..t {
        count[i + 1] += count[i];
    }
    for c in count.into_iter().take(t) {
        println!("{}", c);
    }
}
