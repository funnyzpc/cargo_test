use std::sync::Arc;
use std::cell::RefCell;

///
/// 可变性
///
fn main() {
    // 测试01
    println!("===>测试01<===");
    let  x:i32 = 5;
    let y = &x;
    println!("=> x:{} y:{}",x,y);

    // 测试02
    println!("===>测试02<===");
    let mut x:i32 = 6;
    let  y:&mut i32 = &mut x;
    // x在此时已经被借走了，故不可获取并打印
    // println!("x:{} y:{}",x,y);
    println!("y:{}",y);

    // 测试03
    println!("===>测试03<===");
    let (mut x,y) = (7,8);
    x =  55;
    println!("=> x:{} y:{}",x,y);


    // 测试04 (外部可变性)
    println!("===>测试04<===");
    let x:Arc<i32> = Arc::new(10);
    let y:Arc<i32> = x.clone();
    println!("x:{} y:{}",x,y);

    // 测试05 (内部可变性)
    println!("===>测试05<===");
    let x = RefCell::new(45);
    let y = x.borrow_mut();
    // 已使用借用后x被借走，故不可再使用x
    // println!("x:{} y:{}",x,y);
    println!(" y:{} -",y);

    // 测试06
    let x = RefCell::new(55);
    let y = x.borrow_mut();
    let z = x.borrow_mut();
    // println!("=> x:{} y:{} z:{}",x,y,z);
    println!("=> y:{} z:{}",y,z);

}