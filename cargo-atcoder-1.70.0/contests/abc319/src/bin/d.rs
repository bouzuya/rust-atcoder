use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        l: [usize; n],
    };
    let l = l.into_iter().map(|l_i| l_i + 1).collect::<Vec<usize>>();
    let mut ok = l.iter().copied().sum::<usize>();
    let mut ng = l.iter().copied().max().unwrap() - 1;
    while ok - ng > 1 {
        let mid = ng + (ok - ng) / 2;
        let mut rows = 1_usize;
        let mut cols = 0_usize;
        for l_i in l.iter().copied() {
            cols += l_i;
            if cols > mid {
                rows += 1;
                cols = l_i;
            }
        }
        if rows <= m {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok - 1;
    println!("{}", ans);
}
