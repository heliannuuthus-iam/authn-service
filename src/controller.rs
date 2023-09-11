pub mod challenge_config_controller;
pub mod client_config_controller;
pub mod password_controller;
pub mod sms_config_controller;
pub mod user_association_controller;
pub mod user_controller;

use utoipa::OpenApi;

use crate::{
    common::enums::IdpType,
    pojo::{
        dto::user::{UserAssociationDTO, UserProfileDTO},
        form::{
            client::{
                ClientConfigCreateForm, ClientConfigUpdateForm, ClientIdpConfigSaveOrUpdateForm,
            },
            user::{SrpPasswordForm, UserAssoInitialForm},
        },
        po::{
            challenge::ChallengeCofig,
            client::{ClientConfig, ClientIdpConfig},
            sms::SmsConfig,
            srp::SrpPassword,
            user::{User, UserAssociation},
        },
    },
};
#[derive(OpenApi)]
#[openapi(
    paths(
        password_controller::presist_srp,
        password_controller::user_rsp,
        user_controller::user_profile,
        user_association_controller::user_associations,
        user_association_controller::user_idp_associations,
        user_association_controller::create_user_and_init_idp_asso,
        sms_config_controller::get_sms_config,
        sms_config_controller::set_sms_config,
        challenge_config_controller::list_challenge_config,
        challenge_config_controller::set_challenge_config,
        client_config_controller::create_client,
        client_config_controller::client_config,
        client_config_controller::set_client_config,
        client_config_controller::client_specify_idp_config,
        client_config_controller::client_all_idp_config,
        client_config_controller::set_client_idp_config,
    ),
    components(schemas(
        ClientConfigCreateForm,
        ClientConfigUpdateForm,
        ClientIdpConfigSaveOrUpdateForm,
        SrpPasswordForm,
        ClientConfig,
        ClientIdpConfig,
        SmsConfig,
        ChallengeCofig,
        UserAssociation,
        UserAssociationDTO,
        UserAssoInitialForm,
        SrpPassword,
        User,
        UserProfileDTO,
        IdpType
    ))
)]
pub struct ApiDoc;
