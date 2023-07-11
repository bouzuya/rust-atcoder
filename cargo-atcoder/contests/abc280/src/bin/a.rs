use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        _w: usize,
        s: [Chars; h],
    };
    let ans = s
        .iter()
        .map(|s_i| s_i.iter().filter(|c| c == &&'#').count())
        .sum::<usize>();
    println!("{}", ans);
}
