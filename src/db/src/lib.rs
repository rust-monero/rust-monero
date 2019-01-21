#[macro_use]
extern crate log;

pub mod blockchain_db;
pub mod db_lmdb;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        info!("test1");
        warn!("test2 {}", "monero");
    }
}
