use proconio::input;

fn main() {
    input! {
        x: i64,
        n: usize,
        p: [i64; n],
    };
    let mut c = vec![true; 101 + 1];
    for &p_i in p.iter() {
        c[p_i as usize] = false;
    }
    let mut min_abs = (x + 1).abs();
    let mut ans = -1_i64;
    for i in 0..=101 {
        if c[i] {
            if (x - i as i64).abs() < min_abs {
                min_abs = (x - i as i64).abs();
                ans = i as i64;
            }
        }
    }
    println!("{}", ans);
}
