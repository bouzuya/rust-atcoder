use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut ans = 0_usize;
    let n = s.len();
    for bits in 0..1 << (n - 1) {
        let mut ts = vec![];
        let mut t = String::new();
        for i in 0..n - 1 {
            t.push(s[i]);
            if (bits >> i) & 1 == 1 {
                ts.push(t);
                t = String::new();
            }
        }
        t.push(s[n - 1]);
        ts.push(t);

        let sum = ts
            .into_iter()
            .map(|t| t.parse::<usize>().unwrap())
            .sum::<usize>();
        ans += sum;
    }
    println!("{}", ans);
}
