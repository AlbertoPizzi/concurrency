use std::sync::Arc;
use tokio::sync::Semaphore;

const ELEVATOR_CAPACITY: usize = 4;
const TOTAL_PEOPLE: usize = 10;

#[tokio::main]
async fn main() {
    let elevator = Arc::new(Semaphore::new(ELEVATOR_CAPACITY));
    let mut handles = vec![];

    for person_id in 0..TOTAL_PEOPLE {
        let elevator_clone = elevator.clone();

        let handle = tokio::spawn(async move {
            println!("Person {} is waiting to enter the elevator...", person_id);

            // Wait until elevator has space (acquire permit)
            let permit = elevator_clone.acquire().await.unwrap();

            println!("Person {} entered the elevator.", person_id);

            // Perform elevator usage logic here
            println!("Person {} is inside the elevator.", person_id);

            // Person exits elevator (permit released)
            drop(permit);
            println!("Person {} exited the elevator.", person_id);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("All people have used the elevator.");
}
