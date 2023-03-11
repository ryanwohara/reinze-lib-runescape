use dotenv::dotenv;
// use mysql::prelude::*;
use mysql::*;

pub fn connect() -> std::result::Result<PooledConn, Error> {
    get_connection(&get_connection_string())
}

fn get_connection(url: &str) -> std::result::Result<PooledConn, Error> {
    let pool = Pool::new(url)?;

    pool.get_conn()
}

fn get_connection_string() -> String {
    dotenv().ok();

    let host = std::env::var("DB_HOST").expect("DB_HOST must be set");
    let port = std::env::var("DB_PORT").expect("DB_PORT must be set");
    let user = std::env::var("DB_USER").expect("DB_USER must be set");
    let pass = std::env::var("DB_PASS").expect("DB_PASS must be set");
    let db = std::env::var("DB_NAME").expect("DB_NAME must be set");

    let mut url = String::new();
    url.push_str("mysql://");
    url.push_str(&user);
    url.push_str(":");
    url.push_str(&pass);
    url.push_str("@");
    url.push_str(&host);
    url.push_str(":");
    url.push_str(&port);
    url.push_str("/");
    url.push_str(&db);

    url
}

// fn connect() -> Result<Connection, Error> {
//     let mut config = Config::default();
//     config.set_group(Some("client".to_owned()));
//     config.set_file(Some(PathBuf::from("config.ini")));
//     let config = config.read().unwrap();

//     let host = config.get_str("host").unwrap();
//     let port = config.get_int("port").unwrap();
//     let user = config.get_str("user").unwrap();
//     let pass = config.get_str("pass").unwrap();
//     let db = config.get_str("db").unwrap();

//     let mut url = String::new();
//     url.push_str("mysql://");
//     url.push_str(user);
//     url.push_str(":");
//     url.push_str(pass);
//     url.push_str("@");
//     url.push_str(host);
//     url.push_str(":");
//     url.push_str(&port.to_string());
//     url.push_str("/");
//     url.push_str(db);

//     Connection::connect(url, TlsMode::None)
// }
