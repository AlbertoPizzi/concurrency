use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Debug)]
struct Fork {
    id: usize,
}

impl Fork {
    fn new(id: usize) -> Self {
        Fork { id }
    }
}

struct Philosopher {
    name: String,
    left: Arc<Mutex<Fork>>,
    right: Arc<Mutex<Fork>>,
}

impl Philosopher {
    fn new(name: &str, left: Arc<Mutex<Fork>>, right: Arc<Mutex<Fork>>) -> Self {
        Philosopher {
            name: name.to_string(),
            left,
            right,
        }
    }

    fn dine(&self) {
        loop {
            println!("{} está pensando...", self.name);
            thread::sleep(Duration::from_millis(1000));

            println!("{} quiere comer...", self.name);

            let _left = self.left.lock().unwrap();
            println!("{} tomó el tenedor izquierdo (#{})", self.name, _left.id);

            // Breve pausa para aumentar probabilidad de deadlock si no se maneja
            thread::sleep(Duration::from_millis(10));

            let _right = self.right.lock().unwrap();
            println!("{} tomó el tenedor derecho (#{})", self.name, _right.id);

            println!("{} está comiendo...", self.name);
            thread::sleep(Duration::from_millis(1000));
            println!("{} terminó de comer y dejó los tenedores.\n", self.name);
        }
    }
}

fn main() {
    let num = 5;
    let forks: Vec<_> = (0..num).map(|i| Arc::new(Mutex::new(Fork::new(i)))).collect();

    let philosophers = vec![
        Philosopher::new("Aristóteles", forks[0].clone(), forks[1].clone()),
        Philosopher::new("Kant", forks[1].clone(), forks[2].clone()),
        Philosopher::new("Spinoza", forks[2].clone(), forks[3].clone()),
        Philosopher::new("Marx", forks[3].clone(), forks[4].clone()),
        // Evitar deadlock: el último toma en orden inverso
        Philosopher::new("Nietzsche", forks[0].clone(), forks[4].clone()),
    ];

    let handles: Vec<_> = philosophers
        .into_iter()
        .map(|p| thread::spawn(move || p.dine()))
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}
