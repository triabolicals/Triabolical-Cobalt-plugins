#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*, unit::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use skyline::patching::Patch;
use cobapi::Event;
use cobapi::SystemEvent;

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
        for x in 0..1500 {
            let caps = get_limit(t_list[x], None);
            for i in 0..11 { caps.array.m_item[i] = 0; }
        }
        for x in 0..111 {
            let job = &t_list2[x];
            let cap = job_get_limit(job, None);
            let base = job_get_base(job, None);
            for i in 0..10 { cap.array.m_item[i] = 127 + base.array.m_item[i]; }
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

#[skyline::main(name = "smash")]
pub fn main() {
    cobapi::register_system_event_handler(do_job_caps);
    Patch::in_text(0x01a2a7c0).bytes(&[0xe1,0x0e,0x80,0x12]);
    Patch::in_text(0x01a2a7c4).bytes(&[0x02,0x0f,0x80,0x52]);
    let postive99 = &[0xE2, 0x1F, 0x80, 0x52];
    let negative99 = &[0x61, 0x0C, 0x80, 0x12];

    Patch::in_text(0x01a1d7d0).bytes(negative99);
    Patch::in_text(0x01a1d800).bytes(negative99);
    Patch::in_text(0x01a1d834).bytes(negative99);
    Patch::in_text(0x01a1d868).bytes(negative99);
    Patch::in_text(0x01a1d89c).bytes(negative99);
    Patch::in_text(0x01a1d8d4).bytes(negative99);
    Patch::in_text(0x01a1d90c).bytes(negative99);
    Patch::in_text(0x01a1d940).bytes(negative99);
    Patch::in_text(0x01a1d974).bytes(negative99);
    Patch::in_text(0x01a1d9ac).bytes(negative99);
    Patch::in_text(0x01a1d9e4).bytes(negative99);

//Commit1st
Patch::in_text(0x01f7640c).bytes(postive99);
Patch::in_text(0x01f7643c).bytes(postive99);
Patch::in_text(0x01f76470).bytes(postive99);
Patch::in_text(0x01f764a4).bytes(postive99);
Patch::in_text(0x01f764d8).bytes(postive99);
Patch::in_text(0x01f76510).bytes(postive99);
Patch::in_text(0x01f76548).bytes(postive99);
Patch::in_text(0x01f7657C).bytes(postive99);
Patch::in_text(0x01f765b0).bytes(postive99);
Patch::in_text(0x01f765e8).bytes(postive99);
Patch::in_text(0x01f76620).bytes(postive99);
Patch::in_text(0x01f77b28).bytes(postive99);
//Commit2nd
Patch::in_text(0x01f77b58).bytes(postive99);
Patch::in_text(0x01f77b8c).bytes(postive99);
Patch::in_text(0x01f77bc0).bytes(postive99);
Patch::in_text(0x01f77bf4).bytes(postive99);
Patch::in_text(0x01f77c2c).bytes(postive99);
Patch::in_text(0x01f77c64).bytes(postive99);
Patch::in_text(0x01f77c98).bytes(postive99);
Patch::in_text(0x01f77ccc).bytes(postive99);
Patch::in_text(0x01f77d04).bytes(postive99);
Patch::in_text(0x01f77d3c).bytes(postive99);

Patch::in_text(0x01a1d7d4).bytes(postive99);
Patch::in_text(0x01a1d804).bytes(postive99);
Patch::in_text(0x01a1d838).bytes(postive99);
Patch::in_text(0x01a1d86c).bytes(postive99);
Patch::in_text(0x01a1d8a0).bytes(postive99);
Patch::in_text(0x01a1d8d8).bytes(postive99);
Patch::in_text(0x01a1d910).bytes(postive99);
Patch::in_text(0x01a1d944).bytes(postive99);
Patch::in_text(0x01a1d978).bytes(postive99);
Patch::in_text(0x01a1d9b0).bytes(postive99);
Patch::in_text(0x01a1d9e8).bytes(postive99);
}
