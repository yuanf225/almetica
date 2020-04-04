/// Module for server network packages.
use crate::model::Region;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SAccountPackageList {
    pub account_benefits: Vec<SAccountPackageListEntry>,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SAccountPackageListEntry {
    pub package_id: u32,
    pub expiration_date: i64,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SCheckVersion {
    pub ok: bool,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SGuildName {
    pub guild_name: String,
    pub guild_rank: String,
    pub guild_title: String,
    pub guild_logo: String,
    pub game_id: u64,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SImageData {
    pub name: String,

    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SItemCustomString {
    pub custom_strings: Vec<SItemCustomStringEntry>,
    pub game_id: u64,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SItemCustomStringEntry {
    pub id: u32,
    pub string: String,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SLoadingScreenControlInfo {
    pub custom_screen_enabled: bool,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SLoginAccountInfo {
    pub server_name: String,
    pub account_id: u64,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SLoginArbiter {
    pub success: bool,
    pub login_queue: bool,
    pub status: u32,
    // ignored by client
    pub unk1: u32,
    // must match CLoginArbiter.region
    pub region: Region,
    pub pvp_disabled: bool,
    // checked against 0
    pub unk2: u16,
    // checked against level
    pub unk3: u16,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SRemainPlayTime {
    // 1 = P2P (active subscription), 2 = P2P (no active subscription),
    //3 = F2P (free-play event), 4 = F2P (legacy restriction),
    //5 = Premium, 6 = Basic
    pub account_type: i32,
    pub minutes_left: i32,
}

#[cfg(test)]
#[macro_use]
mod tests {
    use super::*;
    use crate::protocol::serde::{from_vec, to_vec, Error};

    packet_test!(
        name: test_s_account_package_list,
        data: vec![
            0x1, 0x0, 0x8, 0x0, 0x8, 0x0, 0x0, 0x0, 0xb2, 0x1, 0x0, 0x0, 0xff, 0xff, 0xff, 0x7f,
            0x0, 0x0, 0x0, 0x0,
        ],
        expected: SAccountPackageList {
            account_benefits: vec![SAccountPackageListEntry {
                package_id: 434,
                expiration_date: 2147483647,
            }]
        }
    );

    packet_test!(
        name: test_s_check_version,
        data: vec![
            0x1
        ],
        expected: SCheckVersion {
            ok: true,
        }
    );

    packet_test!(
        name: test_s_item_custom_string1,
        data: vec![
            0x0, 0x0, 0x0, 0x0, 0x11, 0x7f, 0x1c, 0x0, 0x0, 0x80, 0x0, 0x2,
        ],
        expected: SItemCustomString {
            custom_strings: Vec::with_capacity(0),
            game_id: 144255925566078737,
        }
    );

    packet_test!(
        name: test_s_item_custom_string2,
        data: vec![
            0x1, 0x0, 0x10, 0x0, 0x3f, 0x3, 0x1c, 0x0, 0x0, 0x80, 0x0, 0x3, 0x10, 0x0, 0x0, 0x0,
            0x1c, 0x0, 0xff, 0xb6, 0x2, 0x0, 0x47, 0x0, 0x61, 0x0, 0x6e, 0x0, 0x74, 0x0, 0x73, 0x0,
            0x75, 0x0, 0x0, 0x0,
        ],
        expected: SItemCustomString {
            custom_strings: vec![SItemCustomStringEntry {
                id: 3070165020,
                string: "Gantsu".to_string(),
            }],
            game_id: 216313519603974975,
        }
    );

    packet_test!(
        name: test_s_guild_name,
        data: vec![
            0x14, 0x0, 0x2e, 0x0, 0x48, 0x0, 0x58, 0x0, 0x2f, 0x3, 0x3f, 0x0, 0x0, 0x80, 0x0, 0x3,
            0x4d, 0x0, 0x61, 0x0, 0x6e, 0x0, 0x68, 0x0, 0x75, 0x0, 0x6e, 0x0, 0x74, 0x0, 0x65, 0x0,
            0x72, 0x0, 0x20, 0x0, 0x4f, 0x0, 0x47, 0x0, 0x0, 0x0, 0x46, 0x0, 0x6f, 0x0, 0x72, 0x0,
            0x20, 0x0, 0x74, 0x0, 0x68, 0x0, 0x65, 0x0, 0x20, 0x0, 0x77, 0x0, 0x69, 0x0, 0x6e, 0x0,
            0x21, 0x0, 0x0, 0x0, 0x47, 0x0, 0x61, 0x0, 0x6e, 0x0, 0x74, 0x0, 0x73, 0x0, 0x75, 0x0,
            0x7e, 0x0, 0x0, 0x0, 0x67, 0x0, 0x75, 0x0, 0x69, 0x0, 0x6c, 0x0, 0x64, 0x0, 0x6c, 0x0,
            0x6f, 0x0, 0x67, 0x0, 0x6f, 0x0, 0x5f, 0x0, 0x39, 0x0, 0x39, 0x0, 0x5f, 0x0, 0x31, 0x0,
            0x31, 0x0, 0x31, 0x0, 0x31, 0x0, 0x5f, 0x0, 0x39, 0x0, 0x31, 0x0, 0x0, 0x0,
        ],
        expected: SGuildName {
            guild_name: "Manhunter OG".to_string(),
            guild_rank: "For the win!".to_string(),
            guild_title: "Gantsu~".to_string(),
            guild_logo: "guildlogo_99_1111_91".to_string(),
            game_id: 216313519606268719,
        }
    );

    packet_test!(
        name: test_s_image_data,
        data: vec![
            0xa, 0x0, 0x36, 0x0, 0xe, 0x00, 0x67, 0x0, 0x75, 0x0, 0x69, 0x0, 0x6c, 0x0, 0x64, 0x0,
            0x6c, 0x0, 0x6f, 0x0, 0x67, 0x0, 0x6f, 0x0, 0x5f, 0x0, 0x32, 0x0, 0x38, 0x0, 0x5f, 0x0,
            0x35, 0x0, 0x35, 0x0, 0x35, 0x0, 0x31, 0x0, 0x39, 0x0, 0x5f, 0x0, 0x36, 0x0, 0x30, 0x0,
            0x0, 0x0, 0x54, 0x45, 0x52, 0x41, 0x1, 0x0, 0x0, 0x0, 0x40, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        expected: SImageData {
            name: "guildlogo_28_55519_60".to_string(),
            data: vec![
                0x54, 0x45, 0x52, 0x41, 0x1, 0x0, 0x0, 0x0, 0x40, 0x0, 0x0, 0x0, 0x0, 0x0,
            ],
        }
    );

    packet_test!(
        name: test_s_loading_screen_control_info,
        data: vec![
            0x1,
        ],
        expected: SLoadingScreenControlInfo {
            custom_screen_enabled: true,
        }
    );

    packet_test!(
        name: test_s_login_arbiter,
        data: vec![
            0x1, 0x0, 0x2, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x6, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0,
        ],
        expected: SLoginArbiter {
            success: true,
            login_queue: false,
            status: 65538,
            unk1: 0,
            region: Region::Europe,
            pvp_disabled: true,
            unk2: 0,
            unk3: 0,
        }
    );

    packet_test!(
        name: test_s_remain_play_time,
        data: vec![
            0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        expected: SRemainPlayTime {
            account_type: 6,
            minutes_left: 0,
        }
    );

    packet_test!(
        name: test_s_login_account_info,
        data: vec![
            0xe, 0x0, 0xfe, 0x5c, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, 0x50, 0x0, 0x6c, 0x0, 0x61, 0x0, 0x6e, 0x0, 0x65, 0x0,
            0x74, 0x0, 0x44, 0x0, 0x42, 0x0, 0x5f, 0x0, 0x32, 0x0, 0x37, 0x0, 0x0, 0x0,
        ],
        expected: SLoginAccountInfo {
            server_name: "PlanetDB_27".to_string(),
            account_id: 482558,
        }
    );
}
