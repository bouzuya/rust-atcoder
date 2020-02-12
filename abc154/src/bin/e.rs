use proconio::input;
use proconio::marker::Bytes;

fn c(n: usize, k: usize) -> usize {
    return if k > n {
        0
    } else if k == 1 {
        n
    } else if k == 2 {
        n * (n - 1) / 2
    } else if k == 3 {
        n * (n - 1) * (n - 2) / 6
    } else {
        unreachable!()
    };
}

fn f(s: &Vec<usize>, i: usize, k: usize, l: bool) -> usize {
    if k == 0 {
        return 1;
    }
    if i == s.len() {
        return 0;
    }
    if l {
        return c(s.len() - i, k) * 9usize.pow(k as u32);
    }
    if s[i] == 0 {
        return f(s, i + 1, k, false);
    }
    return f(s, i + 1, k, true) // 0xxxxx
        + f(s, i + 1, k - 1, true) * (s[i] - 1) // (1-(d-1))xxxxx
        + f(s, i + 1, k - 1, false) // dxxxxx
        ;
}

fn main() {
    input! {
        mut s: Bytes,
        nzc: usize
    };
    let bs = s.into_iter().map(|b| (b - b'0') as usize).collect();
    let ans = f(&bs, 0, nzc, false);
    println!("{}", ans);
}
