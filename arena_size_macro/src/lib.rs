extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn arena_size(_i:TokenStream) -> TokenStream{
    // let a = env!("efearena_size");
    let a = std::env::var("efearena_size").unwrap_or("1024".to_string());
    let s = format!("pub(crate) const ARENA_SIZE:usize = {};",a);
    s.parse().unwrap()
}