use proconio::{
    input,
    marker::{Chars, Usize1},
};

// fn f(s: &[u8], t: usize, k: usize) -> u8 {
//     if t == 0 {
//         return s[k];
//     }
//     (f(s, t - 1, k / 2) + if k % 2 == 0 { 1 } else { 2 }) % 3
// }
fn f(s: &[u8], t: usize, k: usize) -> u8 {
    if t == 0 {
        return s[k];
    }

    if t > 60 {
        let mut x = 0;
        let mut c = k;
        while c != 0 {
            if c % 2 != 0 {
                x += 1;
            }
            c /= 2;
            x %= 3;
        }
        (f(s, 0, 0) + (t % 3) as u8 + x) % 3
    } else {
        (f(s, t - 1, k / 2) + if k % 2 == 0 { 1 } else { 2 }) % 3
    }
}

fn main() {
    input! {
        s: Chars,
        q: usize,
        tk: [(usize, Usize1); q],
    };
    let u = s
        .iter()
        .copied()
        .map(|c| c as u8 - b'A')
        .collect::<Vec<u8>>();
    for (t, k) in tk {
        let ans = f(&u, t, k);
        println!("{}", (ans as u8 + b'A') as char)
    }
}
