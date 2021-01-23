use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        vp: [(i64, i64); n],
    };
    let mut sum = 0_i64;
    for (i, &(v_i, p_i)) in vp.iter().enumerate() {
        sum += v_i * p_i;
        if sum > x * 100 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
