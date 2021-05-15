use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut b = a.clone();
    b.sort();
    let max_1 = b[b.len() - 1];
    let max_2 = b[b.len() - 2];
    for a_i in a {
        let max = if a_i == max_1 { max_2 } else { max_1 };
        println!("{}", max);
    }
}
