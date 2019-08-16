
///
/// if语句
///
fn main(){
    // 类型一
    let x:i64 = 990;
    if x == 99 {
        println!("X is 99 !");
    }else if x == 100 {
        println!("X is 100 !");
    }else{
        println!("X is other value !");
    }
    // 类型二
    let x:i32 = 6;
    let y = if x == 5{10}else{200};
    println!("x:{} y:{}",x,y);

    // 类型二+
    let x = 7;
    let y = if x==6{
        7
    }else{
        8
    };
    println!("y is {}",y);

}