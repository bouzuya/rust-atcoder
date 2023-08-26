use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        x: usize,
        p: [usize; n],
    };

    for (i, p_i) in p.iter().copied().enumerate() {
        if h + p_i >= x {
            println!("{}", i + 1);
            return;
        }
    }
}
