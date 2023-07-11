// サンプル通っていない
use proconio::input;
use proconio::marker::Chars;

fn score(c: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for i in 0..c.len() {
        for j in 0..c.len() - 1 {
            if c[i][j] != c[i][j + 1] {
                count += 1;
            }
        }
    }
    for j in 0..c.len() {
        for i in 0..c.len() - 1 {
            if c[i][j] != c[i + 1][j] {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        mut c: [Chars; n],
    };
    let mut counts = vec![vec![vec![false; n]; n]; 4 + 1];
    let mut qs = vec![];
    for i in 0..n {
        for j in 0..n {
            if c[i][j] == '?' {
                let mut count = 0;
                let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
                for (dy, dx) in dir {
                    let (ny, nx) = (i as i64 + dy, j as i64 + dx);
                    if (0..n as i64).contains(&ny) && (0..n as i64).contains(&nx) {
                        let (ny, nx) = (ny as usize, nx as usize);
                        if c[ny][nx] == 'B' || c[ny][nx] == 'W' {
                            count += 1;
                        }
                    }
                }
                counts[count][i][j] = true;
                qs.push((i, j));
            }
        }
    }

    c[0][0] = 'B';
    let mut count_q = qs.len();
    while count_q > 0 {
        count_q -= 1;
    }

    for count in (1..=4).rev() {
        for i in 0..n {
            for j in 0..n {
                if counts[count][i][j] {}
            }
        }
    }
    let ans = n - a.len();
    println!("{}", ans);
}
