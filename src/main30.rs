use std::sync::Arc;
use std::cell::{RefCell, RefMut};

///
/// 内部可变性&外部可变性
///

fn main() {
    // 外部可变性
    println!("\n===>外部可变性");
    let x = Arc::new(22);
    let y = x.clone();
    println!("x:{} y:{}",x,y);

    // 拥有一个或多个不可变引用(&T)
    println!("\n===>拥有一个或多个不可变引用");
    let j:f64 = 999.888;
    let k:&f64 = &j;
    let l:&f64 = &j;
    println!("j:{} k:{} l:{}",j,k,l);

    // 拥有一个可变引用(&mut T)
    println!("\n===>拥有一个可变引用");
    let mut j = false;
    let k:&mut bool = &mut j;
    // let l = &mut j;
    // println!("j:{} k:{} l:{}",j,k,l);
    // println!("j:{} k:{}",j,k);
    println!("k:{}",k);

    // 内部可变性 (内部可变性遵循一般原则,使用借用规则)
    println!("\n===>内部可变性");
    let x:RefCell<f64> = RefCell::new(34.567);
    let y = x.borrow_mut();
    // let z = x.borrow_mut();
    // println!("x:{} y:{}",x.get_mut(),y);
    println!("y:{}",y);
}















