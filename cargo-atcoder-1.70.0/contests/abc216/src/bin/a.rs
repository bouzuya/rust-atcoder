use proconio::input;

fn main() {
    input! {
        xy: String,
    };
    let xy = xy.split('.').collect::<Vec<&str>>();
    let x = xy[0].parse::<usize>().unwrap();
    let y = xy[1].parse::<usize>().unwrap();
    if (0..=2).contains(&y) {
        println!("{x}-");
    } else if (3..=6).contains(&y) {
        println!("{x}");
    } else if (7..=9).contains(&y) {
        println!("{x}+");
    } else {
        unreachable!()
    }
}
