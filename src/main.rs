fn use_std_fmt() {
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

fn use_struct() {
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

use std::fmt;
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

fn use_display() {
    let structs = Structre(4);
    println!("Structre is {} ", structs);
}

struct List(Vec<i32>);
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个`vec`的引用
        let vec = &self.0;
        write!(f, "[")?;

        // 使用`v` 对`vec`进行迭代，并用`count`记录迭代次数
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素加逗号(第一个元素除外)
            // 使用`?` `try!` 来返回错误
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}:{}", count, v)?
        }
        write!(f, "]")
    }
}

fn use_tuple() {
    let v = List(vec![1, 2, 3, 4]);
    println!("{}", v);
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

fn use_city() {
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
}

fn use_type() {
    let logical = true;
    let a_float = 1.0;
    let an_integer = 5i32;
    let default_float = 3.0;
    let default_integer = 7;
    println!(
        "a_float + default_integer = {}",
        (an_integer + default_integer)
    );
    println!("1 - 2 = {}", 1i32 - 2);

    // #[deny(arithmetic_overflow)]
    // println!("1 - 2 = {}",1u32 - 2); 类型不一致，不能操作

    let mut inferred_type = 12;
    inferred_type = 4294967296i64;
    let mut mutable = 12;
    mutable = 21;

    // mutable = true; 变量类型不能改变

    // shadow 掩盖前面的变量
    let mutable = true;

    // 短路求值的布尔类型
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    println!("0011 AND 0101 is {:4b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:4b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:4b}", 0b0011u32 ^ 0b0101);
    println!(" 1 < 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 使用下划线改善数字的可读性
    println!("One million is writeen as {}", 1_000_000u32);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 可以使用 let 把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;
    (boolean, integer)
}
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn use_list() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // 通过元组的下标来访问具体的值
    println!("long tuple first value: {}",long_tuple.0);
    println!("long tuple second value: {}",long_tuple.1);

    let tuple_of_tuples = ((1u8,2u16,2u32),(4u64,-1i8),-2i16);
    println!("tuple of tuples: {:?}",tuple_of_tuples);
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13); 元组太长，无法打印
    // println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1,true);
    println!("pair is {:?}",pair);
    println!("the reversed pair is {:?}",reverse(pair));
    
    // 创建单元素元组需要一个额外的逗号，这个为了和被括号包含的字面量作区分
    println!("one element tuple: {:?}",(5u32,));
    println!("just an integer: {:?}",(5u32));
    
    let tuple = (1,"hello",4.5,true);
    let (a,b,c,d) = tuple;
    println!("{:?},{:?},{:?},{:?}",a,b,c,d);
    let matrix = Matrix(1.1,1.2,2.1,2.2);
    println!("{:?}",matrix);
}

fn analyze_slice(slice:&[i32]){
    println!("first element of the slice: {}",slice[0]);
    println!("the slice has {} element",slice.len());
}

fn use_slice(){
    use std::mem;

    let xs:[i32;5]=[1,2,3,4,5];
    let ys:[i32;500]=[0;500];

    println!("first the element of the array: {}",xs[0]);
    println!("second the element of the array: {}",xs[1]);

    println!("array size: {}",xs.len());

    // 数组是在栈中分配的
    println!("array occupies {} bytes",mem::size_of_val(&xs));

    analyze_slice(&xs);
    analyze_slice(&ys[1..4]);
}

fn main() {
    println!("Hello, world!");
    use_std_fmt();
    use_struct();
    use_display();
    use_tuple();
    // #[derive(Debug)]
    // struct List(Vec<i32>);
    // let v = List(vec![1,2,3,4]);
    // println!("{:#?}",v);
    use_city();
    use_type();
    use_list();
    use_slice();
    let padded_piece_size = 64u32;
    pub const NODE_SIZE: usize = 32;
    let parts = (padded_piece_size as f64 / NODE_SIZE as f64).ceil() as usize;
    println!("parts is {}",parts);
}
