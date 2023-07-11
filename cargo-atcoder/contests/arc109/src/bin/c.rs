use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };
    let mut t = s.clone();
    for _ in 0..k {
        t = (0..n)
            .map(|i| match (t[(i * 2) % n], t[(i * 2 + 1) % n]) {
                ('R', 'R') => 'R',
                ('R', 'S') => 'R',
                ('R', 'P') => 'P',
                ('S', 'R') => 'R',
                ('S', 'S') => 'S',
                ('S', 'P') => 'S',
                ('P', 'R') => 'P',
                ('P', 'S') => 'S',
                ('P', 'P') => 'P',
                _ => unreachable!(),
            })
            .collect();
    }
    println!("{}", t[0]);
}
