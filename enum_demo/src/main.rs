

enum IpAddrkind{
    v4,
    v6
}

// enum Option<T>{
//     None,
//     Some(T)
// }
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    type Operations = IpAddrkind;
    let my_v4 = IpAddrkind::v4;
    match my_v4{
        Operations::v4=>println!("我是v4"),
        Operations::v6=>println!("我是v6")
    }


    let some_number = Some(5);
    let some_char = Some('c');

    let res = add_one(some_number);
    match res {
        Some(res) => println!("结果是：{}",res),
        _ => ()
    }


    let config_max = Some(3u8);
    match config_max{
        Some(max) => println!("The max is {}",max),
        _=>()
    }
    //if let 写法
    if let Some(max) =config_max{
        println!("hehhe {}",max)
    }


    let coin = Coin::Penny;
    let quarter =Coin::Quarter(UsState::Alabama);
    let mut count = 0;

    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
   
    if let Coin::Quarter(state) = quarter {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }


    println!("Hello, world!");
}


fn add_one(num: Option<i32>) -> Option<i32>{
    match num{
        None => None,
        Some(i) => Some(i +1)
    }   
}
