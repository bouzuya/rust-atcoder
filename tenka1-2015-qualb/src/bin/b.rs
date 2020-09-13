use proconio::input;
use proconio::marker::Chars;

#[derive(Debug)]
enum Expr {
    Dict,
    Set,
    Integer,
}

fn read_expr(s: &[char], i: &mut usize) -> Expr {
    if s[*i] == '{' {
        *i += 1; // {
        if s[*i] == '}' {
            *i += 1; // } (=dict)
            Expr::Dict
        } else {
            read_expr(s, i); // dict key or set value
            if s[*i] == ':' {
                *i += 1; // :
                read_expr(s, i); // dict value
                while s[*i] == ',' {
                    *i += 1; // ,
                    read_expr(s, i); // dict key
                    *i += 1; // :
                    read_expr(s, i); // dict value
                }
                *i += 1; // }
                Expr::Dict
            } else {
                while s[*i] == ',' {
                    *i += 1; // ,
                    read_expr(s, i); // set value
                }
                *i += 1; // }
                Expr::Set
            }
        }
    } else {
        while s[*i].is_digit(10) {
            *i += 1;
        }
        Expr::Integer
    }
}

fn main() {
    input! {
        s: Chars,
    };

    let expr = read_expr(&s, &mut 0);
    match expr {
        Expr::Dict => println!("dict"),
        Expr::Set => println!("set"),
        Expr::Integer => unreachable!(),
    }
}
