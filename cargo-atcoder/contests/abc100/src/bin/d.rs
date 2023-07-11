use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xyz: [(i64, i64, i64); n],
    };

    let mut max = -(1_i64 << 60);
    for bits in 0..1 << 3 {
        let sign = (0..3)
            .map(|i| if ((bits >> i) & 1) == 1 { 1 } else { -1 })
            .collect::<Vec<i64>>();
        let mut b = xyz
            .iter()
            .copied()
            .map(|(x, y, z)| x * sign[0] + y * sign[1] + z * sign[2])
            .collect::<Vec<i64>>();
        b.sort();
        let sum = b.iter().rev().take(m).sum::<i64>();
        max = max.max(sum);
    }

    let ans = max;
    println!("{}", ans);
}
