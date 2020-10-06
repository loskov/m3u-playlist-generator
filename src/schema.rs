table! {
    category (id) {
        id -> Integer,
        name -> VarChar,
        short_name -> VarChar,
        is_active -> Bool,
    }
}

table! {
    channel (id) {
        id -> Integer,
        name -> VarChar,
        tv_guide_logo -> Nullable<VarChar>,
    }
}

table! {
    channel_category (id) {
        id -> Integer,
        channel_id -> Integer,
        category_id -> Integer,
    }
}

table! {
    source (id) {
        id -> Integer,
        name -> VarChar,
        user_agent -> Nullable<VarChar>,
        description -> Nullable<VarChar>,
        query_string -> Nullable<VarChar>,
        is_active -> Bool,
        priority -> Integer,
    }
}

table! {
    source_channel (id) {
        id -> Integer,
        source_id -> Integer,
        channel_id -> Integer,
        url -> VarChar,
        is_active -> Bool,
    }
}

table! {
    tv_guide (id) {
        id -> Integer,
        name -> VarChar,
        url -> VarChar,
    }
}

table! {
    tv_guide_channel (id) {
        id -> Integer,
        external_id -> VarChar,
        tv_guide_id -> Integer,
        channel_id -> Integer,
    }
}

joinable!(channel_category -> channel (channel_id));
joinable!(channel_category -> category (category_id));

joinable!(source_channel -> source (source_id));
joinable!(source_channel -> channel (channel_id));

joinable!(tv_guide_channel -> tv_guide (tv_guide_id));
joinable!(tv_guide_channel -> channel (channel_id));

allow_tables_to_appear_in_same_query!(
    category,
    channel,
    channel_category,
    source,
    source_channel,
    tv_guide_channel,
);
