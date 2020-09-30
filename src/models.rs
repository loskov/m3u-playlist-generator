#[derive(Queryable)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub short_name: String,
    pub is_active: bool,
}

#[derive(Queryable)]
pub struct Channel {
    pub id: i32,
    pub name: String,
    pub tv_guide_logo: Option<String>,
}

#[derive(Queryable)]
pub struct JoinedChannel {
    pub channel: Channel,
    pub category: Option<Category>,
    pub source_channel: SourceChannel,
    pub source: Source,
    pub tv_guide_channel: Option<TvGuideChannel>,
    pub is_excluded: bool,
}

#[derive(Queryable)]
pub struct ChannelCategory {
    pub id: i32,
    pub channel_id: i32,
    pub category_id: i32,
}

#[derive(Queryable)]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub user_agent: Option<String>,
    pub description: Option<String>,
    pub query_string: Option<String>,
    pub is_active: bool,
}

#[derive(Queryable)]
pub struct SourceChannel {
    pub id: i32,
    pub source_id: i32,
    pub channel_id: i32,
    pub url: String,
    pub is_active: bool,
}

#[derive(Queryable)]
pub struct TvGuide {
    pub id: i32,
    pub name: String,
    pub url: String,
}

#[derive(Queryable)]
pub struct TvGuideChannel {
    pub id: i32,
    pub external_id: String,
    pub tv_guide_id: i32,
    pub channel_id: i32,
}
