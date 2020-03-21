extern crate datastructures;
use datastructures::list::List;

#[test]
fn basics2(){
    let mut list = List::new();
    list.push(9);
    list.push(5);

    assert_eq!(list.pop(), Some(5));


    assert_eq!(list.pop(), Some(9));
    

}