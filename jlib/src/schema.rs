// @generated automatically by Diesel CLI.

diesel::table! {
    acl_lists (id) {
        id -> Int4,
        #[max_length = 128]
        acl_name -> Varchar,
        #[max_length = 128]
        acl_default -> Varchar,
    }
}

diesel::table! {
    acl_nodes (id) {
        id -> Int4,
        list_id -> Int4,
        #[max_length = 128]
        node_type -> Varchar,
        #[max_length = 128]
        cidr -> Varchar,
    }
}

diesel::table! {
    agent_params (id) {
        id -> Int4,
        agent_id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        value -> Varchar,
    }
}

diesel::table! {
    agents (id) {
        id -> Int4,
        domain_id -> Int4,
        user_id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        leg_timeout -> Int4,
    }
}

diesel::table! {
    cdr (id) {
        id -> Int4,
        caller_id_name -> Nullable<Varchar>,
        caller_id_number -> Nullable<Varchar>,
        destination_number -> Varchar,
        start_stamp -> Timestamptz,
        answer_stamp -> Nullable<Timestamptz>,
        end_stamp -> Timestamptz,
        duration -> Int4,
        billsec -> Int4,
        hangup_cause -> Varchar,
    }
}

diesel::table! {
    conference_control_details (id) {
        id -> Int4,
        conference_control_id -> Int4,
        #[max_length = 32]
        action -> Varchar,
        #[max_length = 8]
        digits -> Varchar,
    }
}

diesel::table! {
    conference_controls (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 512]
        description -> Varchar,
    }
}

diesel::table! {
    conference_profile_params (id) {
        id -> Int4,
        conference_profile_id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        value -> Varchar,
    }
}

diesel::table! {
    conference_profiles (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 512]
        description -> Varchar,
    }
}

diesel::table! {
    conferences (id) {
        id -> Int4,
        #[max_length = 32]
        exten -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        domain_id -> Int4,
        conference_profile_id -> Int4,
        #[max_length = 512]
        description -> Varchar,
    }
}

diesel::table! {
    domains (id) {
        id -> Int4,
        #[max_length = 128]
        domain_name -> Varchar,
    }
}

diesel::table! {
    extension_types (id) {
        id -> Int4,
        #[max_length = 32]
        name -> Varchar,
    }
}

diesel::table! {
    extensions (id) {
        id -> Int4,
        #[max_length = 128]
        exten -> Varchar,
        #[max_length = 64]
        exten_type -> Varchar,
        domain_id -> Int4,
    }
}

diesel::table! {
    feature_codes (id) {
        id -> Int4,
        domain_id -> Int4,
        #[max_length = 8]
        digits -> Varchar,
        #[max_length = 128]
        action -> Varchar,
    }
}

diesel::table! {
    firewall_rules (id) {
        id -> Int4,
        #[max_length = 45]
        ip_address -> Varchar,
        #[max_length = 16]
        action -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    gateway_param_helps (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 1024]
        range_text -> Varchar,
        #[max_length = 1024]
        help_text -> Varchar,
    }
}

diesel::table! {
    gateway_params (id) {
        id -> Int4,
        gateway_id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        value -> Varchar,
    }
}

diesel::table! {
    gateways (id) {
        id -> Int4,
        profile_id -> Int4,
        #[max_length = 256]
        gateway_name -> Varchar,
    }
}

diesel::table! {
    inbound_routes (id) {
        id -> Int4,
        #[max_length = 64]
        context -> Varchar,
        #[max_length = 512]
        condition -> Varchar,
        #[max_length = 64]
        dest_extension -> Varchar,
    }
}

diesel::table! {
    ivr_attrs (id) {
        id -> Int4,
        ivr_id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        value -> Varchar,
    }
}

diesel::table! {
    ivr_entries (id) {
        id -> Int4,
        ivr_id -> Int4,
        #[max_length = 8]
        digits -> Varchar,
        #[max_length = 64]
        dest_exten -> Varchar,
    }
}

diesel::table! {
    ivrs (id) {
        id -> Int4,
        #[max_length = 32]
        exten -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        domain_id -> Int4,
    }
}

diesel::table! {
    outbound_routes (id) {
        id -> Int4,
        gateway_id -> Int4,
        priority -> Int4,
        #[max_length = 512]
        condition -> Varchar,
        #[max_length = 16]
        prepend -> Varchar,
        prefix -> Int4,
    }
}

diesel::table! {
    portal_tokens (id) {
        id -> Int4,
        portal_user_id -> Int4,
        #[max_length = 256]
        token -> Varchar,
        expire_at -> Timestamptz,
    }
}

diesel::table! {
    portal_users (id) {
        id -> Int4,
        #[max_length = 128]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
    }
}

diesel::table! {
    profile_params (id) {
        id -> Int4,
        profile_id -> Int4,
        #[max_length = 256]
        name -> Varchar,
        #[max_length = 256]
        value -> Varchar,
    }
}

diesel::table! {
    profiles (id) {
        id -> Int4,
        #[max_length = 256]
        name -> Varchar,
    }
}

diesel::table! {
    queue_params (id) {
        id -> Int4,
        queue_id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        value -> Varchar,
    }
}

diesel::table! {
    queues (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 32]
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
        #[max_length = 32]
        name -> Varchar,
        #[max_length = 32]
        group_id -> Varchar,
        domain_id -> Int4,
        #[max_length = 256]
        description -> Nullable<Varchar>,
        ring_time -> Int4,
        #[max_length = 32]
        ring_strategy -> Varchar,
    }
}

diesel::table! {
    sound_files (id) {
        id -> Int4,
        #[max_length = 256]
        name -> Varchar,
        domain_id -> Int4,
        #[max_length = 1024]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    sounds (id) {
        id -> Int4,
        #[max_length = 32]
        exten -> Varchar,
        #[max_length = 32]
        name -> Varchar,
        domain_id -> Int4,
        sound_file_id -> Int4,
    }
}

diesel::table! {
    system_settings (id) {
        id -> Int4,
        #[max_length = 255]
        setting_section -> Varchar,
        #[max_length = 255]
        setting_key -> Varchar,
        #[max_length = 255]
        setting_value -> Varchar,
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
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        value -> Varchar,
    }
}

diesel::table! {
    user_variables (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        value -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        domain_id -> Int4,
        #[max_length = 128]
        user_id -> Varchar,
    }
}

diesel::table! {
    voicemails (id) {
        id -> Int4,
        user_id -> Int4,
        #[max_length = 32]
        password -> Varchar,
        #[max_length = 128]
        email -> Nullable<Varchar>,
    }
}

diesel::joinable!(acl_nodes -> acl_lists (list_id));
diesel::joinable!(agent_params -> agents (agent_id));
diesel::joinable!(agents -> domains (domain_id));
diesel::joinable!(agents -> users (user_id));
diesel::joinable!(conference_control_details -> conference_controls (conference_control_id));
diesel::joinable!(conference_profile_params -> conference_profiles (conference_profile_id));
diesel::joinable!(conferences -> conference_profiles (conference_profile_id));
diesel::joinable!(conferences -> domains (domain_id));
diesel::joinable!(feature_codes -> domains (domain_id));
diesel::joinable!(gateway_params -> gateways (gateway_id));
diesel::joinable!(gateways -> profiles (profile_id));
diesel::joinable!(ivr_attrs -> ivrs (ivr_id));
diesel::joinable!(ivr_entries -> ivrs (ivr_id));
diesel::joinable!(ivrs -> domains (domain_id));
diesel::joinable!(outbound_routes -> gateways (gateway_id));
diesel::joinable!(portal_tokens -> portal_users (portal_user_id));
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
    acl_lists,
    acl_nodes,
    agent_params,
    agents,
    cdr,
    conference_control_details,
    conference_controls,
    conference_profile_params,
    conference_profiles,
    conferences,
    domains,
    extension_types,
    extensions,
    feature_codes,
    firewall_rules,
    gateway_param_helps,
    gateway_params,
    gateways,
    inbound_routes,
    ivr_attrs,
    ivr_entries,
    ivrs,
    outbound_routes,
    portal_tokens,
    portal_users,
    profile_params,
    profiles,
    queue_params,
    queues,
    ringing_group_members,
    ringing_groups,
    sound_files,
    sounds,
    system_settings,
    tiers,
    user_params,
    user_variables,
    users,
    voicemails,
);
