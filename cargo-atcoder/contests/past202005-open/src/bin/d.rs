use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; 5],
    };

    let tbl = vec![
        ".###..#..###.###.#.#.###.###.###.###.###.",
        ".#.#.##....#...#.#.#.#...#.....#.#.#.#.#.",
        ".#.#..#..###.###.###.###.###...#.###.###.",
        ".#.#..#..#.....#...#...#.#.#...#.#.#...#.",
        ".###.###.###.###...#.###.###...#.###.###.",
    ]
    .into_iter()
    .map(|s_i| s_i.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    let mut digits = vec![];
    for i in 0..=9 {
        let l = 4 * i;
        let r = 4 * (i + 1);
        let mut ch = vec![];
        for j in 0..5 {
            ch.push(&tbl[j][l..r]);
        }
        digits.push(ch);
    }

    for i in 0..n {
        let l = 4 * i;
        let r = 4 * (i + 1);
        let mut ch1 = vec![];
        for j in 0..5 {
            ch1.push(&s[j][l..r]);
        }

        let mut d = 0;
        for (j, ch2) in digits.iter().enumerate() {
            if &ch1 == ch2 {
                d = j;
            }
        }
        print!("{}", d);
    }
    println!();
}
