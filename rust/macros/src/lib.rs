#[macro_export]
macro_rules! hashmap {
    ( $( $k:expr => $v:expr, )* ) => {{
       let mut hm = HashMap::new();
       $(
           hm.insert($k, $v);
       )*
       hm
    }};
    ( $( $k:expr => $v:expr ),* ) => {{
       let mut hm = HashMap::new();
       $(
           hm.insert($k, $v);
       )*
       hm
    }};
}
