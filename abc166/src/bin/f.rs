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
    let mut n_a = 0;
    let mut n_b = 0;
    let mut n_c = 0;
    for s_i in s.iter() {
        match (s_i[0], s_i[1]) {
            ('A', 'B') => {
                n_a += 1;
                n_b += 1;
            }
            ('A', 'C') => {
                n_a += 1;
                n_c += 1;
            }
            ('B', 'C') => {
                n_b += 1;
                n_c += 1;
            }
            _ => unreachable!(),
        }
    }
    let mut ans = vec!['_'; n];
    for (i, s_i) in s.iter().enumerate() {
        match (s_i[0], s_i[1]) {
            ('A', 'B') => {
                if a == 0 && b == 0 {
                    println!("No");
                    return;
                } else if b == 0 {
                    a -= 1;
                    b += 1;
                    ans[i] = 'B';
                } else if a == 0 {
                    a += 1;
                    b -= 1;
                    ans[i] = 'A';
                } else {
                    if n_a <= a && n_b <= b {
                        if n_a >= n_b {
                            a += 1;
                            b -= 1;
                            ans[i] = 'A';
                        } else {
                            a -= 1;
                            b += 1;
                            ans[i] = 'B';
                        }
                    } else if n_a <= a {
                        a -= 1;
                        b += 1;
                        ans[i] = 'B';
                    } else if n_b <= b {
                        a += 1;
                        b -= 1;
                        ans[i] = 'A';
                    } else {
                        if n_a >= n_b {
                            a += 1;
                            b -= 1;
                            ans[i] = 'A';
                        } else {
                            a -= 1;
                            b += 1;
                            ans[i] = 'B';
                        }
                    }
                }
                n_a -= 1;
                n_b -= 1;
            }
            ('A', 'C') => {
                if a == 0 && c == 0 {
                    println!("No");
                    return;
                } else if c == 0 {
                    a -= 1;
                    c += 1;
                    ans[i] = 'C';
                } else if a == 0 {
                    a += 1;
                    c -= 1;
                    ans[i] = 'A';
                } else {
                    if n_a <= a && n_c <= c {
                        if n_a >= n_c {
                            a += 1;
                            c -= 1;
                            ans[i] = 'A';
                        } else {
                            a -= 1;
                            c += 1;
                            ans[i] = 'C';
                        }
                    } else if n_a <= a {
                        a -= 1;
                        c += 1;
                        ans[i] = 'C';
                    } else if n_c <= c {
                        a += 1;
                        c -= 1;
                        ans[i] = 'A';
                    } else {
                        if n_a >= n_c {
                            a += 1;
                            c -= 1;
                            ans[i] = 'A';
                        } else {
                            a -= 1;
                            c += 1;
                            ans[i] = 'C';
                        }
                    }
                }
                n_a -= 1;
                n_c -= 1;
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
                    if n_b <= b && n_c <= c {
                        if n_b >= n_c {
                            b += 1;
                            c -= 1;
                            ans[i] = 'B';
                        } else {
                            b -= 1;
                            c += 1;
                            ans[i] = 'C';
                        }
                    } else if n_b <= b {
                        b -= 1;
                        c += 1;
                        ans[i] = 'C';
                    } else if n_c <= c {
                        b += 1;
                        c -= 1;
                        ans[i] = 'B';
                    } else {
                        if n_b >= n_c {
                            b += 1;
                            c -= 1;
                            ans[i] = 'B';
                        } else {
                            b -= 1;
                            c += 1;
                            ans[i] = 'C';
                        }
                    }
                }
                n_b -= 1;
                n_c -= 1;
            }
            _ => unreachable!(),
        }
    }
    println!("Yes");
    for a in ans {
        println!("{}", a);
    }
}
