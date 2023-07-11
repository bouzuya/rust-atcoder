use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let q = n / 100;
    let r = n % 100;
    for a in 0..=20 {
        for b in 0..=20 {
            for c in 0..=20 {
                for d in 0..=20 {
                    for e in 0..=20 {
                        let x = vec![a, b, c, d, e]
                            .iter()
                            .copied()
                            .enumerate()
                            .map(|(i, a_i)| (i + 1) * a_i)
                            .sum::<usize>();
                        if x == r && q >= a + b + c + d + e {
                            println!("1");
                            return;
                        }
                    }
                }
            }
        }
    }
    println!("0");
}
