use mysql::{Pool, PooledConn};

mod user;

pub fn get_connection() -> PooledConn {
    let url = "mysql://tony:ZMwan1314!@localhost:3306/rainmeter";
    let pool = Pool::new(url);
    match pool {
        Ok(_) => {}
        Err(..) => {
            panic!("创建数据连接池失败, 请检查数据库连接配置")
        }
    }
    let mut conn = pool.unwrap().get_conn();
    match conn {
        Ok(conn) => { return conn; }
        Err(..) => {
            panic!("创建连接池失败,请检查数据库连接配置");
        }
    }
}

#[cfg(test)]
mod test {
    use crate::model::get_connection;

    #[test]
    fn it_work() {
        let conn = get_connection();
        assert_ne!(0, conn.connection_id());
    }
}