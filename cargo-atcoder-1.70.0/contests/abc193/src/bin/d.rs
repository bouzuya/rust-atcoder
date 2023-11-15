use proconio::{input, marker::Chars};

fn f(s: &[usize], x: usize) -> usize {
    let mut c = vec![0_usize; 10];
    for s_i in s.iter().copied() {
        c[s_i] += 1;
    }
    c[x] += 1;
    let mut sum = 0_usize;
    for i in 1..=9 {
        sum += i * 10_usize.pow(c[i] as u32);
    }
    sum
}

fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    };

    let s = s
        .into_iter()
        .take(4)
        .map(|c| (c as u8 - b'0') as usize)
        .collect::<Vec<usize>>();
    let t = t
        .into_iter()
        .take(4)
        .map(|c| (c as u8 - b'0') as usize)
        .collect::<Vec<usize>>();
    let mut c = vec![k; 10];
    c[0] = 0;
    for s_i in s.iter().copied() {
        c[s_i] -= 1;
    }
    for t_i in t.iter().copied() {
        c[t_i] -= 1;
    }

    let mut win = 0_usize;
    for i in 1..=9 {
        for j in 1..=9 {
            if f(&s, i) > f(&t, j) {
                win += c[i] * (c[j].saturating_sub(if i == j { 1 } else { 0 }));
            }
        }
    }
    let ans = win as f64 / ((9 * k - 8) * (9 * k - 9)) as f64;
    println!("{}", ans);
}
