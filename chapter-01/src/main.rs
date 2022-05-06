#![feature(scoped_threads)]
use std::{rc::Rc, sync::Arc, thread};

static X: [i32; 3] = [1, 2, 3];

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread.");

    t1.join().unwrap();
    t2.join().unwrap();

    let numbers = vec![1, 2, 3];
    let t = thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    });
    t.join().unwrap();

    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });
    let average = t.join().unwrap();
    println!("average: {average}");

    let numbers = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });

    let t3 = thread::spawn(|| dbg!(&X));
    let t4 = thread::spawn(|| dbg!(&X));
    t3.join().unwrap();
    t4.join().unwrap();

    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    let t5 = thread::spawn(move || dbg!(x));
    let t6 = thread::spawn(move || dbg!(x));
    t5.join().unwrap();
    t6.join().unwrap();

    let a = Rc::new([1, 2, 3]);
    let b = a.clone();
    assert_eq!(a.as_ptr(), b.as_ptr());

    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    let t7 = thread::spawn(move || dbg!(a));
    let t8 = thread::spawn(move || dbg!(b));
    t7.join().unwrap();
    t8.join().unwrap();

    let a = Arc::new([1, 2, 3]);
    let t9 = thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });
    dbg!(a);
    t9.thread().unpark();
    t9.join().unwrap();
}

fn f() {
    println!("Hello from another thread.");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}

pub fn f2(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        println!("Never Happens");
    }
}
