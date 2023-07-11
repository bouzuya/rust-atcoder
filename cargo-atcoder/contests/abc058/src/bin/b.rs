use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        o: Chars,
        e: Chars,
    };
    let mut s = vec![];
    let mut i = 0;
    let mut i_o = 0;
    let mut i_e = 0;
    while i_o < o.len() || i_e < e.len() {
        if i % 2 == 0 {
            if i_o < o.len() {
                s.push(o[i_o]);
            }
            i_o += 1;
        } else {
            if i_e < e.len() {
                s.push(e[i_e]);
            }
            i_e += 1;
        }
        i += 1;
    }
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
