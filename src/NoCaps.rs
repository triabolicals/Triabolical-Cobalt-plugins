#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*, unit::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use skyline::patching::Patch;
use cobapi::Event;
use cobapi::SystemEvent;


pub const BUFF_POS: &[usize] = &[
    0x01f7640c, 0x01f7643c, 0x01f76470, 0x01f764a4, 0x01f764d8, 0x01f76510, 0x01f76548, 0x01f7657C, 
    0x01f765b0, 0x01f765e8, 0x01f76620, 0x01f77b28, 0x01f77b58, 0x01f77b8c, 0x01f77bc0, 0x01f77bf4, 
    0x01f77c2c, 0x01f77c64, 0x01f77c98, 0x01f77ccc, 0x01f77d04, 0x01f77d3c, 0x01a1d7d4, 0x01a1d804, 
    0x01a1d838, 0x01a1d86c, 0x01a1d8a0, 0x01a1d8d8, 0x01a1d910, 0x01a1d944, 0x01a1d978, 0x01a1d9b0, 
    0x01a1d9e8 
];

#[unity::from_offset("App", "JobData", "get_Limit")]
pub fn job_get_limit(this: &JobData, method_info: OptionalMethod) -> & mut Capability;

#[unity::from_offset("App", "JobData", "get_Base")]
pub fn job_get_base(this: &JobData, method_info: OptionalMethod) -> & mut Capability;

#[unity::from_offset("App", "JobData", "set_Limit")]
pub fn job_set_limit(this: &JobData, value :&Capability, method_info: OptionalMethod);

pub fn set_job_caps(){
    unsafe {
        let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
        let t_list = &triabolical.list.items;
        let triabolical2 = JobData::get_list_mut().expect("triabolical2 is 'None'");
        let t_list2 = &triabolical2.list.items;
        for x in 0..1522 {
            //Setting personal caps to 0
            let caps = get_limit(t_list[x], None);
            for i in 0..11 { caps.array.m_item[i] = 0; }
        }
        for x in 0..111 {
            //Setting job caps to 127 + Base
            let job = &t_list2[x];
            let cap = job_get_limit(job, None);
            let base = job_get_base(job, None);
            for i in 0..10 { cap.array.m_item[i] = 127 + base.array.m_item[i]; }
            //Move Cap is set to 99
            cap.array.m_item[10] = 99;
            job_set_limit(job, cap, None);
        }
    }
    println!("Job Caps are set to 127 + base and Person Caps are set to 0");
}

extern "C" fn do_job_caps(event: &Event<SystemEvent>) {
    if let Event::Args(ev) = event {
        match ev {
            SystemEvent::LanguageChanged => {
                set_job_caps();
            },
            _ => {}
        }
    } 
    else {  println!("We received a missing event, and we don't care!"); }
}

#[skyline::main(name = "Max Stat Caps")]
pub fn main() {
    cobapi::register_system_event_handler(do_job_caps);
    Patch::in_text(0x01a2a7c0).bytes(&[0xe1,0x0e,0x80,0x12]);
    Patch::in_text(0x01a2a7c4).bytes(&[0x02,0x0f,0x80,0x52]);
    let postive99 = &[0xE2, 0x1F, 0x80, 0x52];
    let negative99 = &[0x61, 0x0C, 0x80, 0x12];
    // Enhance limit change to -99 to 99
    for x in BUFF_POS {
        Patch::in_text(*x-0x4).bytes(negative99);
        Patch::in_text(*x).bytes(postive99);
    }
}
