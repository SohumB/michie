use caching::caching;

#[caching(key_expr = a)]
fn f(a: bool) -> bool {
    key
}

fn main() {}
