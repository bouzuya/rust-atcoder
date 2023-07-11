use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    };
    let mut p = h[0];
    for h_i in h.iter().copied().skip(1) {
        match p.cmp(&(h_i + 1)) {
            std::cmp::Ordering::Greater => {
                println!("No");
                return;
            }
            std::cmp::Ordering::Equal => {
                p = h_i + 1;
            }
            std::cmp::Ordering::Less => {
                p = h_i;
            }
        }
    }
    println!("Yes");
}
