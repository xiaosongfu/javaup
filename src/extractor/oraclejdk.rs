use super::spec::Extract;

pub struct OracleJdk {}

impl Extract for OracleJdk {
    fn extract(self) -> ! {
        todo!()
    }
}

impl OracleJdk {
    pub fn hello_world(self) {
        println!("Hello, world!");
    }
}
