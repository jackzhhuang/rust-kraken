mod query_kraken;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = query_kraken::get_balance();
        println!("result: {:#?}", result);
    }
}
