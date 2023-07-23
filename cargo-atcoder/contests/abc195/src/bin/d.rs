use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        wv: [(usize, usize); n],
        x: [usize; m],
        lr: [(Usize1, Usize1); q],
    };
    for (l, r) in lr {
        let mut x = x
            .iter()
            .copied()
            .enumerate()
            .filter(|(i, _)| !(l..=r).contains(&i))
            .map(|(_, x_i)| x_i)
            .collect::<Vec<usize>>();
        x.sort();
        let mut sum = 0_usize;
        let mut used = vec![false; n];
        for x_i in x.iter().copied() {
            let mut max = None;
            for (j, (w_j, v_j)) in wv.iter().copied().enumerate() {
                if !used[j] && w_j <= x_i && (v_j > max.map(|(_, v)| v).unwrap_or_default()) {
                    max = Some((j, v_j));
                }
            }
            if let Some((j, v_j)) = max {
                used[j] = true;
                sum += v_j;
            }
        }
        println!("{}", sum);
    }
}
