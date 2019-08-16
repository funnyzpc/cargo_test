///
/// 结束循环 标签循环
///
fn main(){
    println!("\n===>start");
    // return 循环
    let arr:[char;6]=['a','b','c','d','*','哈'];
    // let arr:[&str;6]=["a","b","c","d","*","哈"];
    for item in &arr{
        for val in 0..3{
            println!("current value : {}-{}",item,val);
            if item == &'*' && val == 1{
                println!("break current value : {}-{}",item,val);
                break;
            }
        }
    }
    println!("===>end");

    // 循环标签
    println!("\n===>label START");
    let j:[&str;6]=["a","b","c","d","*","哈"];
    'outer: for item in &j{
         for val in 1..4{
            if item == &"*" &&  val ==2{
                // 此处不可使用return否则无法运行至'END'
                break 'outer;
            }
            println!("label value: {}-{}",item,val);
        }
    }
    println!("===>label END");

}
