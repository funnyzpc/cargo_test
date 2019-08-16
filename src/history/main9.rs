
fn main(){
    println!("===>hello youth!<=====");

    // 布尔类型
    let x = true;
    let y:bool = false;
    println!("\r\n===> x:{} y:{}",x,y);

    // char类型
    let a  = 'X';
    let b:char = '表';
    let c:char = '💕';
    println!("\r\n===>a :{} b:{} c:{}",a,b,c);

    // 数字类型
    let d = 42;
    // let e = 23.4;
    let f:i8 = 127;
    // let g:f32 = -1.1111111111111119;
    println!("\n===>d:{} f:{}",d,f);

    // 数字无符号(仅整数)
    let h:u8 = 122;
    let i:u128 = 8078298727982999179998691987498172468;
    println!("\n===>h:{} i:{}",h,i);

    // 可变大小类型(任何时候都要做类型转换)
    let j:isize = 987424499;
    let k:usize = 824744449;
    println!("\n===>可变 j:{} k:{}",j,k);

    // 浮点类型
    let l:f32 = 8888.0;
    let m:f64 = 99999888822227777.0;
    println!("\n===>浮点 l:{} m:{}",l,m);

    // 数组(可变与不可变)
    let a1:[i32;5] = [1,2,3,4,5];
    let mut a2 = [6,7,8,9,10,222];
    let a3:[f64;3] = [7.777;3];
    println!("\n数组 a1:{} a2:{}",a1[1],a2[5]);
    println!("\n数组默认值 a3:{}",a3[2]);
    a2[5] = 888;
    println!("\n数组 a1:{} a2:{}",a1[1],a2[5]);
    println!("\n数组长度 a1:{} a2:{}",a1.len(),a2.len());

    // 切片
    let b1 = [0,1,2,4,7,122,566,709];
    let b2 = &b1[..];
    let b3 = &b1[1..4];
    println!("\n===>b1:{} b2:{} b3:{}~{}",b1[0],b2[0],b3[0],b3[2]);

    // 字符串
    let mut c1 = "hello";
    const C2:&str = "你好";
    println!("\n===>字符串 c1:{} c2:{}",c1,C2);
    c1 = "youth";
    println!("\n===>字符串 c1:{} c2:{}",c1,C2);

    // 元组
    let d1 = (1,"hello");
    let d2 = (2,true,"youth");
    let  d3:(i32,&str);
    d3 = d1;
    let (d4,) = (1.22,);
    println!("\n===>d1==d3:{} {} {} {}",d1==d3,d2.0,d2.2,d4);

    // 函数(输入类型、输出类型、实例名称)
    let f2:fn(i32) -> i32 = foo;
    println!("\n函数 foo:{} f2:{}",foo(777),f2(888));
}
fn foo(f1:i32) -> i32{f1+2}
















