use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        abc: [(usize, usize, usize); n],
    };

    let mut dp = vec![None; x + 1];
    dp[x] = Some((0_usize, 1_000_000_000_usize));
    for (a_i, b_i, c_i) in abc.iter().copied() {
        let mut next = vec![None; x + 1];
        for j in 0..=x {
            let max = |ca1: Option<(usize, usize)>, ca2: Option<(usize, usize)>| match (ca1, ca2) {
                (None, None) => None,
                (None, Some(ca2)) => Some(ca2),
                (Some(ca1), None) => Some(ca1),
                (Some(ca1), Some(ca2)) => Some(ca1.max(ca2)),
            };
            next[j] = max(next[j], dp[j]);
            if j + b_i <= x {
                next[j] = max(next[j], dp[j + b_i].map(|(c1, a1)| (c1 + c_i, a1 - a_i)));
            }
        }
        dp = next;
    }

    let (c, a, b) = dp
        .into_iter()
        .enumerate()
        .filter_map(|(b, ca1)| ca1.map(|(c, a)| (c, a, b)))
        .fold((0_usize, 0_usize, 0_usize), |abc1, abc2| {
            match abc1.cmp(&abc2) {
                std::cmp::Ordering::Less => abc2,
                std::cmp::Ordering::Equal => unreachable!(),
                std::cmp::Ordering::Greater => abc1,
            }
        });
    println!("{} {} {}", c, a, b);
}
