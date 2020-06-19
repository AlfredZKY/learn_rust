// 对指针来说，解构和解引用要区分开
// 解引用使用 *
// 解构使用& ref 和 ref mut

pub fn use_ref_pointer(){
    let reference = &4;
    match reference {   // 解构
        &vala => println!("Got a value via destructing: {:?}",vala),
    }

    // 如果不想使用&，可以先解引用
    match *reference {
        val => println!("Got a value via destructing: {:?}",val),
    }

    // 如果不使用& 创建一个引用，可以使用ref关键字来创建一个引用
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;
    match value {
        ref r => println!("Got a value via destructing: {:?}",r),
    }

    match mut_value {
        ref mut m => {
            // 已经获得了 mut_value 的引用，先要解引用，才能改变它的值
            *m += 10;
            println!("We added 10. `mut_value` : {:?}",m);
        }
    }
}