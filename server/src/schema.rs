table! {
    blogs (id) {
        id -> Text,
        user_id -> Text,
        title -> Text,
        content -> Text,
        tag -> Nullable<Text>,
        image -> Nullable<Binary>,
        image_url -> Nullable<Text>,
        likes -> Nullable<Integer>,
        page_view_num -> Nullable<Integer>,
        commnet_id -> Nullable<Text>,
        is_deleted -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    product (id) {
        id -> Integer,
        name -> Text,
        title -> Text,
        data_created -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        username -> Nullable<Text>,
        password -> Nullable<Text>,
        email -> Nullable<Text>,
        phone -> Nullable<Text>,
        avatar -> Nullable<Binary>,
        avatar_url -> Nullable<Text>,
        slogan -> Nullable<Text>,
        is_deleted -> Nullable<Integer>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    blogs,
    product,
    users,
);
