
// @generated automatically by Diesel CLI.
diesel::table! {
    blog_posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        content -> Text,
    }
}
