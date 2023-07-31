use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut min = 753;
    let n = s.len();
    for i in 0..n - 3 + 1 {
        let mut t = String::new();
        t.push(s[i]);
        t.push(s[i + 1]);
        t.push(s[i + 2]);
        let x = t.parse::<i64>().unwrap();
        let v = (x - 753).abs();
        min = min.min(v);
    }
    let ans = min;
    println!("{}", ans);
}
