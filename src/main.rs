#[macro_use]
extern crate diesel;

use crate::{models::ChannelInfo, playlist::Playlist};
use diesel::{mysql::MysqlConnection, prelude::*};
use dotenv::dotenv;
use std::env;

mod models;
mod playlist;
mod schema;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}.", database_url))
}

fn get_joined_channels(connection: &MysqlConnection) -> Vec<ChannelInfo> {
    use crate::schema::*;
    use diesel::dsl::sql;

    // To avoid an error "cannot infer type for type parameter `ST` declared on the function `sql`".
    #[derive(QueryId, SqlType)]
    struct ST;

    channel::table
        .left_join(channel_category::table.inner_join(category::table))
        .inner_join(source_channel::table.inner_join(source::table))
        .left_join(tv_guide_channel::table)
        .filter(
            sql::<ST>("(`source_channel`.`id`, 1)").eq_any(
                channel::table
                    .inner_join(source_channel::table.inner_join(source::table))
                    .filter(source::is_active.and(source_channel::is_active))
                    .select(sql("`source_channel`.`id`, \
                                  ROW_NUMBER() OVER(PARTITION BY `channel`.`id` \
                                                    ORDER BY `source`.`priority` DESC)"))
                    .into_boxed(),
            ),
        )
        // Diesel doesn't yet support "HAVING".
        .group_by(sql::<ST>("`channel`.`id` HAVING SUM(IF(`category`.`is_active`, 0, 1)) = 0"))
        .order((category::short_name, channel::name))
        .select((
            channel::all_columns,
            category::all_columns.nullable(),
            source_channel::all_columns,
            source::all_columns,
            tv_guide_channel::all_columns.nullable(),
        ))
        .load::<ChannelInfo>(connection)
        .unwrap()
}

fn main() {
    let connection = establish_connection();
    let joined_channels = get_joined_channels(&connection);
    Playlist::new(joined_channels, true).write_to_file("playlist.m3u8");
}
