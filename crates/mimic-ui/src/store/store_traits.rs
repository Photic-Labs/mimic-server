use rusqlite::Connection;

pub trait Loadable {
    fn load_from_db(conn: &Connection) -> Self;
    fn refresh_data(&mut self, conn: &Connection);
}
