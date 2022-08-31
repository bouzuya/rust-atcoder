use proconio::input;

fn divisors(n: usize) -> Vec<usize> {
    let mut d = vec![];
    for i in 1.. {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            d.push(i);
            if i != n / i {
                d.push(n / i);
            }
        }
    }
    // d.sort();
    d
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut b = vec![];
    let mut c = vec![0_usize; n + 1];
    for (i, a_i) in a.iter().copied().enumerate().rev() {
        let i = i + 1;
        if (a_i + c[i]) % 2 == 1 {
            for j in divisors(i) {
                c[j] += 1;
            }
            b.push(i);
        }
    }

    println!("{}", b.len());
    for b_i in b {
        println!("{}", b_i);
    }
}
