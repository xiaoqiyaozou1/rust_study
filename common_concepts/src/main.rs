
const ONE_NUMBER:i32 = 100;

fn main() {
    println!("Hello, world!");
    println!("常量{}",ONE_NUMBER);

    let _x = 5; //不可变
    let mut _y = 6; //可变
    _y = 7;

    //变量遮挡
    let _xx = 1;
    let _xx = _xx + 1;
    {
        let _xx = _xx * 2; //变量的作用域 仅在此大括号内 
        println!("the _xx = {_xx}")
    }
    println!("the _xx = {_xx}");

    //值类型
    //整数 
    let n = 1;
    let m:i32  = -1;
    //浮点数
    let n = 1.0;

    //数字操作
    let sum = 1+1; //加
    let differnce  = 95.2 -40.1; //减

    // 乘
    let product = 4 * 30;

    // 除
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    //取余
    let remainder = 43 % 5;

    //bool
    let n = true;
    let f:bool = false;
    //char 
    let n = 'a';

    //复杂类型
    //元组
    let tuple:(i32,f64,char) =(1,2.0,'h');
    let one_value = tuple.0;
    let two_value = tuple.1;
    let three_value = tuple.2;
    println!("tuple is {one_value},{two_value},{three_value}");


    let add_res = add_one(3);
    println!("add_res = {add_res}"); 

    // if表达式
    let one_num = 1;
    if one_num ==1 {
        println!("one_num = {one_num}");
    }else if one_num == 2{
        println!("one_num != 1");
    }else{

    }

    // 在let 声明中 用if
    let condition = false;
    let is_true = if condition {true} else {false};
    println!("is_true is {is_true}");

    //loop 循环
    let mut _i = 1;
    loop {   
        if _i >10 {
            break;
            //break _i; //可以返回值
        }
        println!("_i = {_i}");
        _i= _i+1;
    }

    //while 循环
    while _i > 1{
        println!("_i = {_i}");
        _i-=1;
    }

    //for 循环
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..4).rev() {
        println!("{number}!");
    }
}



//函数

fn add_one(a:i32) -> i32 {
    a+1
}