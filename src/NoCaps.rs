#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use engage::gamedata::{*, job::*, unit::*};
use skyline::patching::Patch;
use cobapi::{Event, SystemEvent};

// offset for the -20:20 limit to change to -99:99
pub const BUFF_POS: &[usize] = &[
    0x01a1c92c,    0x01a1c95c,     0x01a1c990,    0x01a1c9c4, 
    0x01a1c9f8,    0x01a1ca30,     0x01a1ca68,    0x01a1ca9c, 
    0x01a1cad0,    0x01a1cb08,     0x01a1cb40,    0x01a1d7d0, 
    0x01a1d800,    0x01a1d834,     0x01a1d868,    0x01a1d89c, 
    0x01a1d8d4,    0x01a1d90c,     0x01a1d940,    0x01a1d974, 
    0x01a1d9ac,    0x01a1d9e4,     0x01f76408,    0x01f76438, 
    0x01f7646c,    0x01f764a0,     0x01f764d4,    0x01f7650c, 
    0x01f76544,    0x01f76578,     0x01f765ac,    0x01f765e4, 
    0x01f7661c,    0x01f77b24,     0x01f77b54,    0x01f77b88, 
    0x01f77bbc,    0x01f77bf0,     0x01f77c28,    0x01f77c60, 
    0x01f77c94,    0x01f77cc8,     0x01f77d00,    0x01f77d38, 
];
pub fn set_job_caps(){
    let persons = PersonData::get_list_mut().expect("triabolical is 'None'");
    let jobs = JobData::get_list_mut().expect("triabolical2 is 'None'");
    for x in 0..persons.len() {
        let caps = persons[x].get_limit();
        for i in 0..11 { caps[i] = 0; }
    }
    for x in 0..jobs.len() {
            //Setting job caps to 127 + Base
        let cap = jobs[x].get_limit();
        let base = jobs[x].get_base();
        cap[10] = 99;    // Move cap is 99
        for i in 0..10 { 
            cap = 127 + base[i];
        }
    }
    println!("Job Caps are set to 127 + base and Person Caps are set to 0");
}

extern "C" fn initalize_random_persons(event: &Event<SystemEvent>) {
    if let Event::Args(ev) = event {
        match ev {
            SystemEvent::ProcInstJump {proc, label } => {
                if proc.hashcode == -988690862 && *label == 0 { set_job_caps(); }
                //Reset things
                if proc.hashcode == -339912801 && *label == 2 { set_job_caps(); }
            }
            _ => {},
        }
    } 
    else {  println!("We received a missing event, and we don't care!"); }
}
#[skyline::main(name = "Max Stat Caps")]
pub fn main() {
    cobapi::register_system_event_handler(do_job_caps);
    Patch::in_text(0x01a2a7c0).bytes(&[0xe1,0x0e,0x80,0x12]);
    Patch::in_text(0x01a2a7c4).bytes(&[0x02,0x0f,0x80,0x52]);
    let postive99 = &[0xE2, 0x7C, 0x80, 0x52];
    let negative99 = &[0x61, 0x0C, 0x80, 0x12];
    // Enhance limit change to -99 to 99
    for x in BUFF_POS {
        Patch::in_text(*x).bytes(negative99);
        Patch::in_text(*x+0x4).bytes(postive99);
    }
}
