use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut ok_index = None;
    for i in (0..s.len()).rev() {
        if (0..t.len())
            .rev()
            .all(|j| i + j < s.len() && (s[i + j] == '?' || t[j] == s[i + j]))
        {
            ok_index = Some(i);
            break;
        }
    }
    match ok_index {
        None => println!("UNRESTORABLE"),
        Some(i) => {
            let mut s2 = s
                .iter()
                .map(|&c| if c == '?' { 'a' } else { c })
                .collect::<Vec<char>>();
            let mut j = i;
            for &c in t.iter() {
                s2[j] = c;
                j += 1;
            }
            println!("{}", s2.iter().collect::<String>());
        }
    }
}
