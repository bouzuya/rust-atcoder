use proconio::input;

fn main() {
    input! {
        n: usize,
        h: u64,
        mut ab: [(u64, u64); n],
    };

    ab.sort_by_key(|&(_, b_i)| std::cmp::Reverse(b_i));
    let max_a = ab.iter().map(|&(a_i, _)| a_i).max().unwrap();
    let mut cnt = 0;
    let mut sum = 0;
    for &(_, b_i) in ab.iter() {
        if b_i <= max_a || sum >= h {
            break;
        }
        sum += b_i;
        cnt += 1;
    }
    if sum >= h {
        println!("{}", cnt);
        return;
    }
    println!("{}", cnt + ((h - sum) + (max_a - 1)) / max_a);
}
