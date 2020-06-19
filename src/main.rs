pub mod mystructs;
pub mod mystd;
pub mod tuplelist;
pub mod judgementtype;
pub mod fileop;
pub mod control;
pub mod matchs;
pub mod refpointer;

extern crate time;
use time::*;


fn main() {
    println!("Hello, world!");
    mystd::use_std_fmt();
    mystructs::use_struct();
    mystructs::use_display();
    tuplelist::use_tuple();
    // #[derive(Debug)]
    // struct List(Vec<i32>);
    // let v = List(vec![1,2,3,4]);
    // println!("{:#?}",v);
    mystructs::use_city();
    judgementtype::use_type();
    tuplelist::use_list();
    tuplelist::use_slice();
    let padded_piece_size = 64u32;
    pub const NODE_SIZE: usize = 32;
    let parts = (padded_piece_size as f64 / NODE_SIZE as f64).ceil() as usize;
    println!("parts is {}",parts);
    let name = "Peter";
    let age = 27;
    
    use mystructs::*;
    use mystructs::WebEvent::*;
    let webevent = KeyPress('x');
    inspect(webevent);

    let _peter = mystructs::Person{name,age};
    mystructs::use_enum();
    mystructs::use_implicit();
    judgementtype::casting_type();
    judgementtype::print_literal_size();
    judgementtype::use_from_into();
    judgementtype::use_try_from_into();
    judgementtype::use_tostring();
    judgementtype::use_strparse();
    let start = time::now();
    // fileop::init();
    // fileop::sort_by_self(0, 0);
    let end = time::now();
    println!("duration: {:?}",end-start);

    control::use_look_label();
    control::loop_return();
    control::use_while();
    control::use_for1();
    control::use_for2();
    matchs::use_match();
    matchs::use_match_tuple();
    refpointer::use_ref_pointer();
}
