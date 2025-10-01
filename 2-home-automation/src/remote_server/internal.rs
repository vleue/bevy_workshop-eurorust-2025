#![allow(dead_code)]

use bevy::{platform::collections::HashMap, prelude::*};

use crate::{lights::Light, remote_server::Device};

use std::{thread, time::Duration};

struct Schedule<'a> {
    device: Device,
    changes: &'a [(u32, bool)],
}

const TYPICAL_DAY: &[Schedule<'static>] = &[
    Schedule {
        device: Device::Light(Light::Kitchen),
        changes: &[
            (5 * 60 + 30, true),
            (7 * 60, false),
            (17 * 60 + 5, true),
            (17 * 60 + 35, false),
            (19 * 60, true),
            (21 * 60, false),
        ],
    },
    Schedule {
        device: Device::Light(Light::Hall),
        changes: &[(17 * 60, true), (17 * 60 + 30, false)],
    },
    Schedule {
        device: Device::Light(Light::Hallway),
        changes: &[
            (5 * 60 + 20, true),
            (6 * 60 + 5, false),
            (21 * 60 + 55, true),
            (22 * 60 + 35, false),
        ],
    },
    Schedule {
        device: Device::Light(Light::Bedroom1),
        changes: &[
            (5 * 60 + 17, true),
            (6 * 60, false),
            (22 * 60, true),
            (23 * 60 + 45, false),
        ],
    },
    Schedule {
        device: Device::Light(Light::Bathroom1),
        changes: &[
            (5 * 60 + 25, true),
            (5 * 60 + 28, false),
            (22 * 60 + 10, true),
            (22 * 60 + 30, false),
        ],
    },
    Schedule {
        device: Device::Light(Light::Bedroom2),
        changes: &[
            (5 * 60 + 15, true),
            (6 * 60 + 3, false),
            (22 * 60, true),
            (23 * 60 + 55, false),
        ],
    },
    Schedule {
        device: Device::Light(Light::Bathroom2),
        changes: &[
            (5 * 60 + 15, true),
            (5 * 60 + 25, false),
            (22 * 60 + 15, true),
            (22 * 60 + 30, false),
        ],
    },
    Schedule {
        device: Device::Light(Light::LivingRoom1),
        changes: &[
            (6 * 60, true),
            (6 * 60 + 30, false),
            (17 * 60 + 30, true),
            (22 * 60 + 30, false),
        ],
    },
    Schedule {
        device: Device::Light(Light::LivingRoom2),
        changes: &[
            (6 * 60, true),
            (6 * 60 + 30, false),
            (17 * 60 + 30, true),
            (22 * 60 + 30, false),
        ],
    },
];

// Fake async function: simulate an external call to get the state at a given time
pub async fn get_current_state(current_time: u32) -> HashMap<Device, bool> {
    let mut state = HashMap::new();
    thread::sleep(Duration::from_millis(100));
    for schedule in TYPICAL_DAY.iter() {
        for (new_time, new_state) in schedule.changes {
            if *new_time < current_time {
                state.insert(schedule.device, *new_state);
            } else {
                break;
            }
        }
        if !state.contains_key(&schedule.device) {
            state.insert(schedule.device, schedule.changes.last().unwrap().1);
        }
    }
    state
}

/// Fake async function: external call to the device API
pub async fn change_state(_device: Device, _new_state: bool) {
    // Nothing to do in our fake remote server
}

/// Fake async function: external call to the device API
pub async fn get_history(device: Device) -> Vec<(u32, bool)> {
    let Some(schedule) = TYPICAL_DAY.iter().find(|d| d.device == device) else {
        return Vec::new();
    };
    schedule.changes.to_vec()
}
