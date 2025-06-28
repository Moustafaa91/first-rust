#[allow(dead_code)]
mod guess_game;
#[allow(dead_code)]
mod types_info;
#[allow(dead_code)]
mod data_structures;
use crate::data_structures::Collection;
mod queue;
use crate::queue::Queue;

fn main() {
    //guess_game::guess_game();
    //types_info::print_types_info();
    let mut queue = Queue::new();
    queue.enqueue('a');
    queue.enqueue('b');
    queue.enqueue('c');
    queue.enqueue('d');
    println!("Queue count: {}", <Queue<char> as Collection<char>>::count(&queue));

    if let Some(val) = queue.dequeue() {
         println!("Dequeued char: {}", val);
    }
    println!("Queue count after dequeue: {}", <Queue<char> as Collection<char>>::count(&queue));
}
