use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
        t: [Chars; h],
    };
    let f = |a: &[char]| -> (usize, usize) {
        a.iter().fold((0, 0), |(c1, c2), c| match c {
            '#' => (c1 + 1, c2),
            '.' => (c1, c2 + 1),
            _ => unreachable!(),
        })
    };
    let ans = s
        .into_iter()
        .zip(t.into_iter())
        .all(|(s_i, t_i)| f(&s_i) == f(&t_i));
    println!("{}", if ans { "Yes" } else { "No" });
}
