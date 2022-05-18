use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let mut bits = vec![];
    for i in 0..50 {
        let cycle_half = 1_usize << i;
        let cycle = cycle_half << 1;
        let f = |x: usize| -> usize {
            let p = (x + 1) / cycle;
            let r = (x + 1) % cycle;
            p * cycle_half + r.saturating_sub(cycle_half)
        };
        bits.push((f(b) - f(a.saturating_sub(1))) % 2);
    }
    bits.reverse();
    let mut ans = 0_usize;
    for b in bits {
        ans <<= 1;
        ans += b;
    }
    println!("{}", ans);
}
