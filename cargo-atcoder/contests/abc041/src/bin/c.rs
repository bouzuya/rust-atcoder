use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut b = a.into_iter().enumerate().collect::<Vec<(usize, i64)>>();
    b.sort_by_key(|&(_, b_i)| b_i);
    for (i, _) in b.iter().rev() {
        println!("{}", i + 1);
    }
}
