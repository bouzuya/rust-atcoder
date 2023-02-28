use proconio::input;

fn main() {
    input! {
        x: i64,
        n: usize,
        p: [i64; n],
    };

    if !p.contains(&x) {
        println!("{}", x);
        return;
    }

    for d in 1.. {
        for y in &[(x - d), (x + d)] {
            if !p.contains(&y) {
                println!("{}", y);
                return;
            }
        }
    }
}
