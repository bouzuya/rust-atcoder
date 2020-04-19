use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut ksv: Vec<(usize, Vec<usize>)> = Vec::new();
    for _ in 0..m {
        input! {
            k: usize,
            sv: [Usize1; k],
        };
        ksv.push((k, sv));
    }
    input! { pv: [usize; m] };

    let ans = (0..1_usize << n)
        .filter(|bits| {
            let ssv: Vec<bool> = (0..n).map(|i| ((bits >> i) & 1) == 1).collect();
            ksv.iter()
                .enumerate()
                .all(|(i, (_, sv))| sv.iter().filter(|&&i| ssv[i]).count() % 2 == pv[i])
        })
        .count();
    println!("{}", ans);
}
