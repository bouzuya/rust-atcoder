use proconio::input;

fn c(n: usize, k: usize) -> usize {
    if n == 0 || k == 0 {
        return 1;
    }
    let mut x = 1_usize;
    for i in 0..k {
        x *= n - i;
        x /= i + 1;
    }
    x
}

fn f(s: &mut Vec<char>, a: usize, b: usize, k: usize, sum: usize) {
    // eprintln!("(a, b, k, s) => ({}, {}, {}, {}, {:?})", a, b, k, sum, s);
    match (a, b) {
        (0, 0) => {}
        (0, _) => {
            s.push('b');
            f(s, a, b - 1, k, 0);
        }
        (_, 0) => {
            s.push('a');
            f(s, a - 1, b, k, 0);
        }
        (1, _) => {
            if k <= sum + 1 {
                s.push('a');
                f(s, a - 1, b, k, 0);
            } else {
                s.push('b');
                f(s, a, b - 1, k, sum + 1);
            }
        }
        (_, _) => {
            if k <= sum + c(a - 1 + b, b) {
                s.push('a');
                f(s, a - 1, b, k, sum);
            } else {
                s.push('b');
                f(s, a, b - 1, k, sum + c(a - 1 + b, b));
            }
        }
    }
}

fn solve(a: usize, b: usize, k: usize) -> String {
    let mut s = vec![];
    f(&mut s, a, b, k, 0);
    s.iter().collect::<String>()
}

#[test]
fn test() {
    assert_eq!(solve(2, 2, 1), "aabb".to_string());
    assert_eq!(solve(2, 2, 2), "abab".to_string());
    assert_eq!(solve(2, 2, 3), "abba".to_string());
    assert_eq!(solve(2, 2, 4), "baab".to_string());
    assert_eq!(solve(2, 2, 5), "baba".to_string());
    assert_eq!(solve(2, 2, 6), "bbaa".to_string());
}

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    };
    let ans = solve(a, b, k);
    println!("{}", ans);
}
