fn area_of(width: i32, height: i32) -> i32 {
    width * height
}

fn volume(width: i32, height: i32, depth: i32) -> i32 {
    width * height * depth
}

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    let area = area_of(width, height);

    println!("{}", area);

    println!("{}", volume(width, height, depth))
}
