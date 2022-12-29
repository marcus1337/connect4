
#[path = "../src/lib.rs"]
mod lib;

#[cfg(test)]
mod tests {    
    use super::*;
    use super::lib::lib_state;

    #[test]
    fn it_works() {
        let result = 0;
        assert_eq!(result, 0);
    }

}