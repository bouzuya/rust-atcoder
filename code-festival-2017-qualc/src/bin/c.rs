use proconio::input;
use proconio::marker::Chars;

fn f(s: &[char]) -> i64 {
    if s.len() == 1 {
        return 0;
    }
    if (0..s.len() / 2).all(|i| s[i] == s[s.len() - i - 1]) {
        return 0;
    }
    let nx = s
        .iter()
        .filter(|&&c| c != 'x')
        .map(|&c| c)
        .collect::<Vec<char>>();
    let is_ok = {
        let mut nxr = nx.clone();
        nxr.reverse();
        nx == nxr
    };
    if !is_ok {
        return -1;
    }
    if nx.is_empty() {
        return 0;
    }
    let mut i_s = 0;
    let mut c = vec![0_i64; nx.len() + 1];
    for (i, &nx_i) in nx.iter().enumerate() {
        while s[i_s] != nx_i {
            if s[i_s] == 'x' {
                c[i] += 1;
            }
            i_s += 1;
        }
        i_s += 1;
    }
    while i_s < s.len() {
        if s[i_s] == 'x' {
            c[nx.len()] += 1;
        }
        i_s += 1;
    }
    let mut ans = 0_i64;
    for i in 0..c.len() / 2 {
        let c_l = c[i];
        let c_r = c[c.len() - i - 1];
        if c_l != c_r {
            ans += (c_l - c_r).abs();
        }
    }
    ans
}

fn main() {
    input! {
        s: Chars,
    };
    let ans = f(&s);
    println!("{}", ans);
}
