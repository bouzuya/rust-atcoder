use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut b = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &a_i| {
            *acc += a_i;
            *acc %= 360;
            Some(*acc)
        }))
        .chain(std::iter::once(360))
        .collect::<Vec<usize>>();
    b.sort();
    let mut max = 0;
    let mut p = 0;
    for b_i in b {
        max = max.max(b_i - p);
        p = b_i;
    }
    println!("{}", max);
}
