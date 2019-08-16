///
/// 引用和借用
/// 借用:(使用指针，对象参数不可变)
///

fn main() {

    fn sum_vec(v:&Vec<i32>) -> i32{
        return v.iter().fold(0,|a,&b| a+b);
    }

    fn foo(v1:&Vec<i32>,v2:&Vec<i32>) -> i32{
        /*
        let s1:i32 = sum_vec(v1);
        let s2:i32 = sum_vec(v2);
        s1+s2
        */
        return  sum_vec(v1) +sum_vec(v2);
    }

    let v1 = vec![100,20,5];
    let v2:Vec<i32> = vec![200,10,9];

    let v1_sum:i32 = sum_vec(&v1);
    let v2_sum:i32 = sum_vec(&v2);

    println!("v1 sum:{}, v2 sum:{}",v1_sum,v2_sum);

    println!("v1 + v2 sum : {}",foo(&v1,&v2));
}