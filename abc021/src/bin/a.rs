use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut m = n;
    let mut xs = vec![];
    while m > 0 {
        if m % 2 == 0 {
            m -= 2;
            xs.push(2);
        } else {
            m -= 1;
            xs.push(1);
        }
    }
    xs.sort();
    println!("{}", xs.len());
    for x_i in xs {
        println!("{}", x_i);
    }
}
