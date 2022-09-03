use std::sync::mpsc::channel;
use std::thread;
use std::time;

fn pause_ms(ms: u64) {
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
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    let handle = thread::spawn(move || expensive_sum(v));

    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        pause_ms(200);
        println!("{}", letter);
    }

    let sum = handle.join().unwrap();
    println!("{}", sum);

    let (tx, rx) = channel();
    let tx2 = tx.clone();
    let handle_a = thread::spawn(move || {
        pause_ms(500);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    pause_ms(100);

    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    for msg in rx {
        println!("{}", msg);
    }

    handle_a.join().unwrap();
    handle_b.join().unwrap();
}
