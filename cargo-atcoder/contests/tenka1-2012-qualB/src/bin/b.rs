use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        c: Chars,
    };
    let (i_s, i_e, s) = {
        let mut i_s = 0;
        for &c_i in c.iter() {
            if c_i != '_' {
                break;
            }
            i_s += 1;
        }
        let mut i_e = 0;
        for &c_i in c.iter().rev() {
            if c_i != '_' {
                break;
            }
            i_e += 1;
        }
        if i_s >= c.len() || i_e >= c.len() {
            println!("{}", c.iter().collect::<String>());
            return;
        }
        (
            i_s,
            i_e,
            c[i_s..c.len() - i_e].iter().cloned().collect::<Vec<char>>(),
        )
    };

    let mut camel_case = true;
    let mut snake_case = true;
    let mut has_underscore = false;
    let mut prev_underscore = false;
    for (i, &c) in s.iter().enumerate() {
        let is_underscore = c == '_';
        let is_lowercase = ('a'..='z').contains(&c);
        let is_uppercase = ('A'..='Z').contains(&c);
        let is_digit = ('0'..='9').contains(&c);

        // camel_case
        if i == 0 && !is_lowercase {
            camel_case = false;
        }
        if is_underscore {
            camel_case = false;
        }

        // snake_case
        if i == 0 && is_digit {
            snake_case = false;
        }
        if prev_underscore && !is_lowercase {
            snake_case = false;
        }
        if is_uppercase {
            snake_case = false;
        }
        if is_underscore {
            has_underscore = true;
            prev_underscore = true;
        } else {
            prev_underscore = false;
        }
    }
    if !has_underscore {
        snake_case = false;
    }
    match (camel_case, snake_case) {
        (false, false) => println!("{}", c.iter().collect::<String>()),
        (true, false) | (true, true) => {
            let mut t = vec![];
            for &c in s.iter() {
                if ('A'..='Z').contains(&c) {
                    t.push('_');
                    t.push(c.to_ascii_lowercase());
                } else {
                    t.push(c);
                }
            }
            println!(
                "{}{}{}",
                "_".repeat(i_s),
                t.iter().collect::<String>(),
                "_".repeat(i_e),
            );
        }
        (false, true) => {
            let mut t = vec![];
            let mut next_upper = false;
            for &c in s.iter() {
                if next_upper {
                    t.push(c.to_ascii_uppercase());
                    next_upper = false;
                } else {
                    if c == '_' {
                        next_upper = true;
                    } else {
                        t.push(c);
                    }
                }
            }
            println!(
                "{}{}{}",
                "_".repeat(i_s),
                t.iter().collect::<String>(),
                "_".repeat(i_e),
            );
        }
    }
}
