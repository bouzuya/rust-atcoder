use proconio::input;

fn main() {
    input! {
        n: usize,
        at: [(i64, usize); n],
        q: usize,
        x: [i64; q],
    };
    let (mut low, mut high, mut add) = (-1 << 62, 1 << 62, 0);
    for (a, t) in at {
        match t {
            1 => {
                low += a;
                high += a;
                add += a;
            }
            2 => {
                low = low.max(a);
                high = high.max(a);
            }
            3 => {
                low = low.min(a);
                high = high.min(a);
            }
            _ => unreachable!(),
        }
    }
    for x_i in x {
        println!("{}", high.min(low.max(x_i + add)));
    }
}
