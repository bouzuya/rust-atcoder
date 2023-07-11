use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    let mut m = n;
    loop {
        m = m.saturating_sub(a);
        if m == 0 {
            println!("Ant");
            return;
        }
        m = m.saturating_sub(b);
        if m == 0 {
            println!("Bug");
            return;
        }
    }
}
