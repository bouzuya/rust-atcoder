use proconio::input;

fn pow_mod(x: usize, mut n: usize, m: usize) -> usize {
    if m == 1 {
        return 0;
    }
    let mut r = 1;
    let mut y = x % m;
    while n != 0 {
        if (n & 1) == 1 {
            r = (r * y) % m;
        }
        y = (y * y) % m;
        n >>= 1;
    }
    r
}

fn f(x: usize, n: usize, m: usize) -> usize {
    if n == 1 {
        return x;
    }
    let r = f(x, n / 2, m);
    let r = (r * pow_mod(10, n / 2, m) + r) % m;
    (if (n & 1) == 0 { r } else { r * 10 + x }) % m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(f(1, 1, 100), 1);
        // assert_eq!(f(1, 2, 100), 11);
        // assert_eq!(f(1, 3, 100), 11);
        // assert_eq!(f(1, 4, 100), 11);
    }

    #[test]
    fn pow_mod_test() {
        let g = pow_mod;
        assert_eq!(g(1, 1, 100), 1);
        assert_eq!(g(1, 2, 100), 1);
        assert_eq!(g(1, 3, 100), 1);
        assert_eq!(g(2, 1, 100), 2);
        assert_eq!(g(2, 2, 100), 4);
        assert_eq!(g(2, 3, 100), 8);
        assert_eq!(g(2, 1, 5), 2);
        assert_eq!(g(2, 2, 5), 4);
        assert_eq!(g(2, 3, 5), 3);
    }
}

fn main() {
    input! {
        k: usize,
        m: usize,
        cd: [(usize, usize); k],
    };
    let mut sum = 0_usize;
    for (c, d) in cd {
        sum = (sum * pow_mod(10, d, m) + f(c, d, m)) % m;
    }
    let ans = sum;
    println!("{}", ans);
}
