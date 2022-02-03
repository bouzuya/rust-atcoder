use proconio::input;

fn main() {
    input! {
        mut n: usize,
    };
    let mut p = vec![];
    for i in 2.. {
        if i * i > n {
            break;
        }

        if n % i == 0 {
            n /= i;
            p.push(i);

            while n % i == 0 {
                n /= i;
                p.push(i);
            }
        }
    }
    if n != 1 {
        p.push(n);
    }
    for p_i in p {
        println!("{}", p_i);
    }
}
