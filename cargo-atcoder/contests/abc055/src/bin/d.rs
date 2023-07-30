use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };

    for t_0 in vec!['S', 'W'] {
        for t_1 in vec!['S', 'W'] {
            let mut t = vec![' '; n];
            t[0] = t_0;
            t[1] = t_1;

            let update = |t: &mut Vec<char>, i: usize| -> bool {
                let i_left = (n + i - 1) % n;
                let i_right = (i + 1) % n;
                match (s[i], t[i]) {
                    ('o', 'S') | ('x', 'W') => {
                        match (t[i_left] == ' ', t[i_right] == ' ') {
                            (false, false) | (true, true) => {
                                // do nothing
                            }
                            (false, true) => {
                                t[i_right] = t[i_left];
                            }
                            (true, false) => {
                                t[i_left] = t[i_right];
                            }
                        }
                        if t[i_left] == ' ' {}
                        t[i_left] == t[i_right]
                    }
                    ('o', 'W') | ('x', 'S') => {
                        match (t[i_left] == ' ', t[i_right] == ' ') {
                            (false, false) | (true, true) => {
                                // do nothing
                            }
                            (false, true) => {
                                t[i_right] = if t[i_left] == 'S' { 'W' } else { 'S' };
                            }
                            (true, false) => {
                                t[i_left] = if t[i_right] == 'S' { 'W' } else { 'S' };
                            }
                        }
                        t[i_left] != t[i_right]
                    }
                    _ => {
                        println!("{} {} {}", i, s[i], t[i]);
                        unreachable!()
                    }
                }
            };

            let mut ok = true;
            for i in 0..n {
                if !update(&mut t, i) {
                    ok = false;
                    break;
                }
            }
            if ok {
                let ans = t.iter().collect::<String>();
                println!("{}", ans);
                return;
            }
        }
    }
    println!("-1");
}
