use std::mem;

fn main() {

    let mut v: Vec<i32> = vec![1, 2];
    let mut  old_v = mem::replace(&mut v, vec![3, 4, 5]);
    assert_eq!(vec![1, 2], old_v);
    assert_eq!(vec![3, 4, 5], v);

    v.push(5);
    old_v.push(5);
   // assert_eq!(v,old_v);
    println!("Hello, world!");
}