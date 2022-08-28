use proconio::input;

fn main() {
    input! {
        abcd: [(i64, i64); 4],
    };
    for i in 0..4 {
        let a = abcd[i];
        let b = abcd[(i + 1) % 4];
        let c = abcd[(i + 2) % 4];
        let ab = (b.0 - a.0, b.1 - a.1);
        let bc = (c.0 - b.0, c.1 - b.1);
        if ab.0 * bc.1 - ab.1 * bc.0 < 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
