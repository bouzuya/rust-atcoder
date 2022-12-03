use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
    };
    let mut cur = s[0];
    let mut a = vec![s[0]];
    for s_i in s.iter().copied().skip(1) {
        let d = s_i - cur;
        a.push(d);
        cur = s_i;
    }
    for ans in a {
        println!("{}", ans);
    }
}
