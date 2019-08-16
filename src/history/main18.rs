///
/// 作用域
///

fn main() {
    // process01();
    // process02();
    // process03();
    // process04();
    process05();
}

// 作用域 测试五
fn process05(){
    // 初始化
    let  y:&i32;
    let x = 5;
    y = &x;
    println!("process05 x:{}“ y:{}",x,y);
}

// 作用域 测试四
/*
fn process04(){
    let y:&i32;
    {
        let x:i32 = 5;
        // y在x之前被声明，意味着y比x生命周期更长，这是不允许的。(这里会报错)
        y = &x;
        println!("x:{} y:{}",x,y);
    }
    println!("y:{}",y);
}
*/

// 作用域 测试三
/*
fn process03(){
    let mut v:Vec<f32> = vec![1.100,2.3,5.01,9.90199];

    for i in &v{
        println!("process03 > {}“",i);
        // 引用后写入会抛错
        // v.push(2.2222);
    }
}
*/

// 作用域 测试二
/*
fn process02(){
    let mut x:i32 = 1;
    {
        let y: &mut i32 = &mut x;
        *y += 1;
        println!("y:{}",y);
    }
    println!("x:{}",x);
}
*/

// 作用域 测试一
/*
fn process01(){
    let mut x:i32 = 1;
    let y:&mut i32 = &mut x;
    *y += 1;
    // println!("x:{} y:{}",x,y);
    println!(" y:{}",y);
}
*/


