use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    if n % 2 == 0 || s[n / 2] != 'b' {
        println!("-1");
        return;
    }
    let pv = vec![('a', 'c'), ('c', 'a'), ('b', 'b')];
    for i in 0..n / 2 {
        let i_l = n / 2 - 1 - i;
        let i_r = n / 2 + 1 + i;
        if (s[i_l], s[i_r]) != pv[i % 3] {
            println!("-1");
            return;
        }
    }
    let ans = n / 2;
    println!("{}", ans);
}
