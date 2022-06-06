extern crate ac_example;

use ac_example::Acs;

fn main() {
    println!("Hello, proc_macro2");
    println!("\tusize: {}", Acs::ac_usize().load());
    println!("\tu64: {}", Acs::ac_u64().load());
    println!("\tu32: {}", Acs::ac_u32().load());
    println!("\tu16: {}", Acs::ac_u16().load());
    println!("\tu8 : {}", Acs::ac_u8().load());
    Acs::ac_usize().add(1);
    // println!("{}", Acs::skip_i8().load());

    let acs = Acs::clear();
    println!("{}", serde_json::to_string(&acs).unwrap());
}
