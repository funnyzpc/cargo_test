use std::str::Lines;

///
/// 循环
///
fn main() {
   // loop循环
    println!("\n===>loop 循环“");
    let mut x:i8 = 1;
    loop{
        println!("current value is {}.",x);
        if x==10{
            println!("Target value is {},now is break.",x);
            break
        }
        x = x+1;
    }

    // while循环
    println!("\n===>while 循环“");
    let mut x:i8 = 2;
    let mut done:bool = false;
    while !done {
        println!("while current value is :{}",x);
        if x % 5 == 0{
            println!("while x % 5 is ZERO,so break.");
            done = true
        }
        x += -3;
    }

    // for循环
    println!("\n===>for 循环“");
    for x in 0..10{
        println!("for value is {}",x);
    }

    let y:[i8;5] = [22,44,55,39,69];
    for val in &y{
        println!("expression value: {}",val);
    }

    for(key,value) in (5..12).enumerate(){
        println!("key: {} and value: {}",key,value);
    }

    // 迭代器
    println!("\n===>iterators 循环“");
    let line:Lines = "hello \n youth\n!".lines();
    for(number,value) in line.enumerate(){
        println!("iterators number:{} value:{}",number,value);
    }

    // continue、break使用
    let mut x = 5;
    let mut done = false;
    while !done {
        x +=2;
        if x%3 == 0{
            continue
        }
        println!("c b : {}",x);
        if x%5 == 0{
            done = true;
        }
    }

    let mut y:i32 = 3;
    loop{
        y += 2;
        println!("val :{}",y);
        if y%7==0{
            break;
        }
    }

    for k in 0..9{
        if k%2 == 0 {
            continue
        }
        if k==7 {
            return
        }
        println!("===>val : {}",k);
    }
}














