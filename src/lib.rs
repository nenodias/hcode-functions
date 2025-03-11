pub fn somar(left: u64, right: u64) -> u64 {
    left + right
}

pub fn subtrair(left: u64, right: u64) -> u64 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn somar_it_works() {
        let result = somar(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works() {
        let result = subtrair(2, 2);
        assert_eq!(result, 0);
    }
}
