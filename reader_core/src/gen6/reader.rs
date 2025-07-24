use super::hook;
use crate::pnp;
use chrono::NaiveDate;
use pkm_rs::{Pk6, PokeCrypto};

pub struct Daycare {
    pub egg_seed: [u32; 2],
    pub is_egg_ready: bool,
    pub parent1: Option<Pk6>,
    pub parent2: Option<Pk6>,
}

struct Gen6Addresses {
    initial_seed: u32,
    mt_start: u32,
    mt_state_index: u32,
    tinymt_state: u32,
    party: u32,
    wild: u32,
    egg_ready_1: u32,
    egg_seed_1: u32,
    parent1_1: u32,
    parent2_1: u32,
    is_parent1_occupied_1: u32,
    is_parent2_occupied_1: u32,
    egg_ready_2: u32,
    egg_seed_2: u32,
    parent1_2: u32,
    parent2_2: u32,
    is_parent1_occupied_2: u32,
    is_parent2_occupied_2: u32,
    tidsid: u32,
    dex_nav_step: u32,
    dex_nav_chain: u32,
    radar_chain: u32,
    seed_save_variable: u32,
    mirage_spot_id: u32,
    mirage_spot_seed: u32,
    last_save_date_struct: u32,
}

const XY_ADDRESSES: Gen6Addresses = Gen6Addresses {
    initial_seed: 0x8c52844,
    mt_start: 0x8c5284c,
    mt_state_index: 0x8c52848,
    tinymt_state: 0x8c52808,
    party: 0x8ce1cf8,
    wild: 0x81ff744,
    egg_ready_1: 0x8c80124,
    egg_seed_1: 0x8c8012c,
    parent1_1: 0x8c7ff4c,
    parent2_1: 0x8c8003c,
    is_parent1_occupied_1: 0x8c7ff44,
    is_parent2_occupied_1: 0x8c80034,
    egg_ready_2: 0x8c80124,
    egg_seed_2: 0x8c8012c,
    parent1_2: 0x8c7ff4c,
    parent2_2: 0x8c8003c,
    is_parent1_occupied_2: 0x8c7ff44,
    is_parent2_occupied_2: 0x8c80034,
    tidsid: 0x8c79c3c,
    dex_nav_step: 0,
    dex_nav_chain: 0,
    radar_chain: 0x8d1b2b8,
    seed_save_variable: 0x8c6a6a4,
    mirage_spot_id: 0,
    mirage_spot_seed: 0,
    last_save_date_struct: 0,
};

const ORAS_ADDRESSES: Gen6Addresses = Gen6Addresses {
    initial_seed: 0x8c59e40,
    mt_start: 0x8c59e48,
    mt_state_index: 0x8c59e44,
    tinymt_state: 0x8c59e04,
    party: 0x8cfb26c,
    wild: 0x81ffa6c,
    egg_ready_1: 0x8c88358,
    egg_seed_1: 0x8c88360,
    parent1_1: 0x8c88180,
    parent2_1: 0x8c88270,
    is_parent1_occupied_1: 0x8c88178,
    is_parent2_occupied_1: 0x8c88268,
    egg_ready_2: 0x8c88548,
    egg_seed_2: 0x8c88550,
    parent1_2: 0x8c88370,
    parent2_2: 0x8c88460,
    is_parent1_occupied_2: 0x8c88368,
    is_parent2_occupied_2: 0x8c88458,
    tidsid: 0x8c81340,
    dex_nav_step: 0x8d3b508,
    dex_nav_chain: 0x8d3b57c,
    radar_chain: 0,
    seed_save_variable: 0x8c71db8,
    mirage_spot_id: 0x8c9ba4a,
    mirage_spot_seed: 0x8c6f9e0,
    last_save_date_struct: 0x8c6f848,
};

pub struct Gen6Reader {
    addrs: &'static Gen6Addresses,
}

impl Gen6Reader {
    pub fn xy() -> Self {
        Self {
            addrs: &XY_ADDRESSES,
        }
    }

    pub fn oras() -> Self {
        Self {
            addrs: &ORAS_ADDRESSES,
        }
    }

    pub fn tid(&self) -> u16 {
        pnp::read::<u16>(self.addrs.tidsid)
    }

    pub fn sid(&self) -> u16 {
        pnp::read::<u16>(self.addrs.tidsid + 2)
    }

    pub fn tsv(&self) -> u16 {
        (self.tid() ^ self.sid()) >> 4
    }

    pub fn trv(&self) -> u16 {
        (self.tid() ^ self.sid()) & 0xf
    }

    pub fn daycare1(&self) -> Daycare {
        Daycare {
            egg_seed: [
                pnp::read(self.addrs.egg_seed_1 + 0x4),
                pnp::read(self.addrs.egg_seed_1),
            ],
            is_egg_ready: pnp::read::<u8>(self.addrs.egg_ready_1) != 0,
            parent1: self.egg_parent(self.addrs.is_parent1_occupied_1, self.addrs.parent1_1),
            parent2: self.egg_parent(self.addrs.is_parent2_occupied_1, self.addrs.parent2_1),
        }
    }

    pub fn daycare2(&self) -> Daycare {
        Daycare {
            egg_seed: [
                pnp::read(self.addrs.egg_seed_2 + 0x4),
                pnp::read(self.addrs.egg_seed_2),
            ],
            is_egg_ready: pnp::read::<u8>(self.addrs.egg_ready_2) != 0,
            parent1: self.egg_parent(self.addrs.is_parent1_occupied_2, self.addrs.parent1_2),
            parent2: self.egg_parent(self.addrs.is_parent2_occupied_2, self.addrs.parent2_2),
        }
    }

    pub fn initial_seed(&self) -> u32 {
        pnp::read(self.addrs.initial_seed)
    }

    fn mt_state_index(&self) -> u32 {
        pnp::read(self.addrs.mt_state_index)
    }

    pub fn mt_state(&self) -> u32 {
        let index = self.mt_state_index();
        pnp::read(self.addrs.mt_start + if index != 624 { index * 4 } else { 0 })
    }

    pub fn tinymt_seed(&self) -> u32 {
        hook::tinymt_seed()
    }

    pub fn tinymt_state(&self) -> [u32; 4] {
        [
            pnp::read(self.addrs.tinymt_state),
            pnp::read(self.addrs.tinymt_state + 0x4),
            pnp::read(self.addrs.tinymt_state + 0x8),
            pnp::read(self.addrs.tinymt_state + 0xc),
        ]
    }

    fn read_pk6(&self, offset: u32) -> Pk6 {
        let bytes = pnp::read_array::<{ Pk6::STORED_SIZE }>(offset);
        Pk6::new_valid(bytes)
    }

    pub fn party_pkm(&self, slot: u32) -> Pk6 {
        let offset = (slot * 484) + self.addrs.party;
        self.read_pk6(offset)
    }

    pub fn wild_pkm(&self, slot: u32) -> Pk6 {
        let offset = (slot * 484) + self.addrs.wild;
        self.read_pk6(offset)
    }

    fn egg_parent(&self, is_present: u32, pkm: u32) -> Option<Pk6> {
        let is_parent_present = pnp::read::<u8>(is_present) != 0;

        if !is_parent_present {
            return None;
        }

        let parent = self.read_pk6(pkm);
        Some(parent)
    }

    pub fn radar_chain(&self) -> u8 {
        pnp::read(self.addrs.radar_chain)
    }

    pub fn dex_nav_step(&self) -> u8 {
        pnp::read(self.addrs.dex_nav_step)
    }

    pub fn dex_nav_chain(&self) -> u8 {
        pnp::read(self.addrs.dex_nav_chain)
    }

    pub fn seed_save_variable(&self) -> u32 {
        pnp::read(self.addrs.seed_save_variable)
    }

    pub fn mirage_spot_id(&self) -> u16 {
        pnp::read(self.addrs.mirage_spot_id)
    }

    pub fn mirage_spot_seed(&self) -> u32 {
        pnp::read(self.addrs.mirage_spot_seed)
    }

    pub fn time_penalty_hours(&self) -> u32 {
        pnp::read(self.addrs.last_save_date_struct + 0x30)
    }

    pub fn last_save_date(&self) -> NaiveDate {
        let last_save_date = self.addrs.last_save_date_struct;
        let year = pnp::read::<u32>(last_save_date + 0xc);
        let month = pnp::read::<u8>(last_save_date + 0x10);
        let day = pnp::read::<u8>(last_save_date + 0x11);
        NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap_or_default()
    }
}
