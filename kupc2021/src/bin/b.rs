use proconio::input;

fn f(s: &mut [Vec<char>], n: usize, o: usize) {
    if o > n {
        return;
    }
    match n - 1 - o {
        0 => {
            s[o][o] = '.';
        }
        1 => {
            s[o][o] = '#';
            s[o][o + 1] = '.';
            s[o + 1][o] = '.';
            s[o + 1][o + 1] = '.';
        }
        _ => {
            s[o][o] = '#';
            for j in o + 2..n {
                s[o][j] = '#';
            }
            for i in o + 2..n {
                s[i][o] = '#';
            }
            for j in o..n {
                s[o + 1][j] = '.';
            }
            for i in o..n {
                s[i][o + 1] = '.';
            }
            f(s, n, o + 2)
        }
    }
}

fn main() {
    input! {
        n: usize,
    };

    let mut s = vec![vec!['x'; n]; n];
    f(&mut s, n, 0);

    for i in 0..n {
        for j in 0..n {
            print!("{}", s[i][j]);
        }
        println!();
    }
}
