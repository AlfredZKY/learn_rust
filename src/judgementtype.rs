// rust 不提供原生类型之间的隐式类型转换，但是可以使用as关键字进行显式类型转换
#![allow(overflowing_literals)]

pub fn use_type() {
    let _logical = true;
    let _a_float = 1.0;
    let an_integer = 5i32;
    let _default_float = 3.0;
    let default_integer = 7;
    println!(
        "a_float + default_integer = {}",
        (an_integer + default_integer)
    );
    println!("1 - 2 = {}", 1i32 - 2);

    // #[deny(arithmetic_overflow)]
    // println!("1 - 2 = {}",1u32 - 2); 类型不一致，不能操作
    let mut _inferred_type = 12;

    let mut _mutable = 12;

    // mutable = true; 变量类型不能改变

    // shadow 掩盖前面的变量
    let _mutable = true;

    // 短路求值的布尔类型
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:4b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:4b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:4b}", 0b0011u32 ^ 0b0101);
    // println!(" 1 << 5 is {}", 1i32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线改善数字的可读性
    println!("One million is writeen as {}", 1_000_000u32);
}

pub fn casting_type() {
    let decimal = 65.4321_f32;
    // let integer = decimal; 不能使用隐式转换

    // 可以显式转换
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 任何类型转换为无符号类型T时，会不断的加上和减去(std::T::MAX + 1) 直到值位于新类型T的范围内
    println!("1000 as a u16 is :{}", 1000 as u16); // 1000 已经在u16的范围内
    println!("1000 as a u8 is :{}", 1000i32 as u8);
    println!("-1000 as a u8 is :{}", (-1000i32) as u8);

    // 1000 - 256 - 256 - 256 = 232
    // 事实上的处理方式是：从最低有效位（LSB，least significant bits）开始保留
    // 8 位，然后剩余位置，直到最高有效位（MSB，most significant bit）都被抛弃。
    // 译注：MSB 就是二进制的最高位，LSB 就是二进制的最低位，按日常书写习惯就是
    // 最左边一位和最右边一位。

    println!("1000 mod 256 is : {}", 1000 % 256);
}

pub fn print_literal_size() {
    // 带有后缀的字面量，其类型在初始化时已经知道了
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面来，其类型取决于如何使用它们
    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {} ", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {} ", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {} ", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {} ", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {} ", std::mem::size_of_val(&f));
}

// from和into两个trait是内在联系着的，事实上这是它们的实现的重要部分，如果能把类型A转换成类型B 那么很容易相信我们可以把类型B 转换成类型A
use std::convert::From;
#[derive(Debug)]
pub struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

pub fn use_from_into() {
    let num = Number::from(30);
    println!("my number is {:?}", num);

    let int = 5;
    let num1: Number = int.into();
    println!("my number is {:?}", num1);
}

// 类似于from和into tryfrom和tryinto 是类型转换通用的trait ,不同与from/into的是，tryfrom和tryfrom用于易出错的转换，也正因为如此，其返回值是Result型
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

pub fn use_try_from_into() {
    // tryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
    println!("result is {:?}", EvenNumber::try_from(8));
    // tryInto
    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

// 任何类型想要转换成string 一般通过实现该类型的display方法的trait 它会自动提供tostring
use std::string::ToString;

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

pub fn use_tostring(){
    let circle = Circle{radius:6};
    println!("{}",circle.to_string());
}

// 解析字符串，把字符串转换成数字 通过parse完成 要转换的类型只要实现了fromstr 就可以
pub fn use_strparse(){
    let parsed:i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum : {:?} ",sum);
}
