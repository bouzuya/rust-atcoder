use proconio::input;

fn main() {
    input! {
        n: usize,
        csf: [(usize, usize, usize); n - 1],
    };
    let mut ans = vec![0_usize; n];
    for i in 0..n - 1 {
        let (c, s, _) = csf[i];
        let mut x = c + s;
        for j in i + 1..n - 1 {
            let (c, s, f) = csf[j];
            if x <= s {
                x = s;
            } else if (x - s) % f != 0 {
                x += f - (x - s) % f;
            }
            x += c;
        }
        ans[i] = x;
    }

    for a in ans {
        println!("{}", a);
    }
}
