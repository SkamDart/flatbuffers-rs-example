#[allow(dead_code, unused_imports)]
pub mod gen {
    include!(concat!(env!("OUT_DIR"), "/", "hello_generated.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
