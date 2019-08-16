
///
///结构体
///
fn main() {
    // 简单结构体
    println!("\n===>简单结构体");
    let g  = Grap{x:11,y:2222222};
    println!("g.x:{} g.y:{}",g.x,g.y);

    // 实例绑定可变
    println!("\n===>实例绑定可变");
    let mut f = Grap{x:99,y:222};
    f.x = 66666;
    println!("f.x:{} f.y:{}",f.x,f.y);
    f.x = 5555555;
    f.y = 7777777;
    println!("f.x:{} f.y:{}",f.x,f.y);

    // 实例参数指针
    println!("\n===>实例参数指针");
    let mut a = Grap{x:11,y:222};
    println!("a before x:{} y:{}",a.x,a.y);
    {
        // let b = Grap2{x:&mut 999,y:&mut 333};
        let b = Grap2{x:&mut a.x,y:&mut a.y};
        println!("b before x:{} y:{}",b.x,b.y);
        *b.x =  999;
        *b.y = 444;
        println!("b after x:{} y:{}“",b.x,b.y);
    }
    println!("a after x:{} y:{}",a.x,a.y);

    // 结构体参数更新
    println!("\n===>结构体参数更新");
    let mut a = Grap{x:1,y:2};
    println!("a before x:{} y:{}",a.x,a.y);
    a =  Grap{x:123,..a};
    println!("a after x:{} y:{}",a.x,a.y);

}

struct Grap{
    x:i32,
    y:i64,
}

struct Grap2<'a>{
    x:&'a mut i32,
    y:&'a mut i64,
}