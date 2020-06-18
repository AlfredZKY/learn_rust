// 该属性用于隐藏对未使用代码的警告
#![allow(dead_code)]

use std::fmt;

#[derive(Debug)]  // 带有生命周期的结构体
pub struct Person<'a>{
    pub name:&'a str,
    pub age:u8,
}

struct Structre(i32);

// 为了使用'{}' 标记 实现Structre 手动为类型实现 fmt::Display trait
impl fmt::Display for Structre {
    // 这个trait要求 `fmt`使用与下面的函数完全一致的函数签名
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 仅将self的第一个元素写入到给定的输出流`f` 返回`fmt::Result`
        // 此结果表明操作成功或失败，
        write!(f, "{}", self.0)
    }
}

pub fn use_display() {
    let structs = Structre(4);
    println!("Structre is {} ", structs);
}

pub fn use_struct() {
    #[derive(Debug)]
    struct Structre(i32);

    #[derive(Debug)]
    struct Deep(Structre);

    println!("Now {:?} will print!", Structre(3));
    println!("Now {:?} will print", Deep(Structre(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        write!(
            f,
            "{}:{:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

pub fn use_city() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", city)
    }
    
    let city = City {
        name: "Dublin",
        lat: 53.347778,
        lon: -6.259722,
    };
    // 通过实现display 方法，间接实现了to_string方法
    println!("city is {}",city.to_string());
}

pub enum WebEvent{
    PageLoad,
    PageUnLoad,
    KeyPress(char),
    Paste(String),
    CLick{x:i64,y:i64}
}

pub fn inspect(event:WebEvent){
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnLoad => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}' .", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".",s),
        WebEvent::CLick{x,y} => {
            println!("clicked at x={},y={} .",x,y);
            println!("i think x is {},y is {}",x,y)
        },
    }
}

pub fn use_enum(){
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my test".to_owned()); // to_owned() 从一个字符串切片创建一个具有所有权的 String
    let click = WebEvent::CLick{x:20,y:30};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnLoad;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}


// 拥有隐式辨别值 (implicit discriminator 从0开始)
enum Number {
    Zero,
    One,
    Two,
    Ss,
}

pub fn use_implicit(){
    println!("zero is {} ",Number::Zero as i32);
    println!("one is {} ",Number::One as i32);
    println!("Ss is {} ",Number::Ss as i32);
}