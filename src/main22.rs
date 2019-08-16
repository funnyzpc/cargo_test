///
/// 生命周期(三)
///
fn main() {
    let x ;
    {
      let y = &6;
        let f = Foo{x:y};
        x = &f.x;
    }
    println!("=> {}",x);
}

struct Foo<'a> {
    x: &'a i32
}