use derive_builder::Builder;
use ring::digest::{self, digest, SHA256};
use tracing::info;

use self::param::SrpParams;

mod client;
mod param;

mod test_rsp {

    use base64::{engine::general_purpose, Engine as _};
    use ring::rand::{self, SystemRandom};
    use tracing::info;

    use crate::common::srp::{client::SrpClientBuilder, param::SrpParams};

    #[test]
    fn test_registry() {
        // credential
        let credential = String::from("alice");
        // proof
        let proof = String::from("password123");

        let mut srp_client = SrpClientBuilder::default()
            .credential(credential)
            .proof(proof)
            .build()
            .unwrap();

        let random = SystemRandom::new();
        // slice
        let salt = rand::generate::<[u8; 32]>(&random)
            .unwrap()
            .expose()
            .to_vec();
        let params = SrpParams::generate4096();
        let verifier = srp_client.verfier(b"salty", &params);
        println!(
            "salt: {:?}, verifier: {:?}, ",
            hex::encode(&salt),
            hex::encode(&verifier)
        );
    }

    #[test]
    fn checkM1() {}
}
