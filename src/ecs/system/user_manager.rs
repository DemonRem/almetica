/// Handles the users of an account. Users in TERA terminology are the player characters of an account.
use std::sync::Arc;

use anyhow::ensure;
use lazy_static::lazy_static;
use regex::Regex;
use shipyard::*;
use tracing::{debug, error, info_span};

use crate::ecs::component::{IncomingEvent, OutgoingEvent};
use crate::ecs::event::Event;
use crate::ecs::resource::WorldId;
use crate::ecs::system::send_event;
use crate::model::{Class, Customization, Gender, Race, Vec3, Vec3a};
use crate::protocol::packet::*;
use crate::Result;

pub fn user_manager_system(
    incoming_events: View<IncomingEvent>,
    mut outgoing_events: ViewMut<OutgoingEvent>,
    mut entities: EntitiesViewMut,
    world_id: UniqueView<WorldId>,
) {
    let span = info_span!("world", world_id = world_id.0);
    let _enter = span.enter();

    // TODO We need to persist users that are dropped. Create a field in a component to listen to (after X seconds after disconnect?)
    (&incoming_events).iter().for_each(|event| match &*event.0 {
        Event::RequestCanCreateUser { connection_id, .. } => {
            handle_can_create_user(*connection_id, &mut outgoing_events, &mut entities);
        }
        Event::RequestGetUserList { connection_id, .. } => {
            handle_user_list(*connection_id, &mut outgoing_events, &mut entities);
        }
        Event::RequestCheckUserName {
            connection_id,
            packet,
        } => {
            if let Err(e) =
                handle_check_user_name(&packet, *connection_id, &mut outgoing_events, &mut entities)
            {
                error!("Rejecting check user name request: {:?}", e);
                send_event(
                    assemble_check_user_name_response(*connection_id, false),
                    &mut outgoing_events,
                    &mut entities,
                );
            }
        }
        _ => { /* Ignore all other events */ }
    });
}

fn handle_user_list(
    connection_id: EntityId,
    outgoing_events: &mut ViewMut<OutgoingEvent>,
    entities: &mut EntitiesViewMut,
) {
    debug!("Get user list event incoming");

    // TODO Just a mock. Proper DB handling comes later.
    let event = OutgoingEvent(Arc::new(Event::ResponseGetUserList {
        connection_id,
        packet: SGetUserList {
            characters: vec![SGetUserListCharacter {
                custom_strings: vec![SGetUserListCharacterCustomString {
                    string: "Pantsu".to_string(),
                    id: 254_312,
                }],
                name: "Almetica".to_string(),
                details: vec![
                    0, 7, 0, 12, 0, 0, 0, 0, 26, 24, 20, 0, 0, 13, 7, 0, 16, 0, 16, 16, 0, 0, 0,
                    14, 17, 29, 12, 24, 26, 16, 7, 3,
                ],
                shape: vec![
                    1, 19, 16, 19, 19, 16, 19, 19, 19, 16, 16, 16, 16, 15, 15, 15, 16, 19, 10, 0,
                    22, 23, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
                guild_name: "".to_string(),
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
                face: 5,
                appearance: Customization {
                    data: vec![0, 0, 0, 0, 0, 0, 0, 0],
                },
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
                style_head_translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                style_head_translation_debug: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                style_faces_scale: 1.0,
                style_face_rotation: Vec3a { x: 0, y: 0, z: 0 },
                style_face_translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                style_face_translation_debug: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                style_back_scale: 1.0,
                style_back_rotation: Vec3a { x: 0, y: 0, z: 0 },
                style_back_translation: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                style_back_translation_debug: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                used_style_head_transform: false,
                is_new_character: false,
                tutorial_state: 0,
                show_style: true,
                appearance2: 100,
                achievement_points: 13565,
                laurel: 0,
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
        },
    }));

    send_event(event, outgoing_events, entities);
}

fn handle_can_create_user(
    connection_id: EntityId,
    outgoing_events: &mut ViewMut<OutgoingEvent>,
    entities: &mut EntitiesViewMut,
) {
    debug!("Can create user event incoming");

    // TODO check the database for current count of users once user table is implemented (hardwired max of 20).

    send_event(
        assemble_can_create_user_response(connection_id, true),
        outgoing_events,
        entities,
    );
}

fn handle_check_user_name(
    packet: &CCheckUserName,
    connection_id: EntityId,
    outgoing_events: &mut ViewMut<OutgoingEvent>,
    entities: &mut EntitiesViewMut,
) -> Result<()> {
    debug!("Check user name event incoming");

    ensure!(
        is_valid_user_name(&packet.name),
        "Invalid username provided"
    );

    // TODO check if the username is already present in the database

    send_event(
        assemble_check_user_name_response(connection_id, true),
        outgoing_events,
        entities,
    );

    Ok(())
}

/// Only alphanumeric characters are currently allowed. The client in rather limited with it's font.
fn is_valid_user_name(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^[[:alnum:]]+$"#).unwrap();
    }
    RE.is_match(text)
}

fn assemble_can_create_user_response(connection_id: EntityId, ok: bool) -> OutgoingEvent {
    OutgoingEvent(Arc::new(Event::ResponseCanCreateUser {
        connection_id,
        packet: SCanCreateUser { ok },
    }))
}

fn assemble_check_user_name_response(connection_id: EntityId, ok: bool) -> OutgoingEvent {
    OutgoingEvent(Arc::new(Event::ResponseCheckUserName {
        connection_id,
        packet: SCheckUserName { ok },
    }))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Instant;

    use shipyard::*;
    use sqlx::PgPool;

    use crate::ecs::component::{Connection, IncomingEvent, OutgoingEvent};
    use crate::ecs::event::Event;
    use crate::model::tests::db_test;
    use crate::Result;

    use super::*;

    fn setup_with_connection(pool: PgPool) -> (World, EntityId) {
        let world = World::new();
        world.add_unique(WorldId(0));
        world.add_unique(pool);

        let connection_id = world.run(
            |mut entities: EntitiesViewMut, mut connections: ViewMut<Connection>| {
                entities.add_entity(
                    &mut connections,
                    Connection {
                        verified: false,
                        version_checked: false,
                        region: None,
                        last_pong: Instant::now(),
                        waiting_for_pong: false,
                    },
                )
            },
        );

        (world, connection_id)
    }

    #[test]
    fn test_can_create_user_true() -> Result<()> {
        async fn test(pool: PgPool) -> Result<()> {
            let (world, connection_id) = setup_with_connection(pool);

            world.run(
                |mut entities: EntitiesViewMut, mut events: ViewMut<IncomingEvent>| {
                    for _i in 0..5 {
                        entities.add_entity(
                            &mut events,
                            IncomingEvent(Arc::new(Event::RequestCanCreateUser {
                                connection_id,
                                packet: CCanCreateUser {},
                            })),
                        );
                    }
                },
            );

            world.run(user_manager_system);

            world.run(|events: ViewMut<OutgoingEvent>| {
                let count = (&events)
                    .iter()
                    .filter(|event| match &*event.0 {
                        Event::ResponseCanCreateUser { packet, .. } => packet.ok,
                        _ => false,
                    })
                    .count();
                assert_eq!(count, 5);
            });

            Ok(())
        }
        db_test(test)
    }

    #[test]
    fn test_is_valid_user_name() {
        // Valid user names
        assert!(is_valid_user_name("Simple"));
        assert!(is_valid_user_name("Simple123"));
        assert!(is_valid_user_name("654562312"));

        // Invalid user names
        assert!(!is_valid_user_name("Simp le"));
        assert!(!is_valid_user_name("Simple!"));
        assert!(!is_valid_user_name("Simple "));
        assert!(!is_valid_user_name(" Simple"));
        assert!(!is_valid_user_name("´test`"));
        assert!(!is_valid_user_name(""));
        assert!(!is_valid_user_name(" "));
        assert!(!is_valid_user_name("\n"));
        assert!(!is_valid_user_name("\t"));
        assert!(!is_valid_user_name("기브스"));
        assert!(!is_valid_user_name("ダース"));
        assert!(!is_valid_user_name("การเดินทาง"));
        assert!(!is_valid_user_name("العربية"));
    }

    #[test]
    fn test_check_user_name_available() -> Result<()> {
        async fn test(pool: PgPool) -> Result<()> {
            let (world, connection_id) = setup_with_connection(pool);

            world.run(
                |mut entities: EntitiesViewMut, mut events: ViewMut<IncomingEvent>| {
                    for i in 0..5 {
                        entities.add_entity(
                            &mut events,
                            IncomingEvent(Arc::new(Event::RequestCheckUserName {
                                connection_id,
                                packet: CCheckUserName {
                                    name: format!("NotTakenUserName{}", i),
                                },
                            })),
                        );
                    }
                },
            );

            world.run(user_manager_system);

            world.run(|events: ViewMut<OutgoingEvent>| {
                let count = (&events)
                    .iter()
                    .filter(|event| match &*event.0 {
                        Event::ResponseCheckUserName { packet, .. } => packet.ok,
                        _ => false,
                    })
                    .count();
                assert_eq!(count, 5);
            });

            Ok(())
        }
        db_test(test)
    }

    #[test]
    fn test_check_user_name_invalid_username() -> Result<()> {
        async fn test(pool: PgPool) -> Result<()> {
            let (world, connection_id) = setup_with_connection(pool);

            world.run(
                |mut entities: EntitiesViewMut, mut events: ViewMut<IncomingEvent>| {
                    for i in 0..5 {
                        entities.add_entity(
                            &mut events,
                            IncomingEvent(Arc::new(Event::RequestCheckUserName {
                                connection_id,
                                packet: CCheckUserName {
                                    name: format!("H!x?or{}", i),
                                },
                            })),
                        );
                    }
                },
            );

            world.run(user_manager_system);

            world.run(|events: ViewMut<OutgoingEvent>| {
                let count = (&events)
                    .iter()
                    .filter(|event| match &*event.0 {
                        Event::ResponseCheckUserName { packet, .. } => !packet.ok,
                        _ => false,
                    })
                    .count();
                assert_eq!(count, 5);
            });

            Ok(())
        }
        db_test(test)
    }

    // TODO write test can_create_user_false() once user table is finished
    // TODO write test check_user_name_double_username once user table is finished
    // TODO write handle_user_list
}
