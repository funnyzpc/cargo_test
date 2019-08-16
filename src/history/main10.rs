fn process(val:i32)->i32{
    val+99
}


fn main(){
    // 这是行注释
    let rst = process(99);
    /// 这是文档注释
    /// 这也是文档注释
    //|! 这是根文件(或模块文件)注释，这里不可使用
    print!("rst:{}",rst);
}