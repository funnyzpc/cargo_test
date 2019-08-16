
///
/// 生命周期
///
fn main() {
    // process01();
    // process02();
}

// 测试01
fn process01(){
    let r;
    // println!("before r:{}",r);
    let i = 1;
    r = & i;
    println!("r:{} i:{}",r,i);
}

// 测试02
fn process02(){
    let r;
    {
        let i = 1;
        r = &i;
        // i的作用域只在代码块内 故赋值后r没有足够的存活时间
        println!("i:{}",i);
    }
    println!("r:{}",r);
}