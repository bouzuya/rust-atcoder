use proconio::input;

fn is_ok(n: usize, max_len: i64, s: &[i64], mid: i64) -> bool {
    let mut l = 0_usize;
    let mut r = 0_usize;
    let mut c = vec![0; n + 1];
    let mut d = vec![0; n + 1];
    d[0] += 1;
    d[1] -= 1;
    for i in 0..n {
        if i > 0 {
            c[i] = c[i - 1] + d[i];
        } else {
            c[i] = d[i];
        }
        if c[i] == 0 {
            continue;
        }
        while l <= n && s[l] - s[i] < mid {
            l += 1;
        }
        if l > n {
            continue;
        }

        while r + 1 <= n && s[r + 1] - s[i] <= max_len {
            r += 1;
        }
        d[l] += 1;
        if r + 1 <= n {
            d[r + 1] -= 1;
        }
    }
    c[n] = c[n - 1] + d[n];
    c[n] > 0
}

fn main() {
    input! {
        n: usize,
        l: i64,
        a: [i64; n],
    }

    let s = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<i64>>();

    let mut ok = 0_i64;
    let mut ng = l + 1;
    while ng - ok > 1 {
        let mid = ok + (ng - ok) / 2;
        if is_ok(n, l, &s, mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
