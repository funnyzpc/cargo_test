
///
/// 结构体
///
fn main() {
    // 定义结构体参数
    println!("\n===>定义结构体参数<===");
    let origin = Point{x:11,y:55};
    println!("=>The struct data is ({},{}).",origin.x,origin.y);
    let origin2 = Point{y:1,x:2};
    println!("=>The struct2 data is ({},{}).",origin2.x,origin2.y);

    // 结构体可变绑定
    println!("\n===>结构体可变绑定<===");
    let mut point  = Point{x:12,y:13};
    point.x = 122;
    point.y = 133;
    println!("(x:{},y:{})",point.x,point.y);

    // 结构体参数可变
    println!("\n===>结构体参数可变<===");
    let r = PointRef{x:&mut 19,y:&mut 20};
    println!("before x:{},y:{}",r.x,r.y);
    *r.x = 55;
    *r.y = 66;
    println!("after x:{},y:{}",r.x,r.y);

    // 结构体参数可变(2)
    println!("\n===>结构体参数可变(2)<===");
    let mut point  = Point{x:0,y:0};
    println!("before x:{} y:{}",point.x,point.y);
    {
        // 此处由于使用的是point的可变引用，故r与point的字段双向绑定
        let r = PointRef{x:&mut point.x,y:&mut point.y};
        *r.x = 77;
        *r.y = 88;
        println!("before r.x:{} r.y:{}",r.x,r.y);
    }
    println!("before point.x:{} point.y:{}",point.x,point.y);

}

struct Point{
    x:i32,
    y:i32,
}

struct PointRef<'a>{
    x: &'a mut i32,
    y: &'a mut i32,
}