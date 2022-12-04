use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [i64; n],
    };
    let mut a = vec![];
    for (i, s_i) in s.iter().copied().enumerate() {
        a.push(if i == 0 { s_i } else { s_i - s[i - 1] });
    }
    let mut s = String::new();
    for (i, a_i) in a.iter().copied().enumerate() {
        s.push_str(&format!(
            "{}{}",
            a_i.to_string(),
            if i == a.len() - 1 { '\n' } else { ' ' }
        ));
    }
    print!("{}", s);
}
