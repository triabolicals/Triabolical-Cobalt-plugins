#![feature(lazy_cell, ptr_sub_ptr)]
use skyline::patching::Patch;
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, unit::*};

#[unity::from_offset("App","Unit", "get_Hp")]
pub fn unit_get_Hp(this: &Unit, method_info: OptionalMethod) -> i32;
#[unity::from_offset("App","Unit", "AddHp")]
pub fn unit_add_Hp(this: &Unit, value:i32, method_info: OptionalMethod);

#[skyline::from_offset(0x01f723b0)]
pub fn capabilityBase_sbyte_set(this: u64, i: i32, v:i32, method_info: OptionalMethod);

#[skyline::from_offset(0x02de7070)]
pub fn capabilityBase_sbyte_get(this: u64, i: i32, method_info: OptionalMethod) -> u8;

#[unity::from_offset("App", "Unit", "GetCapability")]
pub fn unit_get_capability(this: &Unit, type_: i32, calcEnhance: bool, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "JobData", "get_Base")]
pub fn jobdata_get_base(this: &JobData,  method_info: OptionalMethod) -> u64;

#[skyline::hook(offset=0x01a0e1e0)]
pub fn unit_set_hp(this: &mut Unit, value: i32, method_info: OptionalMethod){
    unsafe {
        let base_ClassHP = capabilityBase_sbyte_get(jobdata_get_base(this.m_Job, None), 0, None) as i32;

        let HP_Enhance = unit_get_capability(this, 0, true, None) - unit_get_capability(this, 0, false, None);
        let mut differenceHP: i32 =  value - base_ClassHP as i32 - HP_Enhance;


        if value < HP_Enhance { differenceHP = differenceHP + HP_Enhance - value + 1; }
        if differenceHP != 0 {
        capabilityBase_sbyte_set(this.m_BaseCapability, 0, differenceHP, method_info);
        call_original!(this, value, method_info);
        }
    }

}

#[skyline::main(name = "libpermHP")]
pub fn main() {

    println!("triabolical permenant HP is loaded");
    skyline::install_hooks!(unit_set_hp);
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();
        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };
        let err_msg = format!(
            "triabolical Permenant HP plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );
        skyline::error::show_error(
            3,
            "da triabolical plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
}

