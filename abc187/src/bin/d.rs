use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(i64, i64); n],
    };
    let mut sum_a = ab.iter().map(|(a_i, _)| a_i).sum::<i64>();

    ab.sort_by_key(|(a_i, b_i)| -(a_i + a_i + b_i));

    let mut sum_b = 0_i64;
    for (i, &(a_i, b_i)) in ab.iter().enumerate() {
        sum_a -= a_i;
        sum_b += a_i + b_i;
        if sum_a < sum_b {
            println!("{}", i + 1);
            return;
        }
    }
}
