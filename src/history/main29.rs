

///
/// 可变性
///
fn main() {
    let x:i32 = 5;
    println!("X:<{}>",x);

    let mut y:i64 = 11;
    y = 22;
    println!("Y:<{}>",y);

    let  mut j:i64 = 32;
    let k = &mut j;
    // println!("j:{} k:{}",j,k);
    println!("k:{}",k);

    // 可变引用的可变绑定
    println!("\n===>可变应用的可变绑定");
    let mut x:i32 = 111;
    // X只能在此处重新赋值，否则在借走之后无法赋值
    x = 222;
    let mut y:&mut i32 = &mut x;
    // println!("x:{} y:{}",x,y);
    println!("y:{}",y);

    // 元组可变绑定
    println!("\n===>元组可变绑定");
    let (mut x,y) = (888,999);
    println!("x:{} y:{}",x,y);

}



