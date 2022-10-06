use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    };
    let mut count = vec![0_i64; d + 2];
    for (l, r) in lr {
        count[l] += 1;
        count[r + 1] -= 1;
    }

    for i in 0..d {
        count[i + 1] += count[i];
    }

    for c in count.into_iter().skip(1).take(d) {
        println!("{}", c);
    }
}
