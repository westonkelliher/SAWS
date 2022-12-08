mod conn;
mod server;

pub use conn::{Conn, ConnId};
pub use server::Server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
