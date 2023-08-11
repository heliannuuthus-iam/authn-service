pub mod client_config_controller;
pub mod sms_config_controller;
pub mod user_association_controller;
pub mod user_controller;

use user_controller::RegistryForm;
use utoipa::OpenApi;

use crate::{
    common::enums::IdpType,
    pojo::{
        dto::user::{UserAssociationDTO, UserProfileDTO},
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
        sms_config_controller::get_sms_config,
        user_association_controller::user_associations,
        user_association_controller::user_idp_associations,
        user_controller::user_rsp,
        user_controller::user_profile,
    ),
    components(schemas(
        client_config_controller::ClientConfigCreateForm,
        client_config_controller::ClientConfigUpdateForm,
        ClientConfig,
        ClientIdpConfig,
        SmsConfig,
        UserAssociationDTO,
        UserAssociation,
        SrpPassword,
        User,
        UserProfileDTO,
        RegistryForm,
        IdpType
    ))
)]
pub struct ApiDoc;
