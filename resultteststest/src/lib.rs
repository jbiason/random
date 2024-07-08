fn not_zero(num: usize) -> Result<(), ()> {
    if num == 0 {
        Err(())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_result_ok() -> Result<(), ()> {
        Ok(())
    }

    #[test]
    fn with_result_err() -> Result<(), ()> {
        Err(())
    }

    #[test]
    fn nested() -> Result<(), ()> {
        not_zero(1)?;
        Ok(())
    }

    #[test]
    fn nested2() -> Result<(), ()> {
        not_zero(0)?;
        Ok(())
    }

    #[test]
    fn nested3() -> Result<(), ()> {
        let result = (|| {
            not_zero(2)?;
            Ok(())
        })();

        // do something else, like cleaning up the testing workspace

        result
    }
}
