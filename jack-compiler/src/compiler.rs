pub mod tokenizer;

pub fn compile<S>(input: S) -> Vec<String>
where
    S: AsRef<str>,
{
    let _input: &str = input.as_ref();
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler() {
        compile("");
    }
}
