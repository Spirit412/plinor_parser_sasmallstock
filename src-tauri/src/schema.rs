diesel::table! {
    animals (id) {
        id -> Integer,
        create_at -> Timestamp,
        update_at -> Timestamp,
        data -> Text, // Предполагаем, что данные будут сериализованы в JSON
    }
}