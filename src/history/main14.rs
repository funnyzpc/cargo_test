///
/// 动态数组: Vectors
///
fn main(){
    // 形式一
    let v = vec![1,2,33,5];

    for item in v{
        println!("=>val :{}",item);
    }
    // 形式二
    let mut e = vec![0;5];
    e[2] = 99;
    println!("val : {}",e[2]);

    // 索引(不能使用有符号int做索引)
    let v:Vec<i32> = vec![1,3,7,9,11];
    let i:usize = 0;
    // let j:i32 = 0;
    println!("v[i] = {}",v[i]);
    // println!("v[j] = {}",v[j]);

    // 匹配
    match v.get(8) {
        Some(x) => println!("value is {}",x),
        None => println!("Sorry,never have this value!")
    }

    // 遍历 (多次遍历必须使用引用的方式)
    println!("\n=====>遍历<=====");
    let vr:Vec<char>  = vec!['a','b','c','e'];
    for i in &vr{
        println!("遍历01 > {}",i);
    }

    for k in &vr{
        println!("遍历02 > {}",k);
    }
}















