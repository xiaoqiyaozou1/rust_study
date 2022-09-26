//1、每个值拥有的一个所有者
//2、一次只能有一个所有者
//3、当离开作用域 值会被释放

fn main() {
    /************所有权的基本概念****************/
    {
        let s = "hello"; //此处有效
    }
    //此处无效

    //字符串作为引用类型 rust 对其会有特殊操作
    let str1 = String::from("hello");
    let str2 = str1;
    // println!("{},{}",str1,str2); str1 被借用 是无法再被访问的 两者是指向同一块堆地址  
    let num1 = 2;
    let num2 = num1;
    println!("{},{}",num1,num2);//值类型  直接深度复制 两者同时有效

    /************引用和借用***************/

    let str3 = String::from("nihao");
   // let str3_len = calculate_lenght(str3); //这种方式 使用过str3 之后 str3将被释放
    let str3_len = calculate_lenght2(&str3); //引用的方式 再栈上再创建新的值 指向str3 栈上 存放信息的位置 通过引用的方式 使用  引用的值是不能直接更改的
    println!("{}",str3_len);

    let mut str4 = String::from("change");  //字符串设置成可变 方法 参数设置成可变引用 然后可以对字符串进行修改
    change_str(&mut str4);
    println!("{}",str4);



    /************************slice*****************************/

    let str5 = String::from("hello");
    
    let len = str5.len();
    
    let slice = &str5[3..len];
    let slice = &str5[3..];

    


    println!("Hello, world!");
}

fn change_str(str:&mut String){
    str.push_str(", world");
}

fn calculate_lenght(s:String)->usize{
    s.len()
}
fn calculate_lenght2(s:&String)->usize{
    s.len()
}

