use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut min = (1_usize << 30) - 1;
    for bits in 0_usize..1 << (n - 1) {
        let mut parts = vec![vec![a[0]]];
        for i in 0..n - 1 {
            if ((bits >> i) & 1) == 1 {
                parts.push(vec![a[i + 1]]);
            } else {
                parts.last_mut().unwrap().push(a[i + 1]);
            }
        }

        min = min.min(
            parts
                .iter()
                .map(|part| part.iter().fold(0, |acc, x| acc | x))
                .fold(0, |acc, x| acc ^ x),
        );
    }

    let ans = min;
    println!("{}", ans);
}
