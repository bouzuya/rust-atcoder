use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        n: usize,
        a: Usize1,
        k: Chars,
        b: [Usize1; n],
    };
    let kn = if k.len() <= 8 {
        k.iter().collect::<String>().parse::<usize>().unwrap()
    } else {
        let (o, l) = {
            let mut c = vec![0; n];
            let mut i_b = a;
            let mut s = 0;
            loop {
                if c[i_b] != 0 {
                    let o = c[i_b] - 1;
                    break (o, s - o);
                }
                s += 1;
                c[i_b] = s;
                i_b = b[i_b];
            }
        };
        let x = {
            let mut c = 0_i64;
            for &k_i in k.iter() {
                let d = (k_i as usize - '0' as usize) as i64;
                c = (c * 10 + d) % l;
            }
            c
        };
        let mut m = x;
        while m < o {
            m += l;
        }
        m as usize
    };
    let mut i_b = a;
    for _ in 0..kn {
        i_b = b[i_b];
    }
    println!("{}", i_b + 1);
}
