#![feature(asm)]
#![feature(naked_functions)]
#![feature(map_first_last)]
#![feature(negative_impls)]
#![feature(global_asm)]

global_asm!(include_str!("uthread_switch.s"));

mod utils;

use lazy_static::lazy_static;
use std::cell::RefCell;
use std::mem::{self, align_of, size_of};
use std::sync::{Mutex, Once};

use crate::utils::align_down;

const DEFAULT_STACK_SIZE: usize = 4 * utils::KILOBYTE;

#[derive(Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct CalleeSaved {
    rsp: usize,
    r15: usize,
    r14: usize,
    r13: usize,
    r12: usize,
    rbx: usize,
    rbp: usize,
}

extern "C" {
    fn uthread_switch(prev: *const CalleeSaved, next: *const CalleeSaved);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Available,
    Running,
    Ready,
}

#[derive(Debug)]
struct Thread {
    id: usize,
    state: State,
    stack: Vec<u8>,
    ctx: CalleeSaved,
}

impl Thread {
    pub fn new(id: usize) -> Self {
        Thread {
            id: id,
            stack: vec![0; DEFAULT_STACK_SIZE],
            ctx: CalleeSaved::default(),
            state: State::Available,
        }
    }
}

#[derive(Debug)]
pub struct Runtime {
    workers: Vec<Thread>,
    current: usize,
}
impl !Sync for Runtime {}
impl !Send for Runtime {}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            workers: vec![Thread {
                id: 0,
                stack: vec![0; 0],
                ctx: CalleeSaved::default(),
                state: State::Running,
            }],
            current: 0,
        }
    }

    fn schedule(&mut self) {
        let prev = self.current;
        let next = self
            .workers
            .iter()
            .position(|thread| thread.state == State::Ready);

        println!("self.workers = {:?} next: {:?}", self.workers.len(), next);
        if let Some(next) = next {
            println!("switching from {} to {}", prev, next);
            self.current = next;
            self.workers[next].state = State::Running;

            let prev_ctx = &self.workers[prev].ctx;
            let next_ctx = &self.workers[next].ctx;
            unsafe {
                println!("switching from {:?} to {:?}", prev_ctx, next_ctx);
                uthread_switch(prev_ctx, next_ctx);
                println!("switched from {} to {}", prev, next);
                println!("switched from {:?} to {:?}", prev_ctx, next_ctx);
            }
        } else {
            self.current = 0;
        }
    }

    pub extern "C" fn t_return(&mut self) {
        assert!(self.current != 0);

        let current = self.current;
        self.workers[current].state = State::Available;
        self.schedule();
    }

    pub extern "C" fn yield_now(&mut self) {
        let current = self.current;
        self.workers[current].state = State::Ready;
        println!("yield {}", current);
        self.schedule();
    }

    pub extern "C" fn spawn(&mut self, f: extern "C" fn()) {
        match self
            .workers
            .iter_mut()
            .find(|thread| thread.state == State::Available)
        {
            None => {
                let available_thread = Thread::new(self.workers.len());
                println!("allocated thread {}", available_thread.id);
                self.workers.insert(available_thread.id, available_thread);
                return self.spawn(f);
            }
            Some(available_thread) => {
                unsafe {
                    available_thread.ctx.rsp = align_down(
                        available_thread.stack.as_ptr().add(DEFAULT_STACK_SIZE) as usize,
                        16,
                    );
                    available_thread.ctx.rsp -= 16;
                    (available_thread.ctx.rsp as *mut usize).write_volatile(f as usize);

                    println!("spawned thread {}", available_thread.ctx.rsp);
                }

                available_thread.state = State::Ready;
                println!("spawned thread {}", available_thread.id);
            }
        }
    }
}

thread_local! {
    static START: Once = Once::new();
    pub static RUNTIME: usize = Box::into_raw(Box::new(Runtime::new())) as usize
}
fn retrive_thread_local_rt<'a>() -> &'a mut Runtime {
    let mut addr = 0;
    RUNTIME.with(|rt_addr| addr = *rt_addr);

    unsafe { &mut *(addr as *mut Runtime) }
}

#[no_mangle]
pub extern "C" fn spawn(f: extern "C" fn()) {
    retrive_thread_local_rt().spawn(f);
}

#[no_mangle]
pub extern "C" fn yield_now() {
    retrive_thread_local_rt().yield_now();
}

#[no_mangle]
pub extern "C" fn t_return() {
    retrive_thread_local_rt().t_return()
}

#[no_mangle]
pub extern "C" fn schedule() {
    START.with(|start| {
        start.call_once(|| {
            retrive_thread_local_rt().schedule();
        })
    })
}
