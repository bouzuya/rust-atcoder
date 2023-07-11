use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        e: [[usize; a]; c],
    };
    for x in (0..=100).rev() {
        let count = e
            .iter()
            .filter(|e_i| e_i.iter().copied().filter(|e_ij| *e_ij >= x).count() >= b)
            .count();
        if count >= d {
            println!("{}", x);
            break;
        }
    }
}
