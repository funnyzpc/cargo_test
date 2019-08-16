///
/// 结构体3
fn main() {
    // 测试01
    let origin = Point3d { x: 0, y: 1, z: 2 };
    let point = Point3d {
        z: 1,
        x: 2,
        y:33,
        ..origin
    };
    println!("(x:{},y:{},z:{})",point.x,point.y,point.z);

    // 元组结构体
    println!("\n===>元组结构体<===");
    let black = Color(1,11,111);
    let origin = Point(2,22,222);
    // black和origin并不是相同的类型，即使它们有一模一样的值。
    println!("black ({},{},{})",black.0,black.1,black.2);
    println!("origin ({},{},{})",origin.0,origin.1,origin.2);

    // 元组结构体 类型包装
    println!("\n===>类型包装<===");
    let length  = IntChes(222);
    let IntChes(int_len) = &length;
    println!("int_len:{},length:{}",int_len,length.0);


}

struct IntChes(i32);

// 元组结构体
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);


struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}
