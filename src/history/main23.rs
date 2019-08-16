
///
/// 生命周期(四)
///
fn main() {
    let x = "mine";
    let r1 = substr1(&x,77);
    println!("\nr1 = {}",r1);

    let r2 = substr2(&x,88);
    println!("\nr2= {}",r2);
}

fn substr1<'a>(s: &'a str, until: u32) -> &'a str{
    return format!("s:{},until:{}" ,s,until).as_str();
    // return "hello";
} // elided
fn substr2<'a>(s: &'a str, until: u32) -> &'a str{
    return "youth";
} // expanded
