use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let mut ok = 1_f64;
    let mut ng = 1_000_000_001_f64;
    for _ in 0..100 {
        let x = ok + (ng - ok) / 2_f64;
        let mut sum = 0_usize;
        for a_i in a.iter().copied() {
            sum += (a_i as f64 / x).floor() as usize;
        }
        if sum >= k {
            ok = x;
        } else {
            ng = x;
        }
    }

    let mut ans = vec![];
    for a_i in a {
        ans.push((a_i as f64 / ok).floor());
    }

    for a in ans {
        println!("{}", a);
    }
}
