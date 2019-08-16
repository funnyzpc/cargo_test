
///
/// 所有权
///
fn main(){
    // 测试一
    println!("\n===>测试一<===");
    let mut x:Vec<char> = vec!['a','b','c'];
    x.push('d');
    println!("> {}",x[x.len()-1]);

    let  mut y = x;
    // println!("===>vv :{}",x[0]);
    println!("y> {}",y[y.len()-1]);
    y.push('h');

    println!("y> {}",y[y.len()-1]);

    // 测试二
    println!("\n===>测试二<===");
    let z = vec!['j','k','l'];
    take(z);
    // println!("process take() function after :{}",z[z.len()-1]);

    // 测试三
    println!("\n===>测试三<===");
    let v1 = vec!['a','a','k'];
    let v2 = &v1;
    println!("v1[2] = {}",v1[2]);
    println!("v2[2] = {}",v2[2]);

    // 测试四
    println!("\n===>测试四<===");
    let mut m = vec![2,3,4,5];
    m.truncate(2);// 截取数组
    println!("m> len:{} max_value:{}",m.len(),m[m.len()-1]);

    // 测试五
    println!("\n===>测试五<===");
    let mut a:i32 = 1;
    let b = a;
    a = 3;
    let c = b;
    println!("> a:{} b:{} c:{}",a,b,c);

    // 所有权 测试六
    println!("\n===>测试六<===");
    let a  = 6;
    let _y = double(a);
    println!("val : _y:{} a:{}",_y,a);

    // 所有权 测试七
    println!("\n===>测试七<===");
    let c  = true;
    let _c  = change(c);
    println!("val c:{} _c:{}",c,_c);

    // 所有权 测试八
    println!("\n===>测试八<===");
    let  j:Vec<char> = vec!['a','h','k','i','m'];
    let k = &j;
    println!("j:{} k:{}",j[j.len()-1],k[k.len()-1]);
}

fn change(x:bool)->bool{
    !x
}
fn double(x:i32)->i32{
    x*3
}



fn take(v:Vec<char>){
    println!("process take() function :{}",v[v.len()-1]);
}











