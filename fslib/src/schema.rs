// @generated automatically by Diesel CLI.

diesel::table! {
    agent_params (id) {
        id -> Int4,
        agent_id -> Int4,
        name -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    agents (id) {
        id -> Int4,
        domain_id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        leg_timeout -> Int4,
    }
}

diesel::table! {
    cdrs (id) {
        id -> Int4,
        a_caller_id -> Varchar,
        a_dest -> Varchar,
        start_time -> Timestamp,
        duration -> Int4,
        b_caller_id -> Nullable<Varchar>,
        b_dest -> Nullable<Varchar>,
        uuid -> Nullable<Varchar>,
    }
}

diesel::table! {
    conference_control_details (id) {
        id -> Int4,
        conference_control_id -> Int4,
        action -> Varchar,
        digits -> Varchar,
    }
}

diesel::table! {
    conference_controls (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    conference_profile_params (id) {
        id -> Int4,
        conference_profile_id -> Int4,
        name -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    conference_profiles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

diesel::table! {
    conferences (id) {
        id -> Int4,
        exten -> Varchar,
        name -> Varchar,
        domain_id -> Int4,
        conference_profile_id -> Int4,
        description -> Varchar,
    }
}

diesel::table! {
    domains (id) {
        id -> Int4,
        domain_name -> Varchar,
    }
}

diesel::table! {
    extension_types (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    extensions (id) {
        id -> Int4,
        exten -> Varchar,
        exten_type -> Varchar,
        domain_id -> Int4,
    }
}

diesel::table! {
    feature_codes (id) {
        id -> Int4,
        digits -> Varchar,
        action -> Varchar,
    }
}

diesel::table! {
    gateways (id) {
        id -> Int4,
        profile_id -> Int4,
        gateway_name -> Varchar,
        proxy -> Varchar,
        register -> Varchar,
        username -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
    }
}

diesel::table! {
    inbound_routes (id) {
        id -> Int4,
        context -> Varchar,
        condition -> Varchar,
        dest_extension -> Varchar,
    }
}

diesel::table! {
    ivr_attrs (id) {
        id -> Int4,
        ivr_id -> Int4,
        name -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    ivr_entries (id) {
        id -> Int4,
        ivr_id -> Int4,
        digits -> Varchar,
        dest_exten -> Varchar,
    }
}

diesel::table! {
    ivrs (id) {
        id -> Int4,
        exten -> Varchar,
        name -> Varchar,
        domain_id -> Int4,
    }
}

diesel::table! {
    outbound_routes (id) {
        id -> Int4,
        gateway_id -> Int4,
        priority -> Int4,
        condition -> Varchar,
    }
}

diesel::table! {
    profile_params (id) {
        id -> Int4,
        profile_id -> Int4,
        name -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    profiles (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    queue_params (id) {
        id -> Int4,
        queue_id -> Int4,
        name -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    queues (id) {
        id -> Int4,
        name -> Varchar,
        exten -> Varchar,
        domain_id -> Int4,
    }
}

diesel::table! {
    ringing_group_members (id) {
        id -> Int4,
        ringing_group_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    ringing_groups (id) {
        id -> Int4,
        name -> Varchar,
        group_id -> Varchar,
        domain_id -> Int4,
        description -> Nullable<Varchar>,
        ring_time -> Int4,
        ring_strategy -> Varchar,
    }
}

diesel::table! {
    sound_files (id) {
        id -> Int4,
        name -> Varchar,
        domain_id -> Int4,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    sounds (id) {
        id -> Int4,
        exten -> Varchar,
        name -> Varchar,
        domain_id -> Int4,
        sound_file_id -> Int4,
    }
}

diesel::table! {
    tiers (id) {
        id -> Int4,
        agent_id -> Int4,
        queue_id -> Int4,
        level -> Int4,
        position -> Int4,
    }
}

diesel::table! {
    user_params (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    user_variables (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        value -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        domain_id -> Int4,
        user_id -> Varchar,
    }
}

diesel::table! {
    voicemails (id) {
        id -> Int4,
        user_id -> Int4,
        password -> Varchar,
        email -> Nullable<Varchar>,
    }
}

diesel::joinable!(agent_params -> agents (agent_id));
diesel::joinable!(agents -> domains (domain_id));
diesel::joinable!(agents -> users (user_id));
diesel::joinable!(conference_control_details -> conference_controls (conference_control_id));
diesel::joinable!(conference_profile_params -> conference_profiles (conference_profile_id));
diesel::joinable!(conferences -> conference_profiles (conference_profile_id));
diesel::joinable!(conferences -> domains (domain_id));
diesel::joinable!(gateways -> profiles (profile_id));
diesel::joinable!(ivr_attrs -> ivrs (ivr_id));
diesel::joinable!(ivr_entries -> ivrs (ivr_id));
diesel::joinable!(ivrs -> domains (domain_id));
diesel::joinable!(outbound_routes -> gateways (gateway_id));
diesel::joinable!(profile_params -> profiles (profile_id));
diesel::joinable!(queue_params -> queues (queue_id));
diesel::joinable!(queues -> domains (domain_id));
diesel::joinable!(ringing_group_members -> ringing_groups (ringing_group_id));
diesel::joinable!(ringing_group_members -> users (user_id));
diesel::joinable!(ringing_groups -> domains (domain_id));
diesel::joinable!(sound_files -> domains (domain_id));
diesel::joinable!(sounds -> domains (domain_id));
diesel::joinable!(sounds -> sound_files (sound_file_id));
diesel::joinable!(tiers -> agents (agent_id));
diesel::joinable!(tiers -> queues (queue_id));
diesel::joinable!(user_params -> users (user_id));
diesel::joinable!(user_variables -> users (user_id));
diesel::joinable!(users -> domains (domain_id));
diesel::joinable!(voicemails -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    agent_params,
    agents,
    cdrs,
    conference_control_details,
    conference_controls,
    conference_profile_params,
    conference_profiles,
    conferences,
    domains,
    extension_types,
    extensions,
    feature_codes,
    gateways,
    inbound_routes,
    ivr_attrs,
    ivr_entries,
    ivrs,
    outbound_routes,
    profile_params,
    profiles,
    queue_params,
    queues,
    ringing_group_members,
    ringing_groups,
    sound_files,
    sounds,
    tiers,
    user_params,
    user_variables,
    users,
    voicemails,
);
