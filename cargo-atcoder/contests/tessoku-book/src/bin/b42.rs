use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    };

    let mut max = 0_i64;
    for s1 in &[false, true] {
        for s2 in &[false, true] {
            let mut sum = 0_i64;
            for (a, b) in ab.iter().copied() {
                let score = match (s1, s2) {
                    (true, true) => a + b,
                    (true, false) => a - b,
                    (false, true) => -a + b,
                    (false, false) => -a - b,
                };
                if score > 0 {
                    sum += score;
                }
            }
            max = max.max(sum);
        }
    }
    let ans = max;
    println!("{}", ans);
}
