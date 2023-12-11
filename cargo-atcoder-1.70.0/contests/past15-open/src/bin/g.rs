use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut abs = vec![];
    for _ in 0..m {
        input! {
            k: usize,
            ab: [(Usize1, usize); k],
        }
        abs.push(ab);
    }

    for bits in 0..1 << n {
        let x = (0..n).map(|i| (bits >> i) & 1).collect::<Vec<usize>>();
        if abs
            .iter()
            .all(|ab| ab.iter().copied().any(|(a, b)| x[a] == b))
        {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
