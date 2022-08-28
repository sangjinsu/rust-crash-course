use std::env::args;

fn main() {
    let coords: (f32, f32) = (5.9, 4.9);
    print_difference(coords.0, coords.1);

    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_array(coords_arr);

    let series = [1, 2, 3, 45, 2, 1, 3];
    println!("{:?}", series[6]);

    let mess = ((false, 3.45), true, 3.45, 2);
    println!("{:?}", mess);

    let args = args().skip(1).collect::<Vec<String>>();
    for arg in args {
        if arg == "sum" {
            sum()
        } else if arg == "double" {
            double()
        } else {
            count(arg)
        }
    }
}

fn print_difference(a: f32, b: f32) {
    println!("{}", (a - b).abs())
}

fn print_array(arr: [f32; 2]) {
    println!("{:?}", arr)
}

fn sum() {
    let mut sum = 0;
    for i in 7..23 {
        sum += i
    }
    println!("{}", sum)
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    while x < 500 {
        count += 1;
        x *= 2;
    }
    println!("{}", count)
}

fn count(arg: String) {
    let mut i = 0;
    loop {
        if i == 8 {
            break;
        }
        println!("{}", arg);
        i += 1;
    }
}
