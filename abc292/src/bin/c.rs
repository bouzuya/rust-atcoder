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
    d.sort();
    d
}

fn main() {
    input! {
        n: usize,
    };
    let mut ds = vec![vec![]; n + 1];
    for x in 1..=n {
        ds[x] = divisors(x);
    }

    let mut ans = 0_usize;
    for x in 0..=n {
        let y = n - x;
        ans += ds[x].len() * ds[y].len();
    }
    println!("{}", ans);
}
