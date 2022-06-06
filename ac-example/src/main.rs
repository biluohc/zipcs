extern crate ac_example;

use ac_example::Acs;

fn main() {
    println!("Hello, proc_macro2");
    println!("\tusize: {}", Acs::ac_usize().get());
    println!("\tu64: {}", Acs::ac_u64().get());
    println!("\tu32: {}", Acs::ac_u32().get());
    println!("\tu16: {}", Acs::ac_u16().get());
    println!("\tu8 : {}", Acs::ac_u8().get());
    Acs::ac_usize().add(1);
    // println!("{}", Acs::skip_i8().get());

    let acs = Acs::load();
    println!("{}", serde_json::to_string(&acs).unwrap());
}