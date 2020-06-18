pub fn use_std_fmt() {
    // 通常情况下，'{}'会被任意变量内容所替换
    // 变量内容会转化成字符串
    // 不加后缀的话 31就自动成为i32 类型，通过后缀可以改变类型
    println!("{} days!", 31u32);

    // 变量替换字符串 也可以用位置参数
    println!("{0},this is {1}. {1}, this is {0} ", "Allice", "Bob");

    // 可以使用命名参数
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jump over"
    );

    // 可以在 ":" 后面指定特殊的格式
    println!("{} of {:b} people know binary,the other half don't", 1, 2);

    // 指定宽度来右对齐文本
    println!("{number:>width$}", number = 1, width = 6);

    // 在数字左边补0
    println!("{number:>0width$}", number = 1, width = 6);

    // println! 会检查使用到的参数数量是否正确
    println!("My name is {0},{1} {0}", "Bond", "James");

    // 创建一个包含单个 'i32'的结构体(structre) 命名为 'Structre'
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Structre(i32);
    println!("Structre is {:#?}", Structre(3));
    // use::std::fmt::Display("Hello");
    format!("Hello");
}