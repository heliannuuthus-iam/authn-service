pub mod client_config_controller;
pub mod sms_config_controller;
pub mod user_association_controller;
pub mod user_controller;

use utoipa::OpenApi;

use crate::{
    common::enums::IdpType,
    pojo::{
        dto::user::UserProfileDTO,
        form::{
            client::{
                ClientConfigCreateForm, ClientConfigUpdateForm, ClientIdpConfigSaveOrUpdateForm,
            },
            user::RegistryForm,
        },
        po::{
            client::{ClientConfig, ClientIdpConfig},
            sms_config::SmsConfig,
            srp::SrpPassword,
            user::{User, UserAssociation},
        },
    },
};
#[derive(OpenApi)]
#[openapi(
    paths(
        client_config_controller::create_client,
        client_config_controller::client_config,
        client_config_controller::set_client_config,
        client_config_controller::client_specify_idp_config,
        client_config_controller::client_all_idp_config,
        client_config_controller::set_client_idp_config,
        sms_config_controller::get_sms_config,
        user_association_controller::user_associations,
        user_association_controller::user_idp_associations,
        user_controller::user_rsp,
        user_controller::user_profile,
    ),
    components(schemas(
        ClientConfigCreateForm,
        ClientConfigUpdateForm,
        ClientIdpConfigSaveOrUpdateForm,
        RegistryForm,
        ClientConfig,
        ClientIdpConfig,
        SmsConfig,
        UserAssociation,
        SrpPassword,
        User,
        UserProfileDTO,
        IdpType
    ))
)]
pub struct ApiDoc;
