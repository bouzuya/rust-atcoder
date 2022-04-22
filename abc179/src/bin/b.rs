use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [(usize, usize); n],
    };
    for i in 0..n - 2 {
        let mut ok = true;
        for j in 0..3 {
            if d[i + j].0 != d[i + j].1 {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
