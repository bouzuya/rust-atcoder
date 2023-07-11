use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        w: [Chars; n],
    };
    let ans = w
        .iter()
        .map(|w_i| {
            w_i.iter()
                .map(|&w_i_j| match w_i_j.to_ascii_lowercase() {
                    'a' => None,
                    'b' => Some('1'),
                    'c' => Some('1'),
                    'd' => Some('2'),
                    'e' => None,
                    'f' => Some('4'),
                    'g' => Some('9'),
                    'h' => Some('8'),
                    'i' => None,
                    'j' => Some('3'),
                    'k' => Some('8'),
                    'l' => Some('5'),
                    'm' => Some('7'),
                    'n' => Some('9'),
                    'o' => None,
                    'p' => Some('7'),
                    'q' => Some('4'),
                    'r' => Some('0'),
                    's' => Some('6'),
                    't' => Some('3'),
                    'u' => None,
                    'v' => Some('5'),
                    'w' => Some('2'),
                    'x' => Some('6'),
                    'y' => None,
                    'z' => Some('0'),
                    ',' => None,
                    '.' => None,
                    _ => unreachable!(),
                })
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect::<String>()
        })
        .filter(|w_i| !w_i.is_empty())
        .collect::<Vec<String>>();
    for (i, ans_i) in ans.iter().enumerate() {
        print!("{}{}", ans_i, if i == ans.len() - 1 { "\n" } else { " " });
    }
    if ans.is_empty() {
        println!();
    }
}
