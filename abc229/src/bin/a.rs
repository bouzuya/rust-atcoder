use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 2],
    };

    if (s[0][0] == '#' && s[0][1] == '.' && s[1][0] == '.' && s[1][1] == '#')
        || (s[0][0] == '.' && s[0][1] == '#' && s[1][0] == '#' && s[1][1] == '.')
    {
        println!("No");
    } else {
        println!("Yes");
    }
}
