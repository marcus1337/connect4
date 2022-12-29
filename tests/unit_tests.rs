
#[path = "../src/lib.rs"]
mod lib;

#[cfg(test)]
mod tests {    
    use super::*;

    #[test]
    fn it_works() {
        let result = lib::sub(2, 2);
        assert_eq!(result, 0);
    }
}