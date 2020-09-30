#[macro_use]
extern crate diesel;

use diesel::{ mysql::MysqlConnection, prelude::* };
use dotenv::dotenv;
use std::{ env, io::Write, fs::File };
use crate::{ models::JoinedChannel, playlist::Playlist };

mod models;
mod playlist;
mod schema;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}.", database_url))
}

fn get_joined_channels(connection: &MysqlConnection) -> Vec<JoinedChannel> {
    use diesel::dsl::sql;
    use crate::schema::*;

    channel::table
        .left_join(channel_category::table.inner_join(category::table))
        .inner_join(source_channel::table.inner_join(source::table))
        .left_join(tv_guide_channel::table)
        .filter(source::is_active)
        .filter(source_channel::is_active)
        .filter(sql("TRUE GROUP BY `channel`.`id` HAVING NOT `is_excluded`"))
        .order((category::short_name, channel::name))
        .select((
            channel::all_columns,
            category::all_columns.nullable(),
            source_channel::all_columns,
            source::all_columns,
            tv_guide_channel::all_columns.nullable(),
            sql("SUM(IF(`category`.`is_active`, 0, 1)) > 0 `is_excluded`"),
        ))
        .load::<JoinedChannel>(connection)
        .unwrap()
}

fn main() {
    let connection = establish_connection();
    let joined_channels = get_joined_channels(&connection);
    Playlist::new(joined_channels, true).write_to_file("playlist.m3u8");
}
