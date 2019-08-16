///
/// 结构体2
///
fn main() {
    // 结构体参数拷贝
    println!("\n===>结构体参数拷贝<===");
    let mut point = Point3d{x:3,y:3,z:3};
    println!("before (x:{},y:{},z:{})",point.x,point.y,point.z);
    // 这里使用..代替多个参数体
    point = Point3d{x:1,..point};
    println!("after (x:{},y:{},z:{})",point.x,point.y,point.z);

    let point2 = Point3d{x:9,y:9,z:9};
    point = Point3d{..point2};
    println!("after (x:{},y:{},z:{})",point.x,point.y,point.z);

    // 拷贝只能在同一个结构体的不同实例间进行,故以逻辑会抛异常
    /*
    let point4 = Point4d{y:9,z:9};
    point = Point3d{x:4,..point4};
    println!("after (x:{},y:{},z:{})",point.x,point.y,point.z);
    */

    // let x = struct {x:1,y:2,z:3};

}

struct Point3d{
    x:i32,
    y:i32,
    z:i32,
}
struct Point4d{
    y:i32,
    z:i32,
}