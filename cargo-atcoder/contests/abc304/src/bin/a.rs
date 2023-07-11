use proconio::input;

fn main() {
    input! {
        n: usize,
        sa: [(String, usize); n],
    };

    let min_a = sa.iter().map(|(_, a)| a).min().unwrap();
    let offset = sa.iter().position(|(_, a)| a == min_a).unwrap();
    for i in 0..n {
        let (s, _) = &sa[(offset + i) % n];
        println!("{}", s);
    }
}
