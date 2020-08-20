use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };
    let mut map = std::collections::BTreeMap::new();
    for &s_i in s.iter() {
        *map.entry(s_i).or_insert(0) += 1;
    }
    let mut c_o = 0;
    for (_, v) in map.iter() {
        if v % 2 != 0 {
            c_o += 1;
        }
    }
    let ans = if c_o > 0 {
        let g = (s.len() - c_o) / c_o;
        if g % 2 == 0 {
            g + 1
        } else {
            g / 2 * 2 + 1
        }
    } else {
        s.len()
    };
    println!("{}", ans);
}
