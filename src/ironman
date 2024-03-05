#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::il2cpp::object::Array;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use skyline::patching::Patch;
use engage::{
    singleton::SingletonClass,
    gamevariable::*,
    gameuserdata::*,
    menu::{
        config::ConfigBasicMenuItem,
        BasicMenu, BasicMenuItem,
    },
    proc::{ProcInstFields,Bindable,desc::ProcDesc, ProcInst},
};

#[unity::class("App", "GameUserGlobalData")]
pub struct GameUserGlobalData {}

impl GameUserGlobalData {
    pub fn get_instance() -> &'static mut GameUserGlobalData {
        let idk = get_generic_class!(SingletonClass<GameUserGlobalData>).unwrap();
        let pointer = unsafe { &*(idk.rgctx_data as *const Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 6]) };
        let get_instance =
            unsafe { std::mem::transmute::<_, extern "C" fn(OptionalMethod) -> &'static mut GameUserGlobalData>(pointer[5].method_ptr) };
            
        get_instance(Some(&pointer[5]))
    }
    pub fn get_last_save_data_index() -> i32 {
        let instance = Self::get_instance();
        unsafe {
            gugd_get_last_save_data_index(instance, None)
        }
    }
    pub fn get_last_save_data_type() -> i32 {
        let instance = Self::get_instance();
        unsafe {
            gugd_get_last_save_data_type(instance, None)
        }
    }
}

#[unity::from_offset("App","GameUserGlobalData","get_LastSaveDataIndex")]
fn gugd_get_last_save_data_index(this: &GameUserGlobalData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App","GameUserGlobalData","get_LastSaveDataType")]
fn gugd_get_last_save_data_type(this: &GameUserGlobalData, method_info: OptionalMethod) -> i32;

#[skyline::hook(offset=0x0251ba60)]
pub fn set_last_save_data_info(this: &GameUserGlobalData, _type: i32, index: i32, method_info: OptionalMethod){
    call_original!(this, _type, index, method_info);
    // marks the file as saved so when game over happens the game delete the file
    if GameVariableManager::get_bool("G_Ironman") {
        GameVariableManager::make_entry("G_IronmanSaved", 1);
    }
}
#[unity::hook("App", "MapSequence", "TryRestart")]
pub fn game_over_hook(this: u64, method_info: OptionalMethod) {
    // if ironman mode and save file is saved, delete the save file
    if GameVariableManager::get_bool("G_Ironman"){
        if GameVariableManager::get_bool("G_IronmanSaved") && GameUserGlobalData::get_last_save_data_type() == 6 {
            unsafe {
                let path = game_save_data_get_file_path(6, GameUserGlobalData::get_last_save_data_index(), None);
                if save_data_is_exists(path,None) {
                    save_data_delete(path, None);
                }
            }
        }
    }
    else { call_original!(this, method_info); }
}
#[unity::hook("App", "MainMenuSequence", "ExecuteGameStart")]
pub fn execute_game_start_hook(this: u64, method_info: OptionalMethod){
    call_original!(this, method_info);
    // when new game is created and the new ironman mode is selected
    if GameUserData::get_game_mode() == GameMode::Classic {
        GameVariableManager::make_entry("G_Ironman", 1);
    }
}
#[unity::hook("App", "MapSequence", "Init")]
pub fn map_sequence_init_hook(this: u64, method_info: OptionalMethod) {
    //where the iron man code edits are apply: when map sequence gets initialized
    call_original!(this, method_info);
    ironman_code_edits();
}
#[skyline::from_offset(0x01ec5190)]
pub fn save_data_delete(path: &Il2CppString, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x02281490)]
pub fn game_save_data_get_file_path(_type: i32, index: i32, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::from_offset("App","SaveData", "IsExist")]
pub fn save_data_is_exists(path: &Il2CppString, method_info: OptionalMethod ) -> bool;

#[skyline::hook(offset=0x01fd9ca0)]
pub fn game_mode_bind(this: u64, proc: &mut ProcInst, method_info: OptionalMethod){
    call_original!(this, proc, method_info);
    let config_menu = proc.child.cast_mut::<BasicMenu<BasicMenuItem>>();
    config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("GetName").map(|method| method.method_ptr = ironman_name as _);
    config_menu.full_menu_item_list.items[1].get_class_mut().get_virtual_method_mut("GetHelp").map(|method| method.method_ptr = ironman_help as _);
}


#[skyline::from_offset(0x02285890)]
pub fn game_save_data_write(proc: u64, _type: i32, index: i32, m1: OptionalMethod, method_info: OptionalMethod);

pub extern "C" fn ironman_name(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { "Ironman".into() }
pub extern "C" fn ironman_help(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> &'static Il2CppString { "For the real Fire Emblem purists.\nSave file will be deleted upon a game over.".into() }

pub fn ironman_code_edits(){
    //Code Edits to disable restart/reset/time crystal and forced bookmark if on ironman mode
    if GameVariableManager::get_bool("G_Ironman") {
        // Restart Build Attribute = 4
        Patch::in_text(0x01b72cb0).bytes(&[0x80, 0x00, 0x80, 0xD2]);
        Patch::in_text(0x01b72cb4).bytes(&[0xC0, 0x03, 0x5F, 0xD6]);

        // Reset Build Attribute = 4
        Patch::in_text(0x01b72950).bytes(&[0x80, 0x00, 0x80, 0xD2]);
        Patch::in_text(0x01b72954).bytes(&[0xC0, 0x03, 0x5F, 0xD6]);

        // Rewind Attribute = 4
        Patch::in_text(0x01f52230).bytes(&[0x80, 0x00, 0x80, 0xD2]);
        Patch::in_text(0x01f52234).bytes(&[0xC0, 0x03, 0x5F, 0xD6]);

        // bookmark instead of save for all difficulies
        Patch::in_text(0x01e41118).nop();
        Patch::in_text(0x01e4111c).nop();
        Patch::in_text(0x01e41180).bytes(&[0x08,0x06,0x80, 0x52]);

        Patch::in_text(0x02677308).bytes(&[0x1F, 0x0D, 0x00, 0x71]);
        //Patch::in_text(0x0267730c).nop();

        Patch::in_text(0x02677444).nop();
        Patch::in_text(0x02677448).nop();

        Patch::in_text(0x01e40d7c).nop();
        Patch::in_text(0x01e40d88).nop();

        Patch::in_text(0x01e40f0c).nop();
        Patch::in_text(0x01e40f18).nop();

    }
    // if not store the original code
    else {
        // Restart Build Attribute 
        Patch::in_text(0x01b72cb0).bytes(&[0xfd, 0x7b, 0xbc, 0xa9]);
        Patch::in_text(0x01b72cb4).bytes(&[0xf7, 0x0b, 0x00, 0xf9]);
    
        // Reset Build Attribute
        Patch::in_text(0x01b72950).bytes(&[0xfd, 0x7b, 0xbe, 0xa9]);
        Patch::in_text(0x01b72954).bytes(&[0xf3, 0x0b, 0x00, 0xf9]);
        // Rewind Attribute 
        Patch::in_text(0x01f52230).bytes(&[0xfd, 0x7b, 0xbe, 0xa9]);
        Patch::in_text(0x01f52234).bytes(&[0xf4, 0x4f, 0x01, 0xa9]);

        //Bookmark/Save 
        Patch::in_text(0x01e41118).bytes(&[0x3f, 0x09, 0x00, 0x71]);
        Patch::in_text(0x01e4111c).bytes(&[0x21, 0x02, 0x00, 0x54]);
        Patch::in_text(0x01e41180).bytes(&[0xe8, 0x05, 0x80, 0x52]);
        
        Patch::in_text(0x02677308).bytes(&[0x1f, 0x09, 0x80, 0x71]);
        Patch::in_text(0x0267730c).bytes(&[0x41, 0xff, 0xff, 0x54]);

        Patch::in_text(0x02677444).bytes(&[0x1f, 0x01, 0x00, 0x6a]);
        Patch::in_text(0x02677448).bytes(&[0xc0, 0x07, 0x00, 0x54]);


        Patch::in_text(0x01e40d7c).bytes(&[0x3f, 0x09, 0x00, 0x71]);
        Patch::in_text(0x01e40d88).bytes(&[0xe1, 0x00, 0x00, 0x54]);

        Patch::in_text(0x01e40f0c).bytes(&[0x3f, 0x09, 0x00, 0x71]);
        Patch::in_text(0x01e40f18).bytes(&[0xe1, 0x00, 0x00, 0x54]);
    }

}
#[skyline::main(name = "ironman")]
pub fn main() {
    Patch::in_text(0x01d6563c).bytes(&[0x00,0x00,0x80,0x52]);
    skyline::install_hooks!(game_over_hook, game_mode_bind, set_last_save_data_info, execute_game_start_hook, map_sequence_init_hook);
}
