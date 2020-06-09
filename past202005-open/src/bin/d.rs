use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; 5],
    };
    let t = vec![
        ".###..#..###.###.#.#.###.###.###.###.###.",
        ".#.#.##....#...#.#.#.#...#.....#.#.#.#.#.",
        ".#.#..#..###.###.###.###.###...#.###.###.",
        ".#.#..#..#.....#...#...#.#.#...#.#.#...#.",
        ".###.###.###.###...#.###.###...#.###.###.",
    ]
    .iter()
    .map(|&d_i| d_i.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    let mut ans = vec![];
    for i in 0..n {
        for d in 0..=9 {
            if (0..5).all(|y| (0..4).all(|x| s[y][4 * i + x] == t[y][4 * d + x])) {
                ans.push(std::char::from_digit(d as u32, 10).unwrap());
                break;
            }
        }
    }
    println!("{}", ans.iter().collect::<String>());
}
