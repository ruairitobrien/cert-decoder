fn main() {
    println!("Hello, world!");
}

fn execute(args: Vec<String>) -> Result<(), ()> {
    if args.len() != 1 {
        return Err(());
    }
    Ok(())
}

#[cfg(test)]
mod test {

    use crate::execute;

    #[test]
    fn should_error_if_not_given_a_single_argument() {
        let args = Vec::new();
        let result = execute(args);
        assert!(result.is_err());
    }
}
