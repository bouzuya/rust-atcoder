use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut d = vec![];
    for i in 1.. {
        if i * i > n {
            break;
        }

        if n % i == 0 {
            let a = n / i;
            let b = n / a;
            d.push(a);
            if a != b {
                d.push(b);
            }
        }
    }
    d.sort();
    for d_i in d {
        println!("{}", d_i);
    }
}
