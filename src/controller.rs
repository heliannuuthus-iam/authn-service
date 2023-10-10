pub mod challenge_config_controller;
pub mod client_controller;
pub mod idp_controller;
pub mod password_controller;
pub mod server_controller;
pub mod sms_config_controller;
pub mod user_association_controller;
pub mod user_controller;

use utoipa::OpenApi;

use crate::{
    common::enums::{ClientType, IdpType},
    pojo::{
        dto::user::{UserAssociationDTO, UserProfileDTO},
        form::{
            client::{
                ClientIdpConfigSaveOrUpdateForm, ClientSettingCreateForm, ClientSettingUpdateForm,
            },
            server::{ServerSettingCreateForm, ServerSettingUpdateForm},
            user::{SrpPasswordForm, UserAssoInitialForm},
        },
        po::{
            challenge::ChallengeCofig,
            client::{Client, ClientIdpConfig, ClientSetting},
            server::{Server, ServerSetting},
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
        client_controller::create_client,
        client_controller::list_clients,
        client_controller::client_details,
        client_controller::set_client_config,
        client_controller::remove_client,
        server_controller::server_detail,
        server_controller::create_server,
        server_controller::patch_server,
        server_controller::remove_server,
        idp_controller::client_specify_idp_config,
        idp_controller::client_all_idp_config,
        idp_controller::set_client_idp_config,
    ),
    components(schemas(
        ClientSettingCreateForm,
        ServerSettingCreateForm,
        ClientSettingUpdateForm,
        ServerSettingUpdateForm,
        ClientIdpConfigSaveOrUpdateForm,
        SrpPasswordForm,
        Client,
        ClientSetting,
        Server,
        ServerSetting,
        ClientIdpConfig,
        SmsConfig,
        ChallengeCofig,
        UserAssociation,
        UserAssociationDTO,
        UserAssoInitialForm,
        SrpPassword,
        User,
        UserProfileDTO,
        IdpType,
        ClientType
    ))
)]
pub struct ApiDoc;
