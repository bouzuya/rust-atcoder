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
        k: usize,
        a: [usize; n],
    };
    let sum_a = a.iter().sum::<usize>();
    let ds = divisors(sum_a);
    let mut ans = 0;
    for g in ds {
        let mut r = a.iter().copied().map(|a| a % g).collect::<Vec<usize>>();
        r.sort();
        let sum_r = r.iter().sum::<usize>();
        let count = r[0..(n - sum_r / g)].iter().sum::<usize>();
        if count <= k {
            ans = ans.max(g);
        }
    }
    println!("{}", ans);
}
