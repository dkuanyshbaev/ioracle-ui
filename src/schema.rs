table! {
    answers (id) {
        id -> Text,
        email -> Text,
        question -> Text,
        hexagram -> Text,
        related -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    hexagrams (id) {
        id -> Integer,
        binary -> Text,
        king_wen_order -> Integer,
        shao_yong_order -> Integer,
        gua -> Text,
        pin_yin -> Text,
        character -> Text,
        wilheim -> Text,
        huang -> Text,
        hatcher -> Text,
        no2do -> Text,
        inner_ba_gua -> Text,
        outer_ba_gua -> Text,
        host_yao -> Text,
        judgment -> Text,
        image -> Text,
        lines -> Text,
    }
}

table! {
    trigrams (id) {
        id -> Integer,
        name -> Text,
        image -> Text,
        binary -> Text,
        no -> Text,
        wen -> Text,
        host -> Text,
        element -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    answers,
    hexagrams,
    trigrams,
);
