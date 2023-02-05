use proconio::input;

fn main() {
    input! {
        mut xyz: [usize; 3],
    };
    xyz.swap(0, 1);
    xyz.swap(0, 2);
    println!("{} {} {}", xyz[0], xyz[1], xyz[2]);
}
