use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n],
    };
    let mut ans = 0_usize;
    for bits in 0..1 << n {
        let mut is = vec![];
        for i in 0..n {
            if (bits >> i) & 1 == 1 {
                is.push(i);
            }
        }

        let mut count = 0;
        for j in 0..26 {
            let ch = (b'a' + j) as char;
            let mut c = 0;
            for i in is.iter().copied() {
                if s[i].contains(&ch) {
                    c += 1;
                }
            }
            if c == k {
                count += 1;
            }
        }

        ans = ans.max(count);
    }
    println!("{}", ans);
}
