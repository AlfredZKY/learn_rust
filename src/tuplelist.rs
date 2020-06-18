use std::fmt;
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

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

pub fn use_tuple() {
    let v = List(vec![1, 2, 3, 4]);
    println!("{}", v);
}

pub fn use_list() {
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

pub fn analyze_slice(slice:&[i32]){
    println!("first element of the slice: {}",slice[0]);
    println!("the slice has {} element",slice.len());
}

pub fn use_slice(){
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

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 可以使用 let 把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;
    (boolean, integer)
}
