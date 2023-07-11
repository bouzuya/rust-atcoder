use proconio::input;

fn f(n: usize) -> (usize, usize) {
    for x in 1.. {
        let sum = x * (x + 1) / 2;
        if sum == n {
            return (1, x);
        } else if sum > n {
            let offset = n - (x - 1) * x / 2 - 1;
            return (x - offset, 1 + offset);
        }
    }
    unreachable!()
}

fn g((a, b): (usize, usize)) -> usize {
    let s = a + b;
    if s == 2 {
        return 1;
    }
    let n = s - 2;
    let sum = n * (n + 1) / 2;
    sum + b
}

fn main() {
    input! {
        i: usize,
        j: usize,
    };
    let (a_i, b_i) = f(i);
    let (a_j, b_j) = f(j);
    let x = (a_i + a_j, b_i + b_j);
    let ans = g(x);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_test() {
        assert_eq!(f(1), (1, 1));
        assert_eq!(f(2), (2, 1));
        assert_eq!(f(3), (1, 2));
        assert_eq!(f(4), (3, 1));
        assert_eq!(f(5), (2, 2));
    }

    #[test]
    fn g_test() {
        assert_eq!(g((1, 1)), 1);
        assert_eq!(g((2, 1)), 2);
        assert_eq!(g((1, 2)), 3);
        assert_eq!(g((3, 1)), 4);
        assert_eq!(g((2, 2)), 5);
    }
}
