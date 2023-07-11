use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    let mut ss = vec![true; x];
    let mut ps = vec![];
    for i in 2..x {
        if ss[i] {
            ps.push(i);
            for j in (i..x).step_by(i) {
                ss[j] = false;
            }
        }
    }
    for i in x.. {
        if ps.iter().all(|&p| i % p != 0) {
            println!("{}", i);
            break;
        }
    }
}
