// @generated automatically by Diesel CLI.

diesel::table! {
    cached_features (id) {
        id -> Bigint,
        structure_id -> Char,
    }
}

diesel::table! {
    candidate_lists (id) {
        id -> Integer,
        title -> Char,
    }
}

diesel::table! {
    jobs (id) {
        id -> Bigint,
        request_id -> Bigint,
        node_id -> Tinyint,
        assignment_date -> Datetime,
        completion_date -> Nullable<Datetime>,
        last_checked -> Nullable<Datetime>,
        status_code -> Tinyint,
        secure_hash -> Char,
    }
}

diesel::table! {
    nodes (id) {
        id -> Tinyint,
        ip -> Char,
        domain -> Char,
        active -> Bool,
        working -> Bool,
        sync_date -> Datetime,
        cores -> Tinyint, 
    }
}

diesel::table! {
    requests (id) {
        id -> Bigint,
        reference -> Char,
        candidates_list_id -> Integer,
        custom_list -> Text,
        uncached -> Text,
        hash_value -> Char,
        creation_date -> Datetime,
        meta -> Bool,
        go_term -> Char,
        comparison_mode  -> Tinyint,
        segment_start -> Integer,
        segment_end -> Integer,
        alignment_level -> TinyInt,
        views -> Bigint,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cached_features,
    candidate_lists,
    jobs,
    nodes,
    requests,
);

diesel::table! {
    joined_requests (id) {
        id -> Bigint,
        reference -> Char,
        candidates_list_id -> Integer,
        custom_list -> Text,
        uncached -> Text,
        hash_value -> Char,
        creation_date -> Datetime,
        meta -> Bool,
        go_term -> Char,
        comparison_mode  -> Tinyint,
        segment_start -> Integer,
        segment_end -> Integer, 
        alignment_level -> TinyInt,
        views -> Bigint,
        list_name -> Nullable<Char>,
    }
}


diesel::table! {
    finalized_requests (id) {
        id -> Bigint,
        reference -> Char,
        candidates_list_id -> Integer,
        custom_list -> Text,
        uncached -> Text,
        hash_value -> Char,
        creation_date -> Datetime,
        meta -> Bool,
        go_term -> Char,
        comparison_mode  -> Tinyint,
        segment_start -> Integer,
        segment_end -> Integer, 
        alignment_level -> TinyInt,
        views -> Bigint,
        secure_hash -> Char,
        list_name -> Nullable<Char>,
        status_code -> Tinyint,
    }
}


diesel::table! {
    joined_jobs (id) {
        id -> Bigint, 
        hash_value -> Char, 
        request_id -> BigInt,
        node_id -> Tinyint,
        node_ip -> Char,
        node_domain -> Char,
        comparison_mode -> Tinyint,
    }
}
