/*!

# References
* https://github.com/upsuper/custom-derive-2019/blob/master/script.zh.md
* https://crates.io/crates/proc-macro2
* https://crates.io/crates/syn
* https://crates.io/crates/quote
* https://crates.io/crates/heck
* https://crates.io/crates/darling

*/
#[macro_use]
extern crate ac;
#[macro_use]
extern crate serde;

#[derive(Ac, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[ac(skip_load = true, skip_clear = false)]
pub struct Acs {
    #[ac(default = 1)]
    ac_usize: usize,
    ac_u64: u64,
    ac_u32: u32,
    ac_u16: u16,
    ac_u8: u8,
    #[ac(skip = true)]
    skip_i8: i8,
}
