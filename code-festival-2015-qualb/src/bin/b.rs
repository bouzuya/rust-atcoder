use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };
    let mut counts = vec![0; m + 1];
    for a_i in a {
        counts[a_i] += 1;
    }
    for (i, &c) in counts.iter().enumerate() {
        if c * 2 > n {
            println!("{}", i);
            return;
        }
    }
    println!("?");
}
