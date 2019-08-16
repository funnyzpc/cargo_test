
fn main() {
    let mut x = 10;
    let mut done  = false;

    let y:f32 = 0.1;
    let z:f32 = 0.2;
    let k:f32 = y + z ;
    println!("===>{}",0.1+0.2);
    println!("===>{}",y+z);
    println!("===>{}",k);

    while !done {
        x = x -1;
        println!("=>{}",x);
        /*
        if  x%5 == 0 {
            done = true;
        }
        */
        if x == 0{
            done = true;
        }
    }
}