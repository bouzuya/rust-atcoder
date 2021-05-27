use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        vp: [(usize, usize); n],
    };
    let x = x * 100;
    let mut sum = 0_usize;
    for (i, &(v_i, p_i)) in vp.iter().enumerate() {
        sum += v_i * p_i;
        if sum > x {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
