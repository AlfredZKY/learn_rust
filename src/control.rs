#![allow(unreachable_code)]

pub fn use_look_label() {
    'outer: loop {
        println!("Entered the other loop");
        'inner: loop {
            println!("Entered the inner loop");
            // 内部循环中断
            break;
        }
        // 中断外层循环
        break 'outer;
        println!("this point will nerver be reached");
    }
    println!("Exited the outer loop");
}

pub fn loop_return() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}

pub fn use_while() {
    // 计数器变量
    let mut n = 1;

    // 当 n< 101 时循环
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器累加
        n += 1;
    }
}

pub fn use_for1() {
    // 此时代表左闭右开 两端都闭合加上一个等号即可
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

pub fn use_for2(){
    let names = vec!["Bob","Frank","Ferris"];
    // 把集合转换为迭代器
    // iter在每次迭代时，会借用集合中一个元素，集合本身并不改变
    for name in names.iter() {
        match name {
            &"Feeris" => println!("There is a rustacean among us!"),
            _=> println!("Hello {}",name),
        }
    }

    println!("{:#?}",names);

    let mut names = vec!["Bob","Frank","Ferris"];
    // iter_mut 可变地 借用集合中的每个元素，从而允许集合被就地修改
    for name in names.iter_mut(){
        *name = match name {
            &mut "Ferris" => "This is a rustacean among us!",
            _=> "Hello",
        }
    }
    println!("{:#?}",names);

    let names = vec!["Bob","Frank","Ferris"];
    // into_iter 会消耗集合，在每次迭代中，集合中的数据本身会被提供，集合一旦被消耗了，之后就在无法使用，因为已被移除
    for name in names.into_iter(){
        match name {
            "Feeris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}",name),
        }
    }
    // 集合中元素已经被移除完毕了
    // println!("{:#?}",names);
}
