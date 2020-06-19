pub fn use_match(){
    let number = 13;
    println!("Tell me about {}",number);

    match number{
        // 匹配第一个值
        1 => println!("One"),
        2|3|5|7|11 => println!("This is a prime"),
        13..=19=>println!("A teen"),
        _ => println!("Ani't special"),
    }

    let boolean = true;
    let binary = match boolean {
        // match 分支必须覆盖所有可能的值
        false => 0,
        true => 1,
    };
    println!("{} -> {}",boolean,binary);
}

pub fn use_match_tuple(){
    let pair = (1,-2);
    println!("Tell me about {:?}",pair);
    match pair{
        (0,y) => println!("First is `0` and `y` is `{:?}`",y),
        (x,0) => println!("`x` is `{:?}` and last is `0`",x),
        _ => println!("It doesn't matter what they are"),   // _ 不将变量进行绑定
    }
    let pair = (2,-2);
    match pair {
        (x,y) if x == y => println!("these are twins"),
        (x,y) if x + y == 0 => println!("Antimatter,kaboom"),
        (x,_) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}