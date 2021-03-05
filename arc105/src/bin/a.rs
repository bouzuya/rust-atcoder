use proconio::input;

fn main() {
    input! {
        abcd: [i64; 4],
    };
    for bits in 0..1 << 4 {
        let is = (0..4)
            .map(|i| ((bits >> i) & 1) == 1)
            .collect::<Vec<bool>>();
        let sum = is
            .iter()
            .enumerate()
            .map(|(i, &ate)| if ate { abcd[i] } else { -abcd[i] })
            .sum::<i64>();
        if sum == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
