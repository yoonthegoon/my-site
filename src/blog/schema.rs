diesel::table! {
    posts (slug) {
        slug -> Text,
        author -> Integer,
        title -> Text,
        body -> Text,
        created_at -> BigInt,
        updated_at -> BigInt,
    }
}
