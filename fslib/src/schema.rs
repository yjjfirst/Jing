// @generated automatically by Diesel CLI.

diesel::table! {
    cdrs (id) {
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
    conference_control_details (id) {
        id -> Integer,
        conference_control_id -> Integer,
        action -> Varchar,
        digits -> Varchar,
    }
}

diesel::table! {
    conference_controls (id) {
        id -> Integer,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    conference_profile_params (id) {
        id -> Integer,
        conference_profile_id -> Integer,
        name -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    conference_profiles (id) {
        id -> Integer,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    domains (id) {
        id -> Integer,
        domain_name -> Varchar,
    }
}

diesel::table! {
    extension_types (id) {
        id -> Integer,
        name -> Varchar,
    }
}

diesel::table! {
    extensions (id) {
        id -> Integer,
        exten -> Varchar,
        exten_type -> Varchar,
        domain_id -> Integer,
    }
}

diesel::table! {
    gateways (id) {
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
    inbound_routes (id) {
        id -> Integer,
        context -> Varchar,
        condition -> Varchar,
        dest_extension -> Varchar,
    }
}

diesel::table! {
    ivr_options (id) {
        id -> Integer,
        ivr_id -> Nullable<Integer>,
        digits -> Varchar,
        dest_type -> Varchar,
        dest_exten -> Varchar,
    }
}

diesel::table! {
    ivrs (id) {
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
    outbound_routes (id) {
        id -> Integer,
        gateway_id -> Integer,
        priority -> Integer,
        condition -> Varchar,
    }
}

diesel::table! {
    profile_params (id) {
        id -> Integer,
        profile_id -> Integer,
        name -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    profiles (id) {
        id -> Integer,
        name -> Varchar,
    }
}

diesel::table! {
    ringing_group_members (id) {
        id -> Integer,
        ringing_group_id -> Integer,
        user_id -> Integer,
    }
}

diesel::table! {
    ringing_groups (id) {
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
    sound_files (id) {
        id -> Integer,
        name -> Varchar,
        domain_id -> Integer,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    sounds (id) {
        id -> Integer,
        exten -> Varchar,
        name -> Varchar,
        domain_id -> Integer,
        sound_file_id -> Integer,
    }
}

diesel::table! {
    users (id) {
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
    voicemails (id) {
        id -> Integer,
        user_id -> Integer,
        password -> Varchar,
        email -> Nullable<Varchar>,
    }
}

diesel::joinable!(conference_control_details -> conference_controls (conference_control_id));
diesel::joinable!(conference_profile_params -> conference_profiles (conference_profile_id));
diesel::joinable!(gateways -> profiles (profile_id));
diesel::joinable!(ivr_options -> ivrs (ivr_id));
diesel::joinable!(ivrs -> domains (domain_id));
diesel::joinable!(outbound_routes -> gateways (gateway_id));
diesel::joinable!(profile_params -> profiles (profile_id));
diesel::joinable!(ringing_group_members -> ringing_groups (ringing_group_id));
diesel::joinable!(ringing_group_members -> users (user_id));
diesel::joinable!(ringing_groups -> domains (domain_id));
diesel::joinable!(sound_files -> domains (domain_id));
diesel::joinable!(sounds -> domains (domain_id));
diesel::joinable!(sounds -> sound_files (sound_file_id));
diesel::joinable!(users -> domains (domain_id));
diesel::joinable!(voicemails -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cdrs,
    conference_control_details,
    conference_controls,
    conference_profile_params,
    conference_profiles,
    domains,
    extension_types,
    extensions,
    gateways,
    inbound_routes,
    ivr_options,
    ivrs,
    outbound_routes,
    profile_params,
    profiles,
    ringing_group_members,
    ringing_groups,
    sound_files,
    sounds,
    users,
    voicemails,
);
