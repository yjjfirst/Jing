// @generated automatically by Diesel CLI.

diesel::table! {
    cdr (id) {
        id -> Integer,
        a_caller_id -> Varchar,
        a_dest -> Varchar,
        start_time -> Datetime,
        duration -> Integer,
        b_caller_id -> Nullable<Varchar>,
        b_dest -> Nullable<Varchar>,
        uuid -> Nullable<Varchar>,
    }
}

diesel::table! {
    domain (id) {
        id -> Integer,
        domain_name -> Varchar,
        active -> Bool,
    }
}

diesel::table! {
    extension (id) {
        id -> Integer,
        exten -> Varchar,
        exten_type -> Varchar,
        domain_id -> Integer,
    }
}

diesel::table! {
    extension_type (id) {
        id -> Integer,
        name -> Varchar,
    }
}

diesel::table! {
    gateway (id) {
        id -> Integer,
        profile_id -> Integer,
        gateway_name -> Varchar,
        proxy -> Varchar,
        register -> Varchar,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
    }
}

diesel::table! {
    inbound_route (id) {
        id -> Integer,
        context -> Varchar,
        condition -> Varchar,
        dest_extension -> Varchar,
    }
}

diesel::table! {
    ivr (id) {
        id -> Integer,
        exten -> Varchar,
        name -> Varchar,
        domain_id -> Integer,
        greet_long -> Nullable<Varchar>,
        greet_short -> Nullable<Varchar>,
        invalid_sound -> Nullable<Varchar>,
        exit_sound -> Nullable<Varchar>,
        confirm_attempts -> Nullable<Integer>,
        timeout -> Nullable<Integer>,
        inter_digit_timeout -> Nullable<Integer>,
        max_failures -> Nullable<Integer>,
        max_timeouts -> Nullable<Integer>,
        digit_len -> Nullable<Integer>,
    }
}

diesel::table! {
    ivr_option (id) {
        id -> Integer,
        ivr_id -> Nullable<Integer>,
        digits -> Varchar,
        dest_type -> Varchar,
        dest_exten -> Varchar,
    }
}

diesel::table! {
    outbound_route (id) {
        id -> Integer,
        gateway_id -> Integer,
        priority -> Integer,
        condition -> Varchar,
    }
}

diesel::table! {
    profile (id) {
        id -> Integer,
        name -> Varchar,
    }
}

diesel::table! {
    profile_param (id) {
        id -> Integer,
        profile_id -> Integer,
        name -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    ringing_group (id) {
        id -> Integer,
        name -> Varchar,
        group_id -> Varchar,
        domain_id -> Integer,
        description -> Nullable<Varchar>,
        ring_time -> Integer,
        ring_strategy -> Varchar,
    }
}

diesel::table! {
    ringing_group_member (id) {
        id -> Integer,
        ringing_group_id -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    sound (id) {
        id -> Integer,
        exten -> Varchar,
        name -> Varchar,
        domain_id -> Integer,
        sound_file_id -> Integer,
    }
}

diesel::table! {
    sound_file (id) {
        id -> Integer,
        name -> Varchar,
        domain_id -> Integer,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        domain_id -> Integer,
        number_alias -> Nullable<Varchar>,
        mailbox -> Nullable<Varchar>,
        cidr -> Nullable<Varchar>,
        user_id -> Varchar,
        password -> Varchar,
        toll_allow -> Nullable<Varchar>,
        user_context -> Nullable<Varchar>,
        default_gateway -> Nullable<Varchar>,
        effective_caller_id_name -> Nullable<Varchar>,
        effective_caller_id_number -> Nullable<Varchar>,
        outbound_caller_id_name -> Nullable<Varchar>,
        outbound_caller_id_number -> Nullable<Varchar>,
        callgroup -> Nullable<Varchar>,
        uservar1 -> Nullable<Varchar>,
        uservar2 -> Nullable<Varchar>,
        uservar3 -> Nullable<Varchar>,
    }
}

diesel::table! {
    voicemail (id) {
        id -> Integer,
        user_id -> Integer,
        password -> Varchar,
        email -> Nullable<Varchar>,
    }
}

diesel::joinable!(gateway -> profile (profile_id));
diesel::joinable!(ivr -> domain (domain_id));
diesel::joinable!(ivr_option -> ivr (ivr_id));
diesel::joinable!(outbound_route -> gateway (gateway_id));
diesel::joinable!(profile_param -> profile (profile_id));
diesel::joinable!(ringing_group -> domain (domain_id));
diesel::joinable!(ringing_group_member -> ringing_group (ringing_group_id));
diesel::joinable!(ringing_group_member -> user (user_id));
diesel::joinable!(sound -> domain (domain_id));
diesel::joinable!(sound -> sound_file (sound_file_id));
diesel::joinable!(sound_file -> domain (domain_id));
diesel::joinable!(user -> domain (domain_id));
diesel::joinable!(voicemail -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cdr,
    domain,
    extension,
    extension_type,
    gateway,
    inbound_route,
    ivr,
    ivr_option,
    outbound_route,
    profile,
    profile_param,
    ringing_group,
    ringing_group_member,
    sound,
    sound_file,
    user,
    voicemail,
);
