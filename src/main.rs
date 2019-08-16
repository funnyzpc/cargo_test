use std::cell::Cell;

///
/// 字段级别可变性
///
fn main() {
    // 字段不可变
    println!("\n===>字段可变性:");
    let k = Point{x:11,y:22};
    println!("k.x:{} k.y:{}",k.x,k.y);

    // 绑定可变
    println!("\n===>绑定可变");
    let mut l = Point{x:33,y:44};
    l.x = 999;
    println!("l.x:{} l.y:{}",l.x,l.y);

    // 字段可变
    println!("\n===>字段可变");
    let m = Point2{j:44.11,k:Cell::new(55.11)};
    println!("m.j: {} m.k: {}",m.j,m.k.get());
    m.k.set(88.33);
    println!("m.j:{} m.k:{}",m.j,m.k.get());


}


struct Point{
    x: i32,
    // 这里的x,y只可为同一属性(可变性mut 或不可变现)
    // mut y: i32
    y: i32
}

struct Point2{
    j: f32,
    // 这里使用Cell<T> (模拟)字段可变性
    k: Cell<f64>
}