
fn main(){
    let  f:fn(i32) -> i32 = plus;
    println!("value : {}",f(22));

    let f2 = plus;
    println!("value: {}",f2(23));
}

fn plus(i:i32)->i32{
    i+2
}