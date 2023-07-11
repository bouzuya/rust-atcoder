use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let mut t = vec![];
    for s_i in s.iter() {
        let m = s_i.len();
        let mut z = true;
        let mut p = 0;
        for j in 0..m {
            if s_i[j] != '0' {
                z = false;
                p = j;
                break;
            }
        }
        if z {
            p = m;
        }
        t.push((m - p, &s_i[p..m], -(m as i64), s_i));
    }
    t.sort();
    for &(_, _, _, s_i) in t.iter() {
        println!("{}", s_i.iter().collect::<String>());
    }
}
