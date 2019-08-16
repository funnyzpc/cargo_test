
///
/// 引用 (&mut)
///     只有被标注为mut的可变参数 才可使用&mut进行修改
///     只能借用一次，否则会同时对同一个内存位置进行更改
///     不能同时存在可变引用与不可变引用
///

fn main() {
    // 测试一
    println!("\n===>测试一<===");
    let mut x = 5;
    {
        let  y = &mut x;
        *y += 1;// 使用*号可访问被引用的内容
        println!("y: {}",y);
    }
    println!("x:{}",x);

    // 测试二
    println!("\n===>测试二<===");
    let mut j = 7;
    // 以下只能借用一次，否则会同时对同一个内存位置进行更改
    quote(&mut j);
    quote2(&mut j);
    println!("j:{}",j);
}

fn quote(val:&mut i32){
    let k =  val;
    *k += 1;
    println!("k1:{}",k);
}
fn quote2(val:&mut i32){
    let k =  val;
    *k += 1;
    println!("k2:{}",k);
}