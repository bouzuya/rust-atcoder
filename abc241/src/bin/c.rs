// WA
use proconio::{input, marker::Chars};

fn f(n: usize, s: &[Vec<char>]) -> bool {
    for x in 0..n {
        let y = 0;

        let mut count = 0;
        let mut r = 0;
        for l in 0..n {
            while y + r < n && x + r < n && (r - l < 6) {
                if s[y + r][x + r] == '#' {
                    count += 1;
                }
                r += 1;
            }
            if count >= 4 {
                return true;
            }
            if r == l {
                r += 1;
            } else {
                if s[y + l][x + l] == '#' {
                    count -= 1;
                }
            }
        }
    }

    for y in 0..n {
        let x = 0;

        let mut count = 0;
        let mut r = 0;
        for l in 0..n {
            while y + r < n && x + r < n && (r - l < 6) {
                if s[y + r][x + r] == '#' {
                    count += 1;
                }
                r += 1;
            }
            if count >= 4 {
                return true;
            }
            if r == l {
                r += 1;
            } else {
                if s[y + l][x + l] == '#' {
                    count -= 1;
                }
            }
        }
    }

    false
}

fn g(n: usize, s: &[Vec<char>]) -> bool {
    for i in 0..n {
        let mut count = 0;
        let mut r = 0;
        for l in 0..n {
            while r < n && (r - l < 6) {
                if s[i][r] == '#' {
                    count += 1;
                }
                r += 1;
            }
            if count >= 4 {
                return true;
            }
            if r == l {
                r += 1;
            } else {
                if s[i][l] == '#' {
                    count -= 1;
                }
            }
        }
    }

    for i in 0..n {
        let mut count = 0;
        let mut r = 0;
        for l in 0..n {
            while r < n && (r - l < 6) {
                if s[r][i] == '#' {
                    count += 1;
                }
                r += 1;
            }
            if count >= 4 {
                return true;
            }
            if r == l {
                r += 1;
            } else {
                if s[l][i] == '#' {
                    count -= 1;
                }
            }
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    };

    for _ in 0..4 {
        if f(n, &s) || g(n, &s) {
            println!("Yes");
            return;
        }

        let mut t = vec![vec![' '; n]; n];
        for i in 0..n {
            for j in 0..n {
                t[j][n - i - 1] = s[i][j];
            }
        }
        s = t;
    }

    println!("No");
}
