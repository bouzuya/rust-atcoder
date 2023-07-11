use proconio::input;

fn main() {
    input! {
        xy: String,
    };
    let xy = xy.split('.').collect::<Vec<&str>>();
    let x = xy[0].parse::<usize>().unwrap();
    let y = xy[1].parse::<usize>().unwrap();
    let ys = if y <= 2 {
        "-"
    } else if 3 <= y && y <= 6 {
        ""
    } else if 7 <= y && y <= 9 {
        "+"
    } else {
        unreachable!()
    };
    println!("{}{}", x, ys);
}
