#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::il2cpp::object::Array;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use skyline::patching::Patch;
use engage::{
    menu::{BasicMenuResult, config::{ConfigBasicMenuItemSwitchMethods, ConfigBasicMenuItem}},
    singleton::SingletonClass,
    gamevariable::*,
    gameuserdata::*,
    force::*,
    proc::{ProcInstFields,Bindable,desc::ProcDesc, ProcInst},
};
// Storing the frame count at the start of player phase
pub static mut FRAME_COUNT: i32 = 0;

#[skyline::from_offset(0x0250c6a0)]
pub fn get_frame_count(method_info: OptionalMethod) -> i32;

//hooking to the phase change effect and getting the frame count at player phase
#[unity::hook("App", "TelopManager", "CreatePhaseChange")]
pub fn phase_change_hook(proc: u64, force: i32, now_turn: i32, limit_turn: i32, method_info: OptionalMethod) {
    call_original!(proc, force, now_turn, limit_turn, method_info);
    if force == 0 {
        unsafe {
            // store the current frame count and restore the default behavior when to automatically end the player's turn (no units can act)
            FRAME_COUNT = get_frame_count(None);
            Patch::in_text(0x0267357c).bytes(&[0x80, 0x01, 0x00, 0xb5]);
            println!("Player Phase Turn {}, Frame Count: {}", now_turn, FRAME_COUNT);
        }
    }
}

// converts the player's setting to the number of frames for the timer
pub fn calculate_allocated_time() -> i32 {
    let toggle =  GameVariableManager::get_number(TIMER_KEY);
    if toggle == 0 {
        return -1;
    }
    else if toggle < 4 {
        return 1800*toggle;
    }
    else if toggle >= 4 {
        let player_force = Force::get(ForceType::Player).unwrap();
        return 450*(toggle-3)*player_force.get_count();
    }
    return -1;

}

//hooking to the function that checks if the player's turn should be ended to check for the amount of time passed
#[unity::hook("App", "MapSequenceHuman", "AutoTurnEnd")]
pub fn auto_turn_end_hook(this: u64, method_info: OptionalMethod) {
    unsafe {
        let frames = get_frame_count(None) - FRAME_COUNT;
        let time = calculate_allocated_time(); 
        // IPS edits to bypass the unit count requirement to end the turn
        if time == -1 {
            // normal behavior if setting is to set to no timer
            Patch::in_text(0x0267357c).bytes(&[0x80, 0x01, 0x00, 0xb5]);
        }
        else {
          // removes the check if any player unit can act
            if frames > time { Patch::in_text(0x0267357c).nop();  }
        }
    }
    call_original!(this, method_info);
}
// Plugin setting 
pub const TIMER_KEY: &str = "G_TIMER";
pub struct TimerMod;
impl ConfigBasicMenuItemSwitchMethods for TimerMod {
    fn init_content(this: &mut ConfigBasicMenuItem){ GameVariableManager::make_entry_norewind(TIMER_KEY, 0); }
    extern "C" fn custom_call(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        let toggle =  GameVariableManager::get_number(TIMER_KEY);
        let result = ConfigBasicMenuItem::change_key_value_i(toggle, 0, 6, 1);
        if toggle != result {
            GameVariableManager::set_number(TIMER_KEY, result);
            Self::set_command_text(this, None);
            Self::set_help_text(this, None);
            this.update_text();
            return BasicMenuResult::se_cursor();
        } else {return BasicMenuResult::new(); }
    }
    extern "C" fn set_help_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let typeC =  GameVariableManager::get_number(TIMER_KEY);
        if typeC == 0 {this.help_text = "Player phase will not end automatically based on time.".into(); }
        else if typeC == 1 {this.help_text = "Player phase will automatically end after one minute.".into(); }
        else if typeC == 2 {this.help_text = "Player phase will automatically end after two minutes.".into(); }
        else if typeC == 3 {this.help_text = "Player phase will automatically end after three minutes.".into(); }
        else if typeC == 4 {this.help_text = "Player phase will automatically end after 15 seconds per unit.".into(); }
        else if typeC == 5 {this.help_text = "Player phase will automatically end after 30 seconds per unit.".into(); }
        else if typeC == 6 {this.help_text = "Player phase will automatically end after 45 seconds per unit.".into(); }
    }
    extern "C" fn set_command_text(this: &mut ConfigBasicMenuItem, _method_info: OptionalMethod){
        let type_C =  GameVariableManager::get_number(TIMER_KEY);
        
        if type_C == 0 { this.command_text = "No Turn Timer".into(); }
        else if type_C == 1 { this.command_text = "1 Minute Timer".into(); }
        else if type_C == 2 { this.command_text = "2 Minute Timer".into(); }
        else if type_C == 3 { this.command_text = "3 Minute Timer".into(); }
        else if type_C == 4 { this.command_text = "15 sec/unit".into(); }
        else if type_C == 5 { this.command_text = "30 sec/unit".into(); }
        else if type_C == 6 { this.command_text = "45 sec/unit".into(); }
    }
}
#[no_mangle]
extern "C" fn timer_create() -> &'static mut ConfigBasicMenuItem { 
    ConfigBasicMenuItem::new_switch::<TimerMod>("Timer")
 } 

#[skyline::main(name = "timer")]
pub fn main() {
    skyline::install_hooks!( auto_turn_end_hook, phase_change_hook);
    cobapi::install_game_setting(timer_create);
}
