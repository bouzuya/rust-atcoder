use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        k: usize,
    };

    let a = s
        .iter()
        .map(|&s_i| if s_i == '.' { 1 } else { 0 })
        .collect::<Vec<usize>>();
    let c = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();

    let n = s.len();
    let mut ok = 0_usize;
    let mut ng = n + 1;
    while ng - ok > 1 {
        let x = (ok + ng) / 2;
        let mut is_ok = false;
        for i in 0..n {
            if i + x <= n {
                if c[i + x] - c[i] <= k {
                    is_ok = true;
                }
            }
        }
        if is_ok {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
