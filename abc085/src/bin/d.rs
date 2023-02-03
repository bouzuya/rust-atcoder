use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        ab: [(usize, usize); n],
    };
    let max_a = ab.iter().copied().map(|(a, _)| a).max().unwrap();
    let mut bs = ab
        .iter()
        .copied()
        .filter(|&(_, b)| b > max_a)
        .map(|(_, b)| b)
        .collect::<Vec<usize>>();
    bs.sort();
    let mut count = 0_usize;
    let mut sum = 0_usize;
    for b in bs.into_iter().rev() {
        count += 1;
        sum += b;
        if sum >= h {
            println!("{}", count);
            return;
        }
    }
    let ans = count + (((h - sum) + max_a - 1) / max_a);
    println!("{}", ans);
}
