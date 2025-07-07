// @generated automatically by Diesel CLI.

#[cfg(feature = "server")]
diesel::table! {
    blog_posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        content -> Text,
    }
}
