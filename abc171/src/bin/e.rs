use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    };
    let mut s = 0;
    for i in 0..32 {
        let b = 1 << i;
        let mut x = 0;
        for &a_i in a.iter() {
            x ^= a_i & b;
        }
        s |= x;
    }
    for &a_i in a.iter() {
        print!("{} ", s ^ a_i);
    }
}
