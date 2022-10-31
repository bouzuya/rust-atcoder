use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        ab: [(i64, i64); n],
    };
    let mut max = 0_usize;
    for x in ab.iter().copied().map(|(x, _)| x) {
        for y in ab.iter().copied().map(|(_, y)| y) {
            let count = ab
                .iter()
                .copied()
                .filter(|&(a_i, b_i)| (x..=x + k).contains(&a_i) && (y..=y + k).contains(&b_i))
                .count();
            max = max.max(count);
        }
    }
    let ans = max;
    println!("{}", ans);
}
