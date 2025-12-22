use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use core::{pin::Pin, task::{Context, Poll}};
use futures_util::{stream::{Stream, StreamExt}, task::AtomicWaker};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use alloc::vec::Vec;

use crate::{print, println};

/// ==========================
/// Scancode queue + waker
/// ==========================

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");

        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(cx.waker());

        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

/// Called from the keyboard interrupt handler
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if queue.push(scancode).is_err() {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

/// ==========================
/// Keyboard callback system
/// ==========================

/// Callback type
pub type KeyCallback = fn(DecodedKey);

/// Global callback list (heap-backed)
static KEY_CALLBACKS: Mutex<Vec<KeyCallback>> = Mutex::new(Vec::new());

/// Register a new keyboard callback
///
/// Safe to call from anywhere (init code, drivers, tasks)
pub fn register_key_callback(callback: KeyCallback) {
    let mut callbacks = KEY_CALLBACKS.lock();
    callbacks.push(callback);
}

/// Dispatch a key event to all registered callbacks
#[inline]
pub fn dispatch_key_event(key: DecodedKey) {
    let callbacks = KEY_CALLBACKS.lock();

    for callback in callbacks.iter() {
        callback(key);
    }
}

/// ==========================
/// Keyboard task
/// ==========================

/// Inits the keyboard
pub async fn init() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {

                // Fire callbacks
                dispatch_key_event(key);
            }
        }
    }
}

/// Continusly print all keys pressed onto the screen
/// 
/// Inits the keyboard
#[deprecated(note = "use the 'init' function instead and bind a call back to it to print characters")]
pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore,
    );

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                // Default behavior
                match key {
                    DecodedKey::Unicode(c) => print!("{}", c),
                    DecodedKey::RawKey(k) => print!("{:?}", k),
                }

                // Fire callbacks
                dispatch_key_event(key);
            }
        }
    }
}
