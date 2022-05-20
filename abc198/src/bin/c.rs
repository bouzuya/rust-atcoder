use proconio::input;

fn main() {
    input! {
        r: usize,
        x: usize,
        y: usize,
    };
    let d2 = x * x + y * y;
    for i in 1.. {
        let r2 = (i * r).pow(2);
        if r2 == d2 {
            println!("{}", i);
            break;
        } else if r2 > d2 {
            if i == 1 {
                println!("{}", 2);
            } else {
                println!("{}", i);
            }
            break;
        }
    }
}
