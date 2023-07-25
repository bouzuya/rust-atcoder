use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    };
    let ans = a
        .iter()
        .copied()
        .filter(|a_i| a_i != &x)
        .collect::<Vec<usize>>();
    let mut s = String::new();
    for (i, a_i) in ans.iter().copied().enumerate() {
        s.push_str(&format!(
            "{}{}",
            a_i.to_string(),
            if i == ans.len() - 1 { '\n' } else { ' ' }
        ));
    }
    print!("{}", s);
}
