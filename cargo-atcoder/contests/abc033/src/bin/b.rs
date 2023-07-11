use proconio::input;

fn main() {
    input! {
        n: usize,
        sp: [(String, i64); n],
    };
    let sum = sp.iter().map(|&(_, p_i)| p_i).sum::<i64>();
    for (s_i, p_i) in sp {
        if p_i > sum / 2 {
            println!("{}", s_i);
            return;
        }
    }
    println!("atcoder");
}
