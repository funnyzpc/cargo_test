
///
/// 生命周期(三)
///
fn main() {
    let y = &9;// eq let _y = 5; let y = &_y;
    let f = Foo{x:y};

    println!("=> {}",f.x);

}

struct Foo<'a>{
    x:&'a i32
}

impl<'a> Foo<'a>{
    fn x(&self) -> &'a i32{self.x}
}