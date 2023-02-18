use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    // rc
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());

    // arc
    let aa = Arc::new([1, 2, 3]);
    let ab = aa.clone();

    let t1 = thread::spawn(move || dbg!(aa));
    let t2 = thread::spawn(move || dbg!(ab));
    t1.join().unwrap();
    t2.join().unwrap();
}
