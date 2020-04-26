use proconio::input;

fn is_ok(hss: &Vec<(usize, usize)>, max_h: usize) -> bool {
    let mut ss = vec![];
    for &(h, s) in hss.iter() {
        if h > max_h {
            return false;
        }
        ss.push((max_h - h) / s);
    }
    ss.sort();
    ss.iter().enumerate().all(|(i, &s)| i <= s)
}

fn main() {
    input! {
        n: usize,
        hss: [(usize, usize); n],
    };
    let mut ng = 0_usize;
    let mut ok = 1_000_000_000_usize + 1_000_000_000_usize * (n - 1);
    while ok - ng > 1 {
        let m = (ng + ok) / 2;
        if is_ok(&hss, m) {
            ok = m;
        } else {
            ng = m;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
