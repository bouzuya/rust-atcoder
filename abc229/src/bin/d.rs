use proconio::{input, marker::Chars};

fn f(s: &[char], k: usize) -> usize {
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
    ok
}

fn g(s: &[char], k: usize) -> usize {
    let n = s.len();
    let a = s
        .iter()
        .map(|&s_i| if s_i == '.' { 1 } else { 0 })
        .collect::<Vec<usize>>();
    let mut max = 0_usize;
    let mut sum = 0_usize;
    let mut r = 0;
    for l in 0..n {
        while r < n && sum + a[r] <= k {
            sum += a[r];
            r += 1;
        }
        max = max.max(r - l);
        if r == l {
            r += 1;
        } else {
            sum -= a[l];
        }
    }
    max
}

fn main() {
    input! {
        s: Chars,
        k: usize,
    };
    let ans = g(&s, k);
    println!("{}", ans);
}
