use std::ops::Sub;

use num_bigint::BigInt;

pub struct SrpParams {
    pub n: BigInt,
    pub g: BigInt,
    pub len: u16,
}

pub fn padding(number: Vec<u8>, length: usize) -> Vec<u8> {
    // TODO 对 length 和 number.length 判断，如果 length < number.length panic
    let padding = length - number.len();
    let mut result: Vec<u8> = vec![0; length];
    result.copy_from_slice(&number[padding..]);
    result
}

impl SrpParams {
    pub fn padding(&self, number: BigInt) -> Vec<u8> {
        let mut h = number.to_str_radix(16);
        if h.len() % 2 == 1 {
            h.insert(0, '0');
        }
        padding(hex::decode(h).unwrap(), (self.len / 8).into())
    }

    pub fn generate2048() -> Self {
        Self {
            n: BigInt::parse_bytes(
                r#"
                AC6BDB41 324A9A9B F166DE5E 1389582F AF72B665 1987EE07 FC319294
                3DB56050 A37329CB B4A099ED 8193E075 7767A13D D52312AB 4B03310D
                CD7F48A9 DA04FD50 E8083969 EDB767B0 CF609517 9A163AB3 661A05FB
                D5FAAAE8 2918A996 2F0B93B8 55F97993 EC975EEA A80D740A DBF4FF74
                7359D041 D5C33EA7 1D281E44 6B14773B CA97B43A 23FB8016 76BD207A
                436C6481 F1D2B907 8717461A 5B9D32E6 88F87748 544523B5 24B0D57D
                5EA77A27 75D2ECFA 032CFBDB F52FB378 61602790 04E57AE6 AF874E73
                03CE5329 9CCC041C 7BC308D8 2A5698F3 A8D0C382 71AE35F8 E9DBFBB6
                94B5C803 D89F7AE4 35DE236D 525F5475 9B65E372 FCD68EF2 0FA7111F
                9E4AFF73
                "#
                .trim()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .as_bytes(),
                16,
            )
            .unwrap(),
            g: BigInt::parse_bytes(b"02", 16).unwrap(),
            len: 2048,
        }
    }

    pub fn generate3072() -> Self {
        Self {
            n: BigInt::parse_bytes(
                r#"
                FFFFFFFF FFFFFFFF C90FDAA2 2168C234 C4C6628B 80DC1CD1 29024E08
                8A67CC74 020BBEA6 3B139B22 514A0879 8E3404DD EF9519B3 CD3A431B
                302B0A6D F25F1437 4FE1356D 6D51C245 E485B576 625E7EC6 F44C42E9
                A637ED6B 0BFF5CB6 F406B7ED EE386BFB 5A899FA5 AE9F2411 7C4B1FE6
                49286651 ECE45B3D C2007CB8 A163BF05 98DA4836 1C55D39A 69163FA8
                FD24CF5F 83655D23 DCA3AD96 1C62F356 208552BB 9ED52907 7096966D
                670C354E 4ABC9804 F1746C08 CA18217C 32905E46 2E36CE3B E39E772C
                180E8603 9B2783A2 EC07A28F B5C55DF0 6F4C52C9 DE2BCBF6 95581718
                3995497C EA956AE5 15D22618 98FA0510 15728E5A 8AAAC42D AD33170D
                04507A33 A85521AB DF1CBA64 ECFB8504 58DBEF0A 8AEA7157 5D060C7D
                B3970F85 A6E1E4C7 ABF5AE8C DB0933D7 1E8C94E0 4A25619D CEE3D226
                1AD2EE6B F12FFA06 D98A0864 D8760273 3EC86A64 521F2B18 177B200C
                BBE11757 7A615D6C 770988C0 BAD946E2 08E24FA0 74E5AB31 43DB5BFC
                E0FD108E 4B82D120 A93AD2CA FFFFFFFF FFFFFFFF
                "#
                .trim()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .as_bytes(),
                16,
            )
            .unwrap(),
            g: BigInt::parse_bytes(b"05", 16).unwrap(),
            len: 3072,
        }
    }

    pub fn generate4096() -> Self {
        Self {
            n: BigInt::parse_bytes(
                r#"
                FFFFFFFF FFFFFFFF C90FDAA2 2168C234 C4C6628B 80DC1CD1 29024E08
                8A67CC74 020BBEA6 3B139B22 514A0879 8E3404DD EF9519B3 CD3A431B
                302B0A6D F25F1437 4FE1356D 6D51C245 E485B576 625E7EC6 F44C42E9
                A637ED6B 0BFF5CB6 F406B7ED EE386BFB 5A899FA5 AE9F2411 7C4B1FE6
                49286651 ECE45B3D C2007CB8 A163BF05 98DA4836 1C55D39A 69163FA8
                FD24CF5F 83655D23 DCA3AD96 1C62F356 208552BB 9ED52907 7096966D
                670C354E 4ABC9804 F1746C08 CA18217C 32905E46 2E36CE3B E39E772C
                180E8603 9B2783A2 EC07A28F B5C55DF0 6F4C52C9 DE2BCBF6 95581718
                3995497C EA956AE5 15D22618 98FA0510 15728E5A 8AAAC42D AD33170D
                04507A33 A85521AB DF1CBA64 ECFB8504 58DBEF0A 8AEA7157 5D060C7D
                B3970F85 A6E1E4C7 ABF5AE8C DB0933D7 1E8C94E0 4A25619D CEE3D226
                1AD2EE6B F12FFA06 D98A0864 D8760273 3EC86A64 521F2B18 177B200C
                BBE11757 7A615D6C 770988C0 BAD946E2 08E24FA0 74E5AB31 43DB5BFC
                E0FD108E 4B82D120 A9210801 1A723C12 A787E6D7 88719A10 BDBA5B26
                99C32718 6AF4E23C 1A946834 B6150BDA 2583E9CA 2AD44CE8 DBBBC2DB
                04DE8EF9 2E8EFC14 1FBECAA6 287C5947 4E6BC05D 99B2964F A090C3A2
                233BA186 515BE7ED 1F612970 CEE2D7AF B81BDD76 2170481C D0069127
                D5B05AA9 93B4EA98 8D8FDDC1 86FFB7DC 90A6C08F 4DF435C9 34063199
                FFFFFFFF FFFFFFFF
                "#
                .trim()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("")
                .as_bytes(),
                16,
            )
            .unwrap(),
            g: BigInt::parse_bytes(b"05", 16).unwrap(),
            len: 4096,
        }
    }
}
