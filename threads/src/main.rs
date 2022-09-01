use std::thread;
use std::time;

fn pause_ms(ms: u64){
    thread::sleep(time::Duration::from_millis(ms));
}

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);

    v.iter()
        .filter(|&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum()
}

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    let handle = thread::spawn(move || expensive_sum(v));
    let sum = handle.join().unwrap();
    println!("{}", sum);
}
