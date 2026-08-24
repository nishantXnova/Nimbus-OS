//! PS/2 keyboard driver - IRQ1

use spin::Mutex;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use conquer_once::spin::OnceCell;

#[derive(Debug, Clone, Copy)]
pub enum KeyEvent { Char(char), Enter, Backspace, Tab, Escape, Up, Down, Left, Right, F(u8), Unknown }

const QUEUE_SIZE: usize = 128;

pub struct KeyQueue {
    buf: [Option<KeyEvent>; 128],
    head: usize, tail: usize, len: usize,
}
impl KeyQueue {
    const fn new() -> Self { Self { buf: [None; 128], head: 0, tail: 0, len: 0 } }
    fn push(&mut self, ev: KeyEvent) {
        if self.len < QUEUE_SIZE { self.buf[self.tail]=Some(ev); self.tail=(self.tail+1)%QUEUE_SIZE; self.len+=1; }
    }
    fn pop(&mut self) -> Option<KeyEvent> {
        if self.len==0 { return None; }
        let ev=self.buf[self.head].take(); self.head=(self.head+1)%QUEUE_SIZE; self.len-=1; ev
    }
}

lazy_static! { static ref QUEUE: Mutex<KeyQueue> = Mutex::new(KeyQueue::new()); }
static KEYBOARD: OnceCell<Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>>> = OnceCell::uninit();

pub fn init_keyboard() {
    let kb = Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore);
    KEYBOARD.init_once(|| Mutex::new(kb));
    crate::println!("[KBD] PS/2 keyboard initialized (IRQ1)");
}
pub fn on_scancode(scancode: u8) {
    let mut kb = match KEYBOARD.try_get() { Some(m)=>m.lock(), None=>return };
    if let Ok(Some(ev)) = kb.add_byte(scancode) {
        if let Some(key) = kb.process_keyevent(ev) {
            let mapped = match key {
                DecodedKey::Unicode(c) => match c {
                    '\n'=>KeyEvent::Enter, '\t'=>KeyEvent::Tab, '\x08'=>KeyEvent::Backspace, _=>KeyEvent::Char(c)
                },
                DecodedKey::RawKey(k) => match k {
                    pc_keyboard::KeyCode::ArrowUp=>KeyEvent::Up, pc_keyboard::KeyCode::ArrowDown=>KeyEvent::Down,
                    pc_keyboard::KeyCode::ArrowLeft=>KeyEvent::Left, pc_keyboard::KeyCode::ArrowRight=>KeyEvent::Right,
                    pc_keyboard::KeyCode::Escape=>KeyEvent::Escape, pc_keyboard::KeyCode::Backspace=>KeyEvent::Backspace,
                    pc_keyboard::KeyCode::Tab=>KeyEvent::Tab, pc_keyboard::KeyCode::F1=>KeyEvent::F(1),
                    pc_keyboard::KeyCode::F2=>KeyEvent::F(2), pc_keyboard::KeyCode::F3=>KeyEvent::F(3),
                    pc_keyboard::KeyCode::F4=>KeyEvent::F(4), pc_keyboard::KeyCode::F5=>KeyEvent::F(5),
                    _=>KeyEvent::Unknown,
                }
            };
            QUEUE.lock().push(mapped);
        }
    }
}
pub fn pop_key() -> Option<KeyEvent> { QUEUE.lock().pop() }
pub fn has_key() -> bool { !QUEUE.lock().is_empty() }
