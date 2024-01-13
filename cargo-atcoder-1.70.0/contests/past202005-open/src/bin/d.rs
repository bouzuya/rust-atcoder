use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; 5],
    };

    let lines = vec![
        ".###..#..###.###.#.#.###.###.###.###.###.",
        ".#.#.##....#...#.#.#.#...#.....#.#.#.#.#.",
        ".#.#..#..###.###.###.###.###...#.###.###.",
        ".#.#..#..#.....#...#...#.#.#...#.#.#...#.",
        ".###.###.###.###...#.###.###...#.###.###.",
    ]
    .into_iter()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    let mut ans = vec![];
    for i in 0..n {
        let mut d = 0;
        for j in 0..10 {
            let mut ok = true;
            for k in 0..5 {
                for l in 0..4 {
                    if s[k][4 * i + l] != lines[k][4 * j + l] {
                        ok = false;
                    }
                }
            }
            if ok {
                d = j;
                break;
            }
        }

        ans.push(d);
    }

    let ans = ans.into_iter().map(|d| d.to_string()).collect::<String>();
    println!("{}", ans);
}
