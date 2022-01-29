mod parser;


#[cfg(test)]
mod tests {
    use crate::parser;

    #[test]
    fn parsing_works() {
        parser::start(Some(2));
    }
}
