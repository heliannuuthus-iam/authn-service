use std::sync::Arc;

use nacos_sdk::api::{
    constants,
    naming::{
        NamingChangeEvent, NamingEventListener, NamingService, NamingServiceBuilder,
        ServiceInstance,
    },
    props::ClientProps,
};

use super::config::{env_var, env_var_default};

lazy_static::lazy_static! {
  pub static ref NACOS_CLIENT: Box<dyn NamingService + Send + Sync> = Box::new(NamingServiceBuilder::new(
    ClientProps::new()
        .server_addr(env_var::<String>("NACOS_SERVER"))
        .namespace(env_var::<String>("NACOS_NAMESPACE"))
        .app_name(env_var::<String>("CARGO_PKG_NAME")))
        .build().unwrap());
}

pub struct InstanceChangeListener;

impl NamingEventListener for InstanceChangeListener {
    fn event(&self, event: std::sync::Arc<NamingChangeEvent>) {
        tracing::info!("subscriber notify event={:?}", event);
    }
}

pub async fn init_nacos() {
    if env_var_default::<bool>("NACOS_REGISTRY", true) {
        let _subscribe_ret = NACOS_CLIENT
            .subscribe(
                env_var::<String>("CARGO_PKG_NAME"),
                Some(constants::DEFAULT_GROUP.to_string()),
                Vec::default(),
                Arc::new(InstanceChangeListener),
            )
            .await;

        // example naming register instances
        let _register_instance_ret = NACOS_CLIENT
            .batch_register_instance(
                env!("CARGO_PKG_NAME").to_string(),
                Some(constants::DEFAULT_GROUP.to_string()),
                vec![ServiceInstance {
                    ip: env_var("SERVER_HOST"),
                    port: env_var("SERVER_PORT"),
                    ..Default::default()
                }],
            )
            .await;
    }
}
