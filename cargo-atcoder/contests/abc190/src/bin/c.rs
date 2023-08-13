use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        k: usize,
        cd: [(Usize1, Usize1); k],
    };
    let mut max = 0;
    for bits in 0..1 << k {
        let selected = cd
            .iter()
            .copied()
            .enumerate()
            .map(|(i, (c, d))| if (bits >> i) & 1 == 1 { c } else { d })
            .fold(vec![false; n], |mut selected, j| {
                selected[j] = true;
                selected
            });
        let count = ab
            .iter()
            .copied()
            .filter(|&(a, b)| selected[a] && selected[b])
            .count();
        max = max.max(count);
    }
    let ans = max;
    println!("{}", ans);
}
