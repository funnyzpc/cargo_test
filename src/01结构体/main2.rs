
///
/// 元组结构体
///
fn main() {
    // 元组结构体
    println!("\n===>元组结构体");
   let black = Color(true,false,11.22);
    println!("black 0={} 1={} 2={}",black.0,black.1,black.2);

    let origin = Point(45.0,11.0);
    println!("origin 0={} 1={}",origin.0,origin.1);

    let x = (11,true,22.22);
    println!("x 0:{} 1:{} 2:{}",x.0,x.1,x.2);

    // 元组变体
    println!("\n===>元组变体");
    let  i = Inches(199);
    println!("i.0 is {}",i.0);
    let Inches(i) = i;
    println!("i is {}",i);

}

struct Color(bool,bool,f32);
struct Point(f32,f32);
struct Inches(i64);