use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    for x in n.. {
        let d = x
            .to_string()
            .chars()
            .map(|c| (c as u8 - b'0') as usize)
            .collect::<Vec<usize>>();
        if d[0] * d[1] == d[2] {
            println!("{}{}{}", d[0], d[1], d[2]);
            return;
        }
    }
}
