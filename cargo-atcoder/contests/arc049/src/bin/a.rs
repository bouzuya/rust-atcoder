use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        abcd: [usize; 4]
    };
    let mut t = vec![];
    for i in 0..s.len() {
        if abcd.contains(&i) {
            t.push('"');
        }
        t.push(s[i]);
    }
    if abcd.contains(&s.len()) {
        t.push('"');
    }
    let ans = t.iter().collect::<String>();
    println!("{}", ans);
}
