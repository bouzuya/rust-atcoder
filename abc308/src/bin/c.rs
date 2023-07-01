use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };
    let mut abi = ab
        .iter()
        .copied()
        .enumerate()
        .map(|(i, (a, b))| (a, b, i))
        .collect::<Vec<_>>();
    abi.sort_by(
        |(a1, b1, i1), (a2, b2, i2)| match (a1 * (a2 + b2)).cmp(&(a2 * (a1 + b1))) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => i1.cmp(&i2),
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
        },
    );

    for (_, _, i) in abi {
        println!("{}", i + 1);
    }
}
