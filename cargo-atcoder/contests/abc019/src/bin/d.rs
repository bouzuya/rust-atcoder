use std::cmp::max;
use std::io::stdin;

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    let n = buf.trim().parse::<usize>().unwrap();
    let mut max_d1 = (0, 0);
    for i in 1..n {
        println!("? {} {}", 0 + 1, i + 1);
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let d_i = buf.trim().parse::<usize>().unwrap();
        if d_i > max_d1.0 {
            max_d1 = (d_i, i);
        }
    }
    let mut max_d2 = (0, 0);
    for i in 1..n {
        if i == max_d1.1 {
            continue;
        }
        println!("? {} {}", max_d1.1 + 1, i + 1);
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let d_i = buf.trim().parse::<usize>().unwrap();
        if d_i > max_d2.0 {
            max_d2 = (d_i, i);
        }
    }
    let ans = max(max_d1.0, max_d2.0);
    println!("! {}", ans);
}
