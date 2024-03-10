#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::il2cpp::object::Array;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use skyline::patching::Patch;
use engage::random::Random;
use engage::{
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    singleton::SingletonClass,
    gamevariable::*,
    gameuserdata::*,
    force::*,
    gamedata::unit::*,
    proc::{ProcInstFields,Bindable,desc::ProcDesc, ProcInst},
};
use std::sync::{Mutex, LazyLock};
use serde::{Deserialize, Serialize};

pub const EMBLEM_GIDS: &[&str] = &["GID_マルス", "GID_シグルド", "GID_セリカ", "GID_ミカヤ", "GID_ロイ", "GID_リーフ", "GID_ルキナ", "GID_リン", "GID_アイク", "GID_ベレト", "GID_カムイ", "GID_エイリーク", "GID_エーデルガルト", "GID_チキ", "GID_ヘクトル", "GID_ヴェロニカ", "GID_セネリオ", "GID_カミラ", "GID_クロム"];

#[derive(Default, Serialize, Deserialize)]
pub struct DeploymentConfig {
    deployment_type: i32,
    random_emblems: bool,
}

impl DeploymentConfig {
    pub fn new() -> Self {
        let config_content = std::fs::read_to_string("sd:/engage/config/deployment.toml");
        // If the file is read to a string or there is no failure, parse into the config struct.
        if config_content.is_ok() {
            let content = config_content.unwrap();
            let config = toml::from_str(&content);
            if config.is_ok() {
                println!("Deployment Config file was parsed with no issues.");
                let config = config.unwrap();
                config
            } else {
                // This is mostly intended to create a new file if more items are added to the struct
                println!("Deployment Config: Config file could not be parsed, a default config file has been created.");
                let config = DeploymentConfig::default();
                config
            }
        } else {
            // If the file could not be read to a string then create a new file with default values.
            println!("Deployment Config: The config file was either missing or unable to be read, creating new toml.");
            let config = DeploymentConfig::default();
            config.save();
            config
        }
    }
    pub fn default() -> Self {
        let config = DeploymentConfig  {
            deployment_type: 0,
            random_emblems: false,
        };
        config
    }
    pub fn save(&self) {
        let out_toml = toml::to_string_pretty(&self).unwrap();
        std::fs::write("sd:/engage/config/deployment.toml", out_toml).expect("should be able to write to write default configuration");
    }
}

pub static CONFIG: LazyLock<Mutex<DeploymentConfig>> = LazyLock::new(|| DeploymentConfig::new().into() );

#[skyline::from_offset(0x02334570)]
pub fn god_pool_try_get(gid: &Il2CppString, include_reserved: bool, method_info: OptionalMethod) -> Option<&GodUnit>;

#[skyline::from_offset(0x0233eae0)]
pub fn god_unit_escaped(this: &GodUnit, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01c616f0)]
pub fn remove_all_rings(this: u64, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "SetStatus")]
pub fn unit_set_status(this: &Unit, status: i64, method_info: OptionalMethod);

#[unity::from_offset("App","MapDispos", "GetSortieLimit")]
pub fn get_sortie_limit(method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Force", "Transfer")]
pub fn force_transfer(this: &Force, force_type: i32, is_last: bool, method_info: OptionalMethod);

#[unity::from_offset("App", "Force", "GetHeroUnit")]
pub fn force_get_hero_unit(this: &Force, method_info: OptionalMethod) -> &'static Unit;

#[skyline::from_offset(0x01c54fa0)]
pub fn force_get_unit_from_pid(pid: &Il2CppString, relay: bool, method_info: OptionalMethod) -> Option<&'static Unit>;

#[unity::from_offset("App", "Force", "set_First")]
pub fn force_set_first(this: &Force, value: &Unit, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "Transfer")]
pub fn unit_transfer(this: &Unit, force: i32, is_last: bool, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "TryCreateActor")]
pub fn unit_try_create_actor(this: &Unit, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "Unit", "SetGodUnit")]
pub fn unit_set_god_unit(this: &Unit, god: &GodUnit, method_info: OptionalMethod);

#[unity::from_offset("App", "GameUserData", "IsEvilMap")]
pub fn is_evil_map(this: &GameUserData, method_info: OptionalMethod) -> bool;

//Functions used 
pub fn get_unit_rating(this: &Unit) -> i32 {
    let mut result: i32 = 0;
    for x in 1..8 {
        result += this.get_capability(x as i32, false);
    }
    result
}

pub fn get_emblem_list() -> Vec<&'static str> {
    let mut result: Vec<&str> = Vec::new();
    unsafe {
        for x in EMBLEM_GIDS {
            let god_unit = god_pool_try_get(x.into(), true, None);
            if god_unit.is_some() {
                if !god_unit_escaped(god_unit.unwrap(), None) {
                    result.push(x);
                }
            }
        }
    }
    result
}
//Hook to function that creates the sortie deploy positions to do deployment stuff
#[unity::hook("App", "MapDispos", "CreatePlayerTeam")]
pub fn create_player_team(group: &Il2CppString, method_info: OptionalMethod){
    call_original!(group, method_info);
    let player_force = Force::get(ForceType::Player).unwrap();
    let absent_force = Force::get(ForceType::Absent).unwrap();
    let max_player = player_force.get_count();
    let mut player_count = 0;
    let absent_count = absent_force.get_count();
    let rng = Random::get_game();
    let config = CONFIG.lock().unwrap();
    config.save();
    unsafe {
        if absent_count == 0 || is_evil_map( GameUserData::get_instance(), None) { 
            Patch::in_text(0x01d785f8).bytes(&[0xc0, 0x00, 0x00,0x36]);
            Patch::in_text(0x01d77028).bytes(&[0xc0, 0x00, 0x00, 0x36]);
            if is_evil_map(GameUserData::get_instance(), None) { return; }
        }
        if config.random_emblems && config.deployment_type == 0 {
            Patch::in_text(0x01d785f8).bytes(&[0xc0, 0x00, 0x00,0x36]);
            Patch::in_text(0x01d77028).nop();
            remove_all_rings(0, None);
            let emblem_list = get_emblem_list();
            let mut emblem_count = emblem_list.len();
            let mut set_emblems: [bool; 20] = [false; 20];
            if emblem_count > max_player as usize {
                emblem_count = max_player as usize;
            }
            let mut current_emblem_count = 0;
            let mut force_iter = Force::iter(player_force);
            while let Some(unit) = force_iter.next() {
                let mut value = rng.get_value(emblem_list.len() as i32) as usize;
                while set_emblems[value] == true {
                    value = rng.get_value(emblem_list.len() as i32) as usize;
                }
                let god_unit = god_pool_try_get(emblem_list[value].into(), true, None).unwrap();
                unit_set_god_unit(unit, god_unit, None);
                current_emblem_count += 1;
                set_emblems[value] = true;
                if current_emblem_count == emblem_count {  
                    break;
                } 
            }
            return;
        }
        //Normal Deployment
        if config.deployment_type == 0 || absent_count == 0 {
            Patch::in_text(0x01d785f8).bytes(&[0xc0, 0x00, 0x00,0x36]);
            return;
        } 
        // Move currently deployed units to absent and then move back hero unit (Alear or Veyle)
        force_transfer(player_force, 3, true, method_info);

        //Transfer Dead
        if config.deployment_type != 0 {
            force_transfer(Force::get(ForceType::Dead).unwrap(), 3, true, method_info);
        }
        let hero_unit = force_get_hero_unit(absent_force, method_info);
        unit_transfer(hero_unit, 0, true, None);
        unit_try_create_actor(hero_unit, None);
        if !GameUserData::is_encount_map() {
            unit_set_status(hero_unit, 20, None);
        }
        player_count = player_force.get_count();

        // Lowest Rating Deployment
        if config.deployment_type == 1 {
            Patch::in_text(0x01d785f8).nop();
            Patch::in_text(0x01d77028).bytes(&[0xc0, 0x00, 0x00, 0x36]);
            while player_count < max_player {
                let mut pid: &Il2CppString = "PID_unit".into();
                let mut mpid: &Il2CppString = "MPID_unit".into();
                let mut capability_score = 99999;
    
                let mut force_iter = Force::iter(absent_force);
                while let Some(unit) = force_iter.next() {
                    let cap = get_unit_rating(unit);
                    if cap < capability_score {
                        capability_score = cap;
                        pid = unit.person.pid;
                        mpid = unit.person.get_name().unwrap();
                    }
                }
                println!("{} is deployed with rating of {}", mpid.get_string().unwrap(), capability_score);
                let move_unit = force_get_unit_from_pid(pid, false, None);
                if move_unit.is_some() {
                    let unit = move_unit.unwrap();
                    unit_transfer(unit, 0, true, None);
                    unit_try_create_actor(unit, None);
                }
                player_count = player_force.get_count();
            }
        }
        // Random Deployment
        else if config.deployment_type == 2  {
            Patch::in_text(0x01d785f8).nop();
            Patch::in_text(0x01d77028).bytes(&[0xc0, 0x00, 0x00, 0x36]);

            while player_count < max_player {
                let rng_range = absent_force.get_count();
                let mut index = 0;
                let value = rng.get_value(rng_range);
                let mut force_iter = Force::iter(absent_force);
                while let Some(unit) = force_iter.next() {
                    if index == value {
                        unit_transfer(unit, 0, true, None);
                        unit_try_create_actor(unit, None);
                        player_count = player_force.get_count();
                        break;
                    }
                    index += 1;
                }
            }
        }
        // Random Emblems
        if config.random_emblems {
            Patch::in_text(0x01d77028).nop();
            remove_all_rings(0, None);
            let emblem_list = get_emblem_list();
            let mut emblem_count = emblem_list.len();
            let mut set_emblems: [bool; 20] = [false; 20];
            if emblem_count > max_player as usize {
                emblem_count = max_player as usize;
            }
            let mut current_emblem_count = 0;
            let mut force_iter = Force::iter(player_force);
            while let Some(unit) = force_iter.next() {
                let mut value = rng.get_value(emblem_list.len() as i32) as usize;
                while set_emblems[value] == true {
                    value = rng.get_value(emblem_list.len() as i32) as usize;
                }
                let god_unit = god_pool_try_get(emblem_list[value].into(), true, None).unwrap();
                unit_set_god_unit(unit, god_unit, None);
                current_emblem_count += 1;
                set_emblems[value] = true;
                if current_emblem_count == emblem_count {
                    break;                    
                } 
            }
        }
        else { Patch::in_text(0x01d77028).bytes(&[0xc0, 0x00, 0x00, 0x36]);}
    }
}

pub struct DeploymentMod;
impl ConfigBasicMenuItemSwitchMethods for DeploymentMod {
    fn init_content(this: &mut ConfigBasicMenuItem){ }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let result = ConfigBasicMenuItem::change_key_value_i(CONFIG.lock().unwrap().deployment_type, 0, 2, 1);
        if CONFIG.lock().unwrap().deployment_type != result {
            CONFIG.lock().unwrap().deployment_type = result;
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            //CONFIG.lock().unwrap().save();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        unsafe {
        match CONFIG.lock().unwrap().deployment_type {

            1 => { this.help_text ="Lowest rating units will be deployed.".into(); },
            2 => { this.help_text = "Units will be deployed at random.".into(); }
            _ => { this.help_text = "Normal Deployment".into(); },
        }
        }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        unsafe {
            match CONFIG.lock().unwrap().deployment_type {
                1 => { this.command_text = "Lowest Rating".into(); },
                2 => { this.command_text = "Random".into(); }
                _ => { this.command_text = "Default".into(); },
            }
        }
    }
}

pub struct EmblemMod;
impl ConfigBasicMenuItemSwitchMethods for EmblemMod {
    fn init_content(this: &mut ConfigBasicMenuItem){}
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let result = ConfigBasicMenuItem::change_key_value_b(CONFIG.lock().unwrap().random_emblems);
        if CONFIG.lock().unwrap().random_emblems != result {
            CONFIG.lock().unwrap().random_emblems = result;
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            //CONFIG.lock().unwrap().save();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if CONFIG.lock().unwrap().random_emblems { this.help_text = "Emblems will be randomized onto deployed units.".into();  }
        else { this.help_text = "Emblems are not randomized onto deployed units.".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        if CONFIG.lock().unwrap().random_emblems { this.command_text = "On".into();  }
        else { this.command_text = "Off".into(); }
    }
}

#[no_mangle]
extern "C" fn deploy_create() -> &'static mut ConfigBasicMenuItem { 
    ConfigBasicMenuItem::new_switch::<DeploymentMod>("Deployment Mode")
 } 
 #[no_mangle]
extern "C" fn emblem_create() -> &'static mut ConfigBasicMenuItem { 
    ConfigBasicMenuItem::new_switch::<EmblemMod>("Random Emblems")
 } 

#[skyline::main(name = "deployment")]
pub fn main() {
    skyline::install_hooks!( create_player_team );
    cobapi::install_global_game_setting(deploy_create);
    cobapi::install_global_game_setting(emblem_create);
}
