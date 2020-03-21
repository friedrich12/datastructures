extern crate datastructures;
use datastructures::queue::Queue;

#[test]
fn basics3(){
    let mut q = Queue::new();
    q.push(9);
    q.push(5);

    assert_eq!(q.pop(), Some(9));


    assert_eq!(q.pop(), Some(5));
    

}