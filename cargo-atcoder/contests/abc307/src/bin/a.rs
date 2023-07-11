use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 7 * n],
    };

    let mut sum = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        if i != 0 && i % 7 == 0 {
            println!("{}", sum);
            sum = 0_usize;
        }
        sum += a_i;
    }
    println!("{}", sum);
}
