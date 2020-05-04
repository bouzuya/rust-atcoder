use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut a: usize,
        mut b: usize,
        mut c: usize,
        s: [Chars; n],
    };
    let mut ans = vec!['_'; n];
    for (i, s_i) in s.iter().enumerate() {
        match (s_i[0], s_i[1]) {
            ('A', 'B') => {
                if a == 0 && b == 0 {
                    println!("No");
                    return;
                } else if a == 0 {
                    a += 1;
                    b -= 1;
                    ans[i] = 'A';
                } else if b == 0 {
                    a -= 1;
                    b += 1;
                    ans[i] = 'B';
                } else {
                    if i < n - 1 && (s[i + 1][0], s[i + 1][1]) == ('B', 'C') {
                        a -= 1;
                        b += 1;
                        ans[i] = 'B';
                    } else {
                        a += 1;
                        b -= 1;
                        ans[i] = 'A';
                    }
                }
            }
            ('A', 'C') => {
                if a == 0 && c == 0 {
                    println!("No");
                    return;
                } else if a == 0 {
                    a += 1;
                    c -= 1;
                    ans[i] = 'A';
                } else if c == 0 {
                    a -= 1;
                    c += 1;
                    ans[i] = 'C';
                } else {
                    if i < n - 1 && (s[i + 1][0], s[i + 1][1]) == ('B', 'C') {
                        a -= 1;
                        c += 1;
                        ans[i] = 'C';
                    } else {
                        a += 1;
                        c -= 1;
                        ans[i] = 'A';
                    }
                }
            }
            ('B', 'C') => {
                if b == 0 && c == 0 {
                    println!("No");
                    return;
                } else if c == 0 {
                    b -= 1;
                    c += 1;
                    ans[i] = 'C';
                } else if b == 0 {
                    b += 1;
                    c -= 1;
                    ans[i] = 'B';
                } else {
                    if i < n - 1 && (s[i + 1][0], s[i + 1][1]) == ('A', 'C') {
                        b -= 1;
                        c += 1;
                        ans[i] = 'C';
                    } else {
                        b += 1;
                        c -= 1;
                        ans[i] = 'B';
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    println!("Yes");
    for a in ans {
        println!("{}", a);
    }
}
