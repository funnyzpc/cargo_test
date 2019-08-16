use std::cell::Cell;

///
/// 可变性
///
fn main() {
    // 结构体可变性
    println!("===>结构体可变性<===");
    let mut a = Point{x:4,y:5};
    a.x = 11;
    println!("first x:{} y:{}",a.x,a.y);
    // 这里必须将b声明为一个可修改的mut,不然结构体内参数不可修改。
    let mut b = Point{x:7,y:8};
    b.x = 22;
    println!("next x:{} y:{}",b.x,b.y);

    // 字段级别可变性
    println!("\n===>字段级别可变性<===");
    let point = Point2{x:9,y:Cell::new(27)};
    point.y.set(66);
    println!("point x:{:?} y:{:?}",point.x,point.y);
    let point2 = Point2{x:77,y:Cell::new(88)};
    point2.y.set(99);
    println!("point2 x:{:?} y:{:?}",point2.x,point2.y);

}
struct Point2{
    x:i32,
    y:Cell<i32>,
}
struct Point{
    x:i32,
    y:i32
}