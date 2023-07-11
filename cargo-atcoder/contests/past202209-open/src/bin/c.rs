use proconio::input;

fn f(ans: &mut f64, selected: &mut Vec<usize>, p: &[Vec<usize>], k: usize, i_p: usize) {
    if selected.len() == p.len() {
        if k == selected.iter().copied().map(|s| s + 1).sum() {
            *ans += selected
                .iter()
                .copied()
                .enumerate()
                .map(|(i, j)| p[i][j])
                .product::<usize>() as f64
                / 1_000_000_f64;
        }
        return;
    }

    for j in 0..6 {
        selected.push(j);
        f(ans, selected, p, k, i_p + 1);
        selected.pop();
    }
}

fn main() {
    input! {
        p: [[usize; 6]; 3]
    };

    for k in 1..=18 {
        let mut ans = 0_f64;
        let mut selected = vec![];
        f(&mut ans, &mut selected, &p, k, 0);
        println!("{}", ans as f64);
    }
}
