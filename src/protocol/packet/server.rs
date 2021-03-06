/// Module for server network packages.
use crate::model::{Class, Customization, Gender, Race, Region, Vec3, Vec3a};
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
pub struct SCanCreateUser {
    pub ok: bool,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SCheckVersion {
    pub ok: bool,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SCheckUserName {
    pub ok: bool,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SCreateUser {
    pub ok: bool,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SGetUserList {
    pub characters: Vec<SGetUserListCharacter>,
    pub veteran: bool,
    pub bonus_buf_sec: i32,
    pub max_characters: i32,
    pub first: bool,
    pub more: bool,
    pub left_del_time_account_over: i32,
    pub deletion_section_classify_level: i32,
    pub delete_character_expire_hour1: i32,
    pub delete_character_expire_hour2: i32,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SGetUserListCharacter {
    pub custom_strings: Vec<SGetUserListCharacterCustomString>,
    pub name: String,
    #[serde(with = "serde_bytes")]
    pub details: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub shape: Vec<u8>,
    pub guild_name: String,
    pub db_id: i32,
    pub gender: Gender,
    pub race: Race,
    pub class: Class,
    pub level: i32,
    pub hp: i64,
    pub mp: i32,
    pub world_id: i32,
    pub guard_id: i32,
    pub section_id: i32,
    pub last_logout_time: i64,
    pub is_deleting: bool,
    pub delete_time: i64,
    pub delete_remain_sec: i32,
    pub weapon: i32,
    pub earring1: i32,
    pub earring2: i32,
    pub body: i32,
    pub hand: i32,
    pub feet: i32,
    pub unk_item7: i32, // TODO research what it does
    pub ring1: i32,
    pub ring2: i32,
    pub underwear: i32,
    pub head: i32,
    pub face: i32,
    pub appearance: Customization,
    pub is_second_character: bool,
    pub admin_level: i32, // >0 = [GM] tag added to name
    pub is_banned: bool,
    pub ban_end_time: i64,
    pub ban_remain_sec: i32, // -1 = Permanent
    pub rename_needed: i32, // 0 = no, 1 = yes. Client will ask the player to rename the character once selected.
    pub weapon_model: i32,
    pub unk_model2: i32,
    pub unk_model3: i32,
    pub body_model: i32,
    pub hand_model: i32,
    pub feet_model: i32,
    pub unk_model7: i32,
    pub unk_model8: i32,
    pub unk_model9: i32,
    pub unk_model10: i32,
    pub unk_dye1: i32,
    pub unk_dye2: i32,
    pub weapon_dye: i32,
    pub body_dye: i32,
    pub hand_dye: i32,
    pub feet_dye: i32,
    pub unk_dye7: i32,
    pub unk_dye8: i32,
    pub unk_dye9: i32,
    pub underwear_dye: i32,
    pub style_back_dye: i32,
    pub style_head_dye: i32,
    pub style_face_dye: i32,
    pub style_head: i32,
    pub style_face: i32,
    pub style_back: i32,
    pub style_weapon: i32,
    pub style_body: i32,
    pub style_footprint: i32,
    pub style_body_dye: i32,
    pub weapon_enchant: i32,
    pub rest_bonus_xp: i64,
    pub max_rest_bonus_xp: i64,
    pub show_face: bool,
    pub style_head_scale: f32,
    pub style_head_rotation: Vec3a,
    pub style_head_translation: Vec3,
    pub style_head_translation_debug: Vec3,
    pub style_faces_scale: f32,
    pub style_face_rotation: Vec3a,
    pub style_face_translation: Vec3,
    pub style_face_translation_debug: Vec3,
    pub style_back_scale: f32,
    pub style_back_rotation: Vec3a,
    pub style_back_translation: Vec3,
    pub style_back_translation_debug: Vec3,
    pub used_style_head_transform: bool,
    pub is_new_character: bool,
    pub tutorial_state: i32,
    // TODO research what it does
    pub show_style: bool,
    pub appearance2: i32,
    pub achievement_points: i32,
    pub laurel: i32,
    // TODO enum: -1..5 (none, none, bronze, silver, gold, diamond, champion)
    pub position: i32,
    // 1..characterCount (position in character selection screen)
    pub guild_logo_id: i32,
    pub awakening_level: i32,
    pub has_broker_sales: bool,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SGetUserListCharacterCustomString {
    pub string: String,
    pub id: i32,
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
    pub string: String,
    pub id: i32,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SLoadingScreenControlInfo {
    pub custom_screen_enabled: bool,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SLoginAccountInfo {
    pub server_name: String,
    pub account_id: i64,
    pub integrity_iv: u32, // IV for the custom hash function of some client packets
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SLoginArbiter {
    pub success: bool,
    pub login_queue: bool,
    pub status: i32,
    pub unk1: u32,      // ignored by client
    pub region: Region, // must match CLoginArbiter.region
    pub pvp_disabled: bool,
    pub unk2: u16, // 0
    pub unk3: u16, // 0
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SPing {}

#[derive(Clone, Deserialize, Serialize, PartialEq, Debug)]
pub struct SRemainPlayTime {
    // 1 = P2P (active subscription)
    // 2 = P2P (no active subscription),
    // 3 = F2P (free-play event)
    // 4 = F2P (legacy restriction),
    // 5 = Premium, 6 = Basic
    pub account_type: u32,
    pub minutes_left: u32,
}

#[cfg(test)]
#[macro_use]
mod tests {
    use crate::protocol::serde::{from_vec, to_vec, Result};

    use super::*;

    packet_test!(
        name: test_account_package_list,
        data: vec![
            0x1, 0x0, 0x8, 0x0, 0x8, 0x0, 0x0, 0x0, 0xb2, 0x1, 0x0, 0x0, 0xff, 0xff, 0xff, 0x7f,
            0x0, 0x0, 0x0, 0x0,
        ],
        expected: SAccountPackageList {
            account_benefits: vec![SAccountPackageListEntry {
                package_id: 434,
                expiration_date: 2_147_483_647,
            }]
        }
    );

    packet_test!(
        name: test_can_create_user,
        data: vec![
            0x1,
        ],
        expected: SCanCreateUser {
            ok: true,
        }
    );

    packet_test!(
        name: test_check_username,
        data: vec![
            0x1
        ],
        expected: SCheckUserName {
            ok: true,
        }
    );

    packet_test!(
        name: test_check_version,
        data: vec![
            0x1
        ],
        expected: SCheckVersion {
            ok: true,
        }
    );

    packet_test!(
        name: test_create_user,
        data: vec![
            0x1
        ],
        expected: SCreateUser {
            ok: true,
        }
    );

    packet_test!(
        name: test_item_custom_string1,
        data: vec![
            0x0, 0x0, 0x0, 0x0, 0x11, 0x7f, 0x1c, 0x0, 0x0, 0x80, 0x0, 0x2,
        ],
        expected: SItemCustomString {
            custom_strings: Vec::with_capacity(0),
            game_id: 144_255_925_566_078_737,
        }
    );

    packet_test!(
        name: test_item_custom_string2,
        data: vec![
            0x1, 0x0, 0x10, 0x0, 0x11, 0x21, 0x11, 0x0, 0x0, 0x80, 0x0, 0x3, 0x10, 0x0, 0x0, 0x0, 0x1a, 0x0, 0x22, 0x11,
            0x2, 0x0, 0x50, 0x0, 0x61, 0x0, 0x6e, 0x0, 0x74, 0x0, 0x73, 0x0, 0x75, 0x0, 0x0, 0x0,
        ],
        expected: SItemCustomString {
            custom_strings: vec![SItemCustomStringEntry {
                string: "Pantsu".to_string(),
                id: 135_458,
            }],
            game_id: 216_313_519_603_261_713,
        }
    );

    packet_test!(
        name: test_get_user_list,
        data: vec![
            0x1, 0x0, 0x23, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xc, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x28, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x18, 0x0, 0x0, 0x0, 0x23, 0x0, 0x0, 0x0, 0x1, 0x0, 0xeb, 0x1, 0x3, 0x2, 0x15, 0x2,
            0x20, 0x0, 0x35, 0x2, 0x40, 0x0, 0x75, 0x2, 0x3, 0x85, 0x1e, 0x0, 0x1, 0x0, 0x0, 0x0, 0x4, 0x0, 0x0, 0x0, 0x1,
            0x0, 0x0, 0x0, 0x41, 0x0, 0x0, 0x0, 0x17, 0xd9, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0xd0, 0x7, 0x0, 0x0, 0x1, 0x0,
            0x0, 0x0, 0x2, 0x0, 0x0, 0x0, 0x8, 0x0, 0x0, 0x0, 0xf1, 0xe, 0x6b, 0x5e, 0x0, 0x0, 0x0, 0x0, 0x0, 0x80, 0x51,
            0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0xed, 0xb, 0x79, 0xa1, 0xd1, 0x6e, 0x0, 0x0, 0x8f, 0x78, 0x1, 0x0, 0x8e, 0x78,
            0x1, 0x0, 0x19, 0x78, 0x1, 0x0, 0x1b, 0x78, 0x1, 0x0, 0x1d, 0x78, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x88, 0x78,
            0x1, 0x0, 0x87, 0x78, 0x1, 0x0, 0x5b, 0xbb, 0x2, 0x0, 0x88, 0xc3, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x2, 0x3,
            0x4, 0x5, 0x6, 0x7, 0x8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x6d, 0xba,
            0x77, 0xa1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x7a, 0xb3, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x2d, 0x98, 0x2, 0x0, 0x61, 0xb6, 0x2, 0x0, 0x0, 0x0, 0x0, 0x0, 0x3c, 0x19, 0x19, 0x19, 0xf, 0x0, 0x0, 0x0,
            0x40, 0x46, 0x74, 0x11, 0x0, 0x0, 0x0, 0x0, 0x4c, 0x46, 0x74, 0x11, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x80,
            0x3f, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x80, 0x3f, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x80, 0x3f, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x64, 0x0, 0x0, 0x0, 0xfd, 0x34, 0x0,
            0x0, 0x2, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0xa9, 0x11, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xeb, 0x1, 0x0,
            0x0, 0xf5, 0x1, 0x68, 0xe1, 0x3, 0x0, 0x50, 0x0, 0x61, 0x0, 0x6e, 0x0, 0x74, 0x0, 0x73, 0x0, 0x75, 0x0, 0x0,
            0x0, 0x41, 0x0, 0x6c, 0x0, 0x6d, 0x0, 0x65, 0x0, 0x74, 0x0, 0x69, 0x0, 0x63, 0x0, 0x61, 0x0, 0x0, 0x0, 0x0,
            0x7, 0x0, 0xc, 0x0, 0x0, 0x0, 0x0, 0x1a, 0x18, 0x14, 0x0, 0x0, 0xd, 0x7, 0x0, 0x10, 0x0, 0x10, 0x10, 0x0, 0x0,
            0x0, 0xe, 0x11, 0x1d, 0xc, 0x18, 0x1a, 0x10, 0x7, 0x3, 0x1, 0x13, 0x10, 0x13, 0x13, 0x10, 0x13, 0x13, 0x13,
            0x10, 0x10, 0x10, 0x10, 0xf, 0xf, 0xf, 0x10, 0x13, 0xa, 0x0, 0x16, 0x17, 0x9, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x55, 0x0, 0x6e, 0x0, 0x6c, 0x0, 0x69, 0x0,
            0x6d, 0x0, 0x69, 0x0, 0x74, 0x0, 0x65, 0x0, 0x64, 0x0, 0x20, 0x0, 0x50, 0x0, 0x6f, 0x0, 0x77, 0x0, 0x65, 0x0,
            0x72, 0x0, 0x0, 0x0,
        ],
        expected: SGetUserList {
            characters: vec![SGetUserListCharacter {
                custom_strings: vec![SGetUserListCharacterCustomString {
                    string: "Pantsu".to_string(),
                    id: 254_312,
                }],
                name: "Almetica".to_string(),
                details: vec![
                    0, 7, 0, 12, 0, 0, 0, 0, 26, 24, 20, 0, 0, 13, 7, 0, 16, 0, 16, 16, 0, 0, 0, 14, 17, 29, 12, 24, 26,
                    16, 7, 3,
                ],
                shape: vec![
                    1, 19, 16, 19, 19, 16, 19, 19, 19, 16, 16, 16, 16, 15, 15, 15, 16, 19, 10, 0, 22, 23, 9, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0,
                ],
                guild_name: "Unlimited Power".to_string(),
                db_id: 2_000_131,
                gender: Gender::Female,
                race: Race::ElinPopori,
                class: Class::Lancer,
                level: 65,
                hp: 121_111,
                mp: 2000,
                world_id: 1,
                guard_id: 2,
                section_id: 8,
                last_logout_time: 1_584_074_481,
                is_deleting: false,
                delete_time: 86400,
                delete_remain_sec: -1_585_902_611,
                weapon: 28369,
                earring1: 96399,
                earring2: 96398,
                body: 96281,
                hand: 96283,
                feet: 96285,
                unk_item7: 0,
                ring1: 96392,
                ring2: 96391,
                underwear: 179_035,
                head: 50056,
                face: 0,
                appearance: Customization(vec![1,2,3,4,5,6,7,8]),
                is_second_character: false,
                admin_level: 0,
                is_banned: false,
                ban_end_time: 0,
                ban_remain_sec: -1_585_989_011,
                rename_needed: 0,
                weapon_model: 0,
                unk_model2: 0,
                unk_model3: 0,
                body_model: 0,
                hand_model: 0,
                feet_model: 0,
                unk_model7: 0,
                unk_model8: 0,
                unk_model9: 0,
                unk_model10: 0,
                unk_dye1: 0,
                unk_dye2: 0,
                weapon_dye: 0,
                body_dye: 0,
                hand_dye: 0,
                feet_dye: 0,
                unk_dye7: 0,
                unk_dye8: 0,
                unk_dye9: 0,
                underwear_dye: 0,
                style_back_dye: 0,
                style_head_dye: 0,
                style_face_dye: 0,
                style_head: 177_018,
                style_face: 0,
                style_back: 0,
                style_weapon: 170_029,
                style_body: 177_761,
                style_footprint: 0,
                style_body_dye: 421_075_260,
                weapon_enchant: 15,
                rest_bonus_xp: 292_832_832,
                max_rest_bonus_xp: 292_832_844,
                show_face: true,
                style_head_scale: 1.0,
                style_head_rotation: Vec3a { x: 0, y: 0, z: 0 },
                style_head_translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                style_head_translation_debug: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                style_faces_scale: 1.0,
                style_face_rotation: Vec3a { x: 0, y: 0, z: 0 },
                style_face_translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                style_face_translation_debug: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                style_back_scale: 1.0,
                style_back_rotation: Vec3a { x: 0, y: 0, z: 0 },
                style_back_translation: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                style_back_translation_debug: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
                used_style_head_transform: false,
                is_new_character: false,
                tutorial_state: 0,
                show_style: true,
                appearance2: 100,
                achievement_points: 13565,
                laurel: 2,
                position: 1,
                guild_logo_id: 4521,
                awakening_level: 0,
                has_broker_sales: false,
            }],
            veteran: false,
            bonus_buf_sec: 0,
            max_characters: 12,
            first: true,
            more: false,
            left_del_time_account_over: 0,
            deletion_section_classify_level: 40,
            delete_character_expire_hour1: 0,
            delete_character_expire_hour2: 24,
        }
    );

    packet_test!(
        name: test_guild_name,
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
            game_id: 216_313_519_606_268_719,
        }
    );

    packet_test!(
        name: test_image_data,
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
        name: test_loading_screen_control_info,
        data: vec![
            0x1,
        ],
        expected: SLoadingScreenControlInfo {
            custom_screen_enabled: true,
        }
    );

    packet_test!(
        name: test_login_arbiter,
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
        name: test_ping,
        data: vec![],
        expected: SPing {}
    );

    packet_test!(
        name: test_remain_play_time,
        data: vec![
            0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0,
        ],
        expected: SRemainPlayTime {
            account_type: 6,
            minutes_left: 0,
        }
    );

    packet_test!(
        name: test_login_account_info,
        data: vec![
            0x12, 0x0, 0xfe, 0x5c, 0x7, 0x0, 0x0, 0x0, 0x0, 0x0, 0xfe, 0xfe, 0xfe, 0xfe, 0x50, 0x0,
            0x6c, 0x0, 0x61, 0x0, 0x6e, 0x0, 0x65, 0x0, 0x74, 0x0, 0x44, 0x0, 0x42, 0x0, 0x5f, 0x0,
            0x32, 0x0, 0x37, 0x0, 0x0, 0x0,
        ],
        expected: SLoginAccountInfo {
            server_name: "PlanetDB_27".to_string(),
            account_id: 482_558,
            integrity_iv: 4278124286,
        }
    );
}
