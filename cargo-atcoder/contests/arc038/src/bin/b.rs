use proconio::input;
use proconio::marker::Chars;

fn is_win(s: &Vec<Vec<char>>, i: usize, j: usize, memo: &mut Vec<Vec<Option<bool>>>) -> bool {
    match memo[i][j] {
        Some(b) => b,
        None => {
            if s[i][j] == '#' {
                true
            } else {
                let b1 = !is_win(&s, i + 1, j, memo);
                let b2 = !is_win(&s, i + 1, j + 1, memo);
                let b3 = !is_win(&s, i, j + 1, memo);
                let b = b1 || b2 || b3;
                memo[i][j] = Some(b);
                b
            }
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut s: [Chars; h],
    };

    for i in 0..h {
        s[i].push('#');
    }
    s.push((0..w + 1).map(|_| '#').collect::<Vec<char>>());

    let mut memo = vec![vec![None; w + 2]; h + 2];
    let ans = is_win(&s, 0, 0, &mut memo);
    println!("{}", if ans { "First" } else { "Second" });
}
