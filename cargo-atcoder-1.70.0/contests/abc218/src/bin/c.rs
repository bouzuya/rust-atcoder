use proconio::{input, marker::Chars};

fn rotate(s: &[Vec<char>]) -> Vec<Vec<char>> {
    let n = s.len();
    let mut t = vec![vec![' '; n]; n];
    for i in 0..n {
        for j in 0..n {
            t[j][n - 1 - i] = s[i][j];
        }
    }
    t
}

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
        t: [Chars; n],
    };

    if s.iter()
        .map(|s_i| s_i.iter().filter(|&&c| c == '#').count())
        .sum::<usize>()
        != t.iter()
            .map(|t_i| t_i.iter().filter(|&&c| c == '#').count())
            .sum::<usize>()
    {
        println!("No");
        return;
    }

    for _ in 0..4 {
        let n2 = n as i64;
        for oi in -n2..=n2 {
            for oj in -n2..=n2 {
                let mut ok = true;
                'outer: for i in 0..n {
                    for j in 0..n {
                        if s[i][j] == '#'
                            && (!(0..n2).contains(&(i as i64 + oi))
                                || !(0..n2).contains(&(j as i64 + oj))
                                || t[(i as i64 + oi) as usize][(j as i64 + oj) as usize] != '#')
                        {
                            ok = false;
                            break 'outer;
                        }
                    }
                }
                if ok {
                    println!("Yes");
                    return;
                }
            }
        }
        s = rotate(&s);
    }
    println!("No");
}
