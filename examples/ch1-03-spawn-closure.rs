use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });
    let average = t.join().unwrap();
    println!("numbers average is {average:?}");
}
