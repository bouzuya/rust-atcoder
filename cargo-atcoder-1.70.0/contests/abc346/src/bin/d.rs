use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        c: [usize; n],
    };

    // 1010...
    let mut c1 = vec![0_usize; n + 1];
    for (i, s_i) in s.iter().copied().enumerate() {
        let expected = if i % 2 == 0 { '1' } else { '0' };
        c1[i + 1] = c1[i] + if s_i != expected { c[i] } else { 0 };
    }

    // 0101...
    let mut c2 = vec![0_usize; n + 1];
    for (i, s_i) in s.iter().copied().enumerate() {
        let expected = if i % 2 == 0 { '0' } else { '1' };
        c2[i + 1] = c2[i] + if s_i != expected { c[i] } else { 0 };
    }

    // println!("{:?}", c1);
    // println!("{:?}", c2);

    let mut min = 1 << 60;
    for i in 0..n - 1 {
        min = min.min((c1[i + 1] + c2[n] - c2[i + 1]).min(c2[i + 1] + c1[n] - c1[i + 1]));
    }
    println!("{}", min);
}
