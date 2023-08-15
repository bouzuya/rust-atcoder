use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };

    let x_0 = a
        .iter()
        .copied()
        .enumerate()
        .map(|(i, a_i)| if i % 2 == 0 { 1 } else { -1 } * a_i)
        .sum::<i64>();
    let mut ans = vec![x_0];
    let mut cur = x_0;
    for a_i in a.iter().copied().take(n - 1) {
        cur = (a_i - cur / 2) * 2;
        ans.push(cur);
    }

    let mut s = String::new();
    for (i, a_i) in ans.iter().copied().enumerate() {
        s.push_str(&format!(
            "{}{}",
            a_i,
            if i == ans.len() - 1 { '\n' } else { ' ' }
        ));
    }
    print!("{}", s);
}
