
///
/// 生命周期(二)
///
fn main() {

    let line = "lang:en=Hello youth!";
    let lang = "en";
    let v;
    {
        let p = format!("lang：{}",lang);
        v = skip_prefix(line,p.as_str());
    }


    println!("v = > {}",v);
}

// 标注生命周期
fn skip_prefix<'a,'b>(line:&'a str ,prefix:&'b str) -> &'a str{
    // return concat!(line,prefix);
    // return lineprefix;
    // return "success";
     return format!("{} {}",line,prefix).as_str();
    // return line + " " + prefix;
}