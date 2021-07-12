use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };
    let max_x = 1_000_000;
    let mut count = vec![0_i64; max_x + 1 + 1];
    for (a_i, b_i) in ab {
        count[a_i] += 1;
        count[b_i + 1] -= 1;
    }

    for i in 0..max_x {
        count[i + 1] += count[i];
    }
    let ans = *count[0..=max_x].iter().max().unwrap();
    println!("{}", ans);
}
