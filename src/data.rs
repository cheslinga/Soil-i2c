extern crate mysql;

pub mod d_mysql {
    //init: Initializes a pooled connection with a MySQL server and returns it
    //to be used for inserts, etc.
    //
    pub fn init(url: &str) -> mysql::PooledConn {
        use data::mysql::*;
        use data::mysql::prelude::*;

        let pool = match Pool::new(url) {
            Ok(m) => m,
            Err(err) => panic!("Error connecting to MySQL server! {}", err),
        };
        
        let conn = pool.get_conn().unwrap();
        return conn;
    }
    //insert: Takes a pooled connection and drops my standard insert statement
    //into it. You could modify it based on what your database looks like/what
    //you want to see.
    //
    pub fn insert(conn: &mut mysql::PooledConn, plname: String, tempwrite: f32, moistwrite: u16) -> Result<(), mysql::Error> {
        use data::mysql::*;
        use data::mysql::prelude::*;

        match conn.exec_drop(
            r"INSERT INTO SoilData (Plant,Readtime,Moisture,Temperature) VALUES (?,CURRENT_TIME,?,?)", (plname, moistwrite, tempwrite)
        ) {
            Ok(m) => m,
            Err(err) => println!("Error processing MySQL query! {}", err),
        };

        Ok(())
    }
}
