use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    };

    let max_a_i = 5;
    let mut grundy = vec![0; max_a_i + 1];
    for i in 0..=max_a_i {
        let mut transit = vec![false; 3];
        if i >= x {
            transit[grundy[i - x]] = true;
        }
        if i >= y {
            transit[grundy[i - y]] = true;
        }
        grundy[i] = transit.iter().copied().position(|v| !v).unwrap();
    }
    let ans = a
        .iter()
        .copied()
        .fold(0_usize, |acc, a_i| acc ^ grundy[a_i % 5])
        != 0;
    println!("{}", if ans { "First" } else { "Second" });
}
