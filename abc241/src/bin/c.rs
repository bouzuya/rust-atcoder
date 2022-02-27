use proconio::{input, marker::Chars};

fn check_row(n: usize, s: &[Vec<char>]) -> bool {
    for y in 0..n {
        for x in 0..=n - 6 {
            let mut count = 0;
            for i in 0..6 {
                if s[y][x + i] == '#' {
                    count += 1;
                }
            }
            if count >= 4 {
                return true;
            }
        }
    }
    false
}

fn check_col(n: usize, s: &[Vec<char>]) -> bool {
    for x in 0..n {
        for y in 0..=n - 6 {
            let mut count = 0;
            for i in 0..6 {
                if s[y + i][x] == '#' {
                    count += 1;
                }
            }
            if count >= 4 {
                return true;
            }
        }
    }
    false
}

fn check_diagonal(n: usize, s: &[Vec<char>]) -> bool {
    for x in 0..=n - 6 {
        for y in 0..=n - 6 {
            let mut count = 0;
            for i in 0..6 {
                if s[y + i][x + i] == '#' {
                    count += 1;
                }
            }
            if count >= 4 {
                return true;
            }
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    if check_row(n, &s) || check_col(n, &s) || check_diagonal(n, &s) {
        println!("Yes");
        return;
    }

    let mut t = vec![vec![' '; n]; n];
    for i in 0..n {
        for j in 0..n {
            t[j][n - i - 1] = s[i][j];
        }
    }
    let s = t;

    if check_row(n, &s) || check_col(n, &s) || check_diagonal(n, &s) {
        println!("Yes");
        return;
    }

    println!("No");
}
