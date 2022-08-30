trait Noisy {
    fn get_noise(&self) -> &Self;
}

trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)]
struct RedFox {
    enemy: bool,
    life: u8,
}

impl RedFox {
    pub fn new(enemy: bool, life: u8) -> Self {
        Self { enemy, life }
    }
}

#[derive(Debug)]
struct Grapes {
    amount_left: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}

enum Color {
    Red,
    Blue,
}

enum DispenserItem {
    Empty,
    Things(String, i32),
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let fox = RedFox::new(true, 100);
    println!("{} {}", fox.life, fox.enemy);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("{:?}", grapes)
}
