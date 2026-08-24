//! Cyberdeck v0.4 — 8 views: Shell | Procs | Timeline | Mem | Chronos | Nexus | Atlas | Synapse
//! Single compositor owns 80x25. Tab cycles, 1-8 jump, context keys per view.

extern crate alloc;
use alloc::{string::{String, ToString}, vec::Vec, format};
use spin::Mutex;
use lazy_static::lazy_static;
use crate::vga_buffer::{Color, BUFFER_WIDTH, BUFFER_HEIGHT};
use crate::scheduler::{SCHEDULER, TaskState, Priority};
use crate::keyboard::{KeyEvent, pop_key};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum View { Shell=0, Processes=1, Timeline=2, Memory=3, Chronos=4, Nexus=5, Atlas=6, Synapse=7 }
impl View {
    fn next(self)->Self{ match self{ Self::Shell=>Self::Processes, Self::Processes=>Self::Timeline, Self::Timeline=>Self::Memory, Self::Memory=>Self::Chronos, Self::Chronos=>Self::Nexus, Self::Nexus=>Self::Atlas, Self::Atlas=>Self::Synapse, Self::Synapse=>Self::Shell } }
    fn from_num(n:u8)->Self{ match n{ 1=>Self::Shell,2=>Self::Processes,3=>Self::Timeline,4=>Self::Memory,5=>Self::Chronos,6=>Self::Nexus,7=>Self::Atlas,8=>Self::Synapse,_=>Self::Shell } }
    fn name(self)->&'static str{ match self{ Self::Shell=>" SHELL ", Self::Processes=>" PROCESSES ", Self::Timeline=>" TIMELINE ", Self::Memory=>" MEMORY ", Self::Chronos=>" CHRONOS ", Self::Nexus=>" NEXUS ", Self::Atlas=>" ATLAS ", Self::Synapse=>" SYNAPSE " } }
}

pub struct Deck {
    view: View,
    tick: u64,
    selected: usize,
    shell_input: String,
    shell_log: Vec<String>,
    mem_scroll: usize,
    chronos_seek: u64, // seek tick for rewind
    chronos_live: bool,
    synapse_input: String,
    atlas_scroll: usize,
}

impl Deck {
    pub fn new()->Self{
        Self{
            view: View::Shell,
            tick: 0,
            selected: 0,
            shell_input: String::new(),
            shell_log: alloc::vec![
                "CHRONO-VECTIS v0.4 — CHRONOS+NEXUS+ATLAS+SYNAPSE".into(),
                "1:Shell 2:Procs 3:Time 4:Mem 5:Chronos 6:Nexus 7:Atlas 8:Synapse TAB cycle".into(),
                "Try: help | ps | chronos | nexus | atlas | synapse".into(),
            ],
            mem_scroll: 0,
            chronos_seek: 0,
            chronos_live: true,
            synapse_input: String::new(),
            atlas_scroll: 0,
        }
    }
    pub fn set_tick(&mut self, t:u64){ self.tick=t; if self.chronos_live { self.chronos_seek=t; } }

    pub fn handle_key(&mut self, k: KeyEvent){
        // global jumps
        match k {
            KeyEvent::Tab => { self.view=self.view.next(); return; },
            KeyEvent::Char(c) if c>='1' && c<='8' => { self.view=View::from_num(c as u8 - b'0'); return; },
            KeyEvent::Char('q') if self.view!=View::Shell => { self.view=View::Shell; return; },
            _=>{}
        }
        match self.view {
            View::Shell=>self.shell_key(k),
            View::Processes=>self.proc_key(k),
            View::Timeline=>self.timeline_key(k),
            View::Memory=>self.mem_key(k),
            View::Chronos=>self.chronos_key(k),
            View::Nexus=>self.nexus_key(k),
            View::Atlas=>self.atlas_key(k),
            View::Synapse=>self.synapse_key(k),
        }
    }

    fn shell_key(&mut self, k: KeyEvent){
        match k{
            KeyEvent::Char(c)=>self.shell_input.push(c),
            KeyEvent::Backspace=>{self.shell_input.pop();},
            KeyEvent::Enter=>{
                let cmd=self.shell_input.clone();
                self.shell_log.push(format!("> {}", cmd));
                self.exec(&cmd);
                self.shell_input.clear();
                if self.shell_log.len()>120{ self.shell_log.drain(0..30); }
            },
            _=>{}
        }
    }
    fn exec(&mut self, cmd:&str){
        let p:Vec<&str>=cmd.split_whitespace().collect();
        match p.get(0).copied().unwrap_or("") {
            ""=>{},
            "help"=>{ self.shell_log.push(" help ps kill <id> pri <id> h|l spawn [name] clear".into());
                      self.shell_log.push(" chronos snap|seek <tick>|live | nexus list|spawn <pkg> | atlas | synapse <pid> <blink|counter|loop>".into()); },
            "ps"=>{ for t in SCHEDULER.tasks.lock().iter(){ self.shell_log.push(format!(" {} '{}' {:?} runs:{}", t.id, t.name, t.state, t.runs.load(core::sync::atomic::Ordering::SeqCst))); } },
            "clear"=>self.shell_log.clear(),
            "kill"=>{ if let Some(id)=p.get(1).and_then(|s| s.parse::<u64>().ok()){ let ok=SCHEDULER.kill(id); crate::chronos::record_kill(id); self.shell_log.push(if ok{format!("killed {}",id)}else{format!("no pid {}",id)});} else{ self.shell_log.push("usage: kill <id>".into()); } },
            "pri"=>{ if p.len()>=3{ if let Ok(id)=p[1].parse::<u64>(){ let pr=match p[2]{"high"=>Priority::High,"low"=>Priority::Low,_=>Priority::Normal}; let ok=SCHEDULER.set_priority(id,pr); self.shell_log.push(if ok{format!("pid {} -> {:?}",id,pr)}else{"no pid".into()}); } } else{ self.shell_log.push("usage: pri <id> high|normal|low".into()); } },
            "spawn"=>{ let n=p.get(1).copied().unwrap_or("task"); let leaked: &'static str=alloc::boxed::Box::leak(n.to_string().into_boxed_str()); let id=crate::scheduler::spawn(leaked, crate::dummy_task as u64, 4096); if let Some(pid)=id{ crate::chronos::record_spawn(pid); } self.shell_log.push(format!("spawned {:?} -> {:?}", n, id)); },
            "chronos"=>{ match p.get(1).copied(){ Some("snap")=>{ crate::chronos::take_snapshot(); self.shell_log.push("snapshot taken".into()); }, Some("live")=>{ self.chronos_live=true; self.shell_log.push("chronos live".into()); }, Some("seek")=>{ if let Some(t)=p.get(2).and_then(|s| s.parse::<u64>().ok()){ self.chronos_seek=t; self.chronos_live=false; self.view=View::Chronos; self.shell_log.push(format!("seek -> {}",t)); } }, _=>{ self.shell_log.push(format!("chronos: {}", crate::chronos::stats_line())); } } },
            "nexus"=>{ match p.get(1).copied(){ Some("list")=>{ for m in crate::wasm::list_pkgs(){ self.shell_log.push(format!(" {} {}B {:?}", m.name, m.size, m.exports)); } }, Some("spawn")=>{ let pkg=p.get(2).copied().unwrap_or("fib.wasm"); let id=crate::wasm::spawn_wasm(pkg); self.shell_log.push(format!("nexus spawn {} -> {:?}", pkg, id)); }, _=>self.shell_log.push("nexus list | nexus spawn <pkg>".into()) } },
            "atlas"=>{ for (id,name,from,to,fill,sent,recv,_) in crate::ipc::list_pipes(){ self.shell_log.push(format!(" pipe{} {} {}->{} fill:{}% sent:{} recv:{}", id, name, from,to,fill,sent,recv)); } },
            "synapse"=>{ if p.len()>=3{ if let Ok(pid)=p[1].parse::<u64>(){ let kind=match p[2]{"blink"=>crate::synapse::PatchKind::BlinkRate,"counter"=>crate::synapse::PatchKind::Counter,"loop"=>crate::synapse::PatchKind::Loop,_=>crate::synapse::PatchKind::Custom}; let ok=crate::synapse::hot_patch(pid,kind); self.shell_log.push(if ok{format!("patched {}",pid)}else{"no pid".into()}); } } else{ self.shell_log.push("synapse <pid> blink|counter|loop|custom".into()); } },
            "snap"=>{ crate::chronos::take_snapshot(); self.shell_log.push("snap taken".into()); },
            other=>self.shell_log.push(format!("unknown: {} (help)", other)),
        }
    }

    fn proc_key(&mut self, k: KeyEvent){
        let n=SCHEDULER.task_count().max(1);
        match k{
            KeyEvent::Up=>self.selected=self.selected.saturating_sub(1),
            KeyEvent::Down=>self.selected=(self.selected+1).min(n-1),
            KeyEvent::Char('k')=>{ let id=SCHEDULER.tasks.lock().get(self.selected).map(|t| t.id).unwrap_or(0); SCHEDULER.kill(id); crate::chronos::record_kill(id); },
            KeyEvent::Char('p')=>{ let id=SCHEDULER.tasks.lock().get(self.selected).map(|t| t.id).unwrap_or(0); let cur=SCHEDULER.tasks.lock().get(self.selected).map(|t| t.priority).unwrap_or(Priority::Normal); let nxt=match cur{Priority::High=>Priority::Normal,Priority::Normal=>Priority::Low,_=>Priority::High}; SCHEDULER.set_priority(id,nxt); },
            KeyEvent::Char('r')=>{ let id=crate::scheduler::spawn("spawned", crate::dummy_task as u64, 4096); if let Some(pid)=id{ crate::chronos::record_spawn(pid); } },
            KeyEvent::Char('e')=>{ // hot-patch selected
                let id=SCHEDULER.tasks.lock().get(self.selected).map(|t| t.id).unwrap_or(0);
                crate::synapse::hot_patch(id, crate::synapse::PatchKind::Counter);
                self.view=View::Synapse;
            },
            KeyEvent::Char('s')=>{ crate::chronos::take_snapshot(); },
            _=>{}
        }
    }
    fn timeline_key(&mut self, k: KeyEvent){
        match k{ KeyEvent::Left=>{ self.chronos_live=false; self.chronos_seek=self.chronos_seek.saturating_sub(10); }, KeyEvent::Right=>{ self.chronos_seek=self.chronos_seek.saturating_add(10); }, KeyEvent::Char('l')=>self.chronos_live=true, _=>{} }
    }
    fn mem_key(&mut self, k: KeyEvent){ match k{ KeyEvent::Up=>self.mem_scroll=self.mem_scroll.saturating_sub(1), KeyEvent::Down=>self.mem_scroll+=1, _=>{} } }
    fn chronos_key(&mut self, k: KeyEvent){
        match k{
            KeyEvent::Left=>{ self.chronos_live=false; self.chronos_seek=self.chronos_seek.saturating_sub(20); },
            KeyEvent::Right=>{ self.chronos_seek=self.chronos_seek.saturating_add(20); if self.chronos_seek>=self.tick{ self.chronos_live=true; self.chronos_seek=self.tick; } },
            KeyEvent::Char('l')=>{ self.chronos_live=true; self.chronos_seek=self.tick; },
            KeyEvent::Char('s')=>crate::chronos::take_snapshot(),
            KeyEvent::Up=>self.selected=self.selected.saturating_sub(1),
            KeyEvent::Down=>self.selected+=1,
            _=>{}
        }
    }
    fn nexus_key(&mut self, k: KeyEvent){
        match k{
            KeyEvent::Up=>self.selected=self.selected.saturating_sub(1),
            KeyEvent::Down=>self.selected=(self.selected+1).min(crate::wasm::list_pkgs().len().saturating_sub(1)),
            KeyEvent::Enter=>{ let pkgs=crate::wasm::list_pkgs(); if let Some(m)=pkgs.get(self.selected){ crate::wasm::spawn_wasm(&m.name); } },
            KeyEvent::Char('r')=>crate::wasm::init_nexus(),
            _=>{}
        }
    }
    fn atlas_key(&mut self, k: KeyEvent){
        match k{
            KeyEvent::Up=>self.atlas_scroll=self.atlas_scroll.saturating_sub(1),
            KeyEvent::Down=>self.atlas_scroll+=1,
            KeyEvent::Char('n')=>{ crate::ipc::pipe_create(0,0,"user-pipe"); },
            KeyEvent::Char('f')=>{ crate::ipc::pipe_send(0, b"ping"); },
            _=>{}
        }
    }
    fn synapse_key(&mut self, k: KeyEvent){
        match k{
            KeyEvent::Char(c) if c!='\n' =>{ self.synapse_input.push(c); crate::synapse::edit_push(c); },
            KeyEvent::Backspace=>{ self.synapse_input.pop(); crate::synapse::edit_pop(); },
            KeyEvent::Enter=>{
                let pid=SCHEDULER.tasks.lock().get(self.selected).map(|t| t.id).unwrap_or(0);
                crate::synapse::edit_apply_to(pid);
                self.synapse_input.clear(); crate::synapse::edit_clear();
            },
            KeyEvent::Up=>self.selected=self.selected.saturating_sub(1),
            KeyEvent::Down=>self.selected=(self.selected+1).min(SCHEDULER.task_count().saturating_sub(1)),
            KeyEvent::Char('1')=>{ let pid=SCHEDULER.tasks.lock().get(self.selected).map(|t| t.id).unwrap_or(0); crate::synapse::hot_patch(pid, crate::synapse::PatchKind::BlinkRate); },
            KeyEvent::Char('2')=>{ let pid=SCHEDULER.tasks.lock().get(self.selected).map(|t| t.id).unwrap_or(0); crate::synapse::hot_patch(pid, crate::synapse::PatchKind::Counter); },
            KeyEvent::Char('3')=>{ let pid=SCHEDULER.tasks.lock().get(self.selected).map(|t| t.id).unwrap_or(0); crate::synapse::hot_patch(pid, crate::synapse::PatchKind::Loop); },
            _=>{}
        }
    }

    pub fn drain_keyboard(&mut self){ while let Some(k)=pop_key(){ self.handle_key(k); } }

    pub fn render(&self){
        use crate::vga_buffer::{WRITER, Color};
        let mut w=WRITER.lock();
        w.clear_screen();
        // top bar
        w.set_position(0,0);
        let bar=self.view.name();
        let tabs=" [1]SH [2]PS [3]TM [4]MM [5]CH [6]NX [7]AT [8]SY TAB  q:home";
        for x in 0..BUFFER_WIDTH{ w.set_position(x,0); w.write_char(' ', Color::White); }
        w.set_position(0,0); w.write_str(" CHRONO-VECTIS v0.4 ", Color::White);
        w.set_position(22,0); w.write_str(bar, Color::Yellow);
        w.set_position(34,0); w.write_str(tabs, Color::LightGray);
        let ticks=crate::scheduler::get_ticks();
        let sw=SCHEDULER.get_switches();
        let status=format!(" T:{} SW:{} ", ticks, sw);
        let sx=BUFFER_WIDTH - status.len() - 1;
        w.set_position(sx,0); w.write_str(&status, Color::Cyan);

        match self.view{
            View::Shell=>self.render_shell(&mut w),
            View::Processes=>self.render_procs(&mut w),
            View::Timeline=>self.render_timeline(&mut w),
            View::Memory=>self.render_memory(&mut w),
            View::Chronos=>self.render_chronos(&mut w),
            View::Nexus=>self.render_nexus(&mut w),
            View::Atlas=>self.render_atlas(&mut w),
            View::Synapse=>self.render_synapse(&mut w),
        }
        // bottom help
        let help=match self.view{
            View::Shell=>" help ps kill pri spawn chronos nexus atlas synapse | Enter",
            View::Processes=>" ↑↓ sel k kill p pri r spawn e patch s snap",
            View::Timeline=>" ←→ scrub l live | pipes flow below",
            View::Memory=>" heap + ramfs ↑↓ scroll",
            View::Chronos=>" ←→ rewind l live s snap | shows events+snapshots",
            View::Nexus=>" ↑↓ pkg Enter spawn | orchard growth",
            View::Atlas=>" ↑↓ scroll n new pipe f flood | visible IPC",
            View::Synapse=>" ↑↓ pid 1 blink 2 count 3 loop Enter custom | hot-patch",
        };
        for x in 0..help.len().min(BUFFER_WIDTH){ w.set_position(x, BUFFER_HEIGHT-1); w.write_char(help.as_bytes()[x] as char, Color::White); }
    }

    fn render_shell(&self, w:&mut crate::vga_buffer::Writer){
        w.set_position(0,1); w.write_str("+-- SHELL ---------------------------------------------------+", Color::DarkGray);
        let start=self.shell_log.len().saturating_sub(17);
        for (i,line) in self.shell_log.iter().skip(start).enumerate(){
            let y=2+i; if y>=BUFFER_HEIGHT-1{break;}
            w.set_position(1,y); let c=if line.len()>78{&line[..78]}else{line}; w.write_str(c, Color::LightGray);
        }
        w.set_position(1,22); w.write_str("> ", Color::Yellow); w.write_str(&self.shell_input, Color::White);
        let cx=3+self.shell_input.len(); if cx<BUFFER_WIDTH{ w.set_position(cx,22); w.write_char(if (self.tick%10)<5{'_'}else{' '}, Color::Yellow); }
    }
    fn render_procs(&self, w:&mut crate::vga_buffer::Writer){
        w.set_position(0,1); w.write_str("+ PID  NAME          STATE  PRIO  RUNS  CPU  STACK +", Color::DarkGray);
        let tasks=SCHEDULER.tasks.lock();
        for (i,t) in tasks.iter().enumerate(){
            let y=2+i; if y>=BUFFER_HEIGHT-2{break;}
            let sel=i==self.selected;
            w.set_position(0,y); w.write_str(if sel{">"}else{" "}, if sel{Color::Yellow}else{Color::DarkGray});
            let st=match t.state{TaskState::Running=>"RUN ",TaskState::Ready=>"RDY ",TaskState::Sleeping(_)=>"SLP ",TaskState::Terminated=>"END ",TaskState::Blocked=>"BLK "};
            let pr=match t.priority{Priority::High=>"HIGH",Priority::Normal=>"NORM",Priority::Low=>"LOW ",Priority::Idle=>"IDLE"};
            let line=format!(" {:3}  {:12}  {} {} {:4} {:4}ms {:4}K", t.id, t.name, st, pr, t.runs.load(core::sync::atomic::Ordering::SeqCst), t.cpu_ms.load(core::sync::atomic::Ordering::SeqCst)%1000, t.stack_layout.size()/1024);
            let c=if line.len()>70{&line[..70]}else{&line}; w.write_str(c, if sel{Color::White}else{Color::LightGray});
            let bar=(t.runs.load(core::sync::atomic::Ordering::SeqCst)%10) as usize;
            w.set_position(68,y); for b in 0..6{ w.write_char(if b<bar{'#'}else{'.'}, if t.state==TaskState::Running{Color::Green}else{Color::DarkGray}); }
        }
        w.set_position(42,14); w.write_str("+-- INSPECTOR: e patch s snap ---------+", Color::DarkGray);
        if let Some(t)=tasks.get(self.selected){
            w.set_position(42,15); w.write_str(&format!(" ID:{} '{}' ", t.id, t.name), Color::Cyan);
            w.set_position(42,16); w.write_str(&format!(" RIP:{:#x} RSP:{:#x}", t.context.rip, t.context.rsp), Color::LightGray);
            w.set_position(42,17); w.write_str(&format!(" patches:{} chronos:{}", crate::synapse::patch_count(), crate::chronos::snapshot_count()), Color::LightGray);
        }
    }
    fn render_timeline(&self, w:&mut crate::vga_buffer::Writer){
        w.set_position(0,1); w.write_str("+-- CPU TIMELINE + ATLAS FLOW (100Hz) -----------------------+", Color::DarkGray);
        let tasks=SCHEDULER.tasks.lock();
        for (i,t) in tasks.iter().enumerate(){
            let y=3+i*2; if y>=19{break;}
            w.set_position(1,y); w.write_str(&format!("{:8}", t.name), Color::Yellow);
            w.set_position(10,y); w.write_str("[", Color::DarkGray);
            let total=44; let fill=(t.runs.load(core::sync::atomic::Ordering::SeqCst) as usize*3)%total; let is_run=t.state==TaskState::Running;
            for x in 0..total{ let ch=if x<fill{if is_run{'#'}else{'='}}else{'.'}; let col=if x<fill{if is_run{Color::Green}else{Color::Cyan}}else{Color::DarkGray}; w.write_char(ch,col); }
            w.write_str("]", Color::DarkGray); w.set_position(60,y); w.write_str(&format!("{:4}ms", t.cpu_ms.load(core::sync::atomic::Ordering::SeqCst)%10000), Color::LightGray);
            // atlas spark under
            w.set_position(10,y+1); for s in 0..44{ let idx=((t.id as usize+s+self.tick as usize)%5); let ch=[' ','.','-','=','#'][idx]; w.write_char(ch, Color::DarkGray); }
        }
        // show pipes flow line
        let pipes=crate::ipc::list_pipes();
        w.set_position(1,20); w.write_str("PIPES:", Color::Yellow);
        for (i,(_,name,_,_,fill,_,_,pos)) in pipes.iter().enumerate(){
            let y=21; let x=1+i*14; if x+12>=BUFFER_WIDTH{break;}
            w.set_position(x,y); w.write_str(&format!("{}:", &name[..name.len().min(6)]), Color::Cyan);
            w.set_position(x,y+1); w.write_str("[",Color::DarkGray);
            let dots=(*pos as usize)%10; for b in 0..8{ w.write_char(if b==dots{'o'}else if b*12 < *fill{'='}else{'.'}, if b==dots{Color::Yellow}else{Color::Green}); } w.write_str("]",Color::DarkGray);
        }
        if self.chronos_live{ w.set_position(1,23); w.write_str(" live ←→ to rewind, l live", Color::DarkGray); } else{ w.set_position(1,23); w.write_str(&format!(" REWIND @{}  l live", self.chronos_seek), Color::Yellow); }
    }
    fn render_memory(&self, w:&mut crate::vga_buffer::Writer){
        w.set_position(0,1); w.write_str("+-- MEMORY + HEAP + RAMFS ----------------------------------+", Color::DarkGray);
        let (used,total)=crate::heap::heap_stats();
        let pct=if total>0{(used*50)/total}else{0};
        w.set_position(1,3); w.write_str("HEAP ",Color::Cyan); w.set_position(6,3); w.write_str("[",Color::DarkGray);
        for i in 0..50{ w.write_char(if i<pct{'#'}else{'.'}, if i<pct{Color::Green}else{Color::DarkGray}); } w.write_str("]",Color::DarkGray);
        w.set_position(58,3); w.write_str(&format!("{}/{} KiB", used/1024, total/1024), Color::LightGray);
        w.set_position(1,5); w.write_str("RAMFS:",Color::Yellow);
        let files=crate::ramfs::list();
        for (i,(name,size)) in files.iter().skip(self.mem_scroll).enumerate(){
            let y=6+i; if y>=20{break;}
            w.set_position(2,y); w.write_str(if name.contains("/pkg"){"#"}else if name.contains(".log"){"*"}else{"-"}, Color::Cyan);
            w.set_position(4,y); w.write_str(name, Color::White); w.set_position(32,y); w.write_str(&format!("{} B", size), Color::DarkGray);
        }
        w.set_position(1,21); w.write_str(&format!(" snaps:{} events:{} pipes:{} pkgs:{}", crate::chronos::snapshot_count(), crate::chronos::event_count(), crate::ipc::list_pipes().len(), crate::wasm::list_pkgs().len()), Color::DarkGray);
    }
    fn render_chronos(&self, w:&mut crate::vga_buffer::Writer){
        w.set_position(0,1); w.write_str("+-- CHRONOS — TIME TRAVEL (←→ seek, l live, s snap) ---------+", Color::DarkGray);
        let live=self.chronos_live;
        w.set_position(1,3); w.write_str(&format!("MODE: {}  SEEK:{}  LIVE:{}", if live{"LIVE "}else{"REWIND"}, self.chronos_seek, self.tick), if live{Color::Green}else{Color::Yellow});
        w.set_position(1,4); w.write_str(&format!("{}  snaps:{} events:{}", crate::chronos::stats_line(), crate::chronos::snapshot_count(), crate::chronos::event_count()), Color::LightGray);
        // seek bar 50 chars
        w.set_position(1,5); w.write_str("[",Color::DarkGray);
        let total:usize=50; let pos:usize=if self.tick>0{ ((self.chronos_seek as usize)*total/(self.tick as usize)) } else {0};
        for i in 0..total{ w.write_char(if i==pos{'^'}else if i<pos{'#'}else{'.'}, if i==pos{Color::Yellow}else{Color::Cyan}); }
        w.write_str("]",Color::DarkGray);
        // recent events
        w.set_position(1,7); w.write_str("RECENT EVENTS:",Color::Yellow);
        let evs=crate::chronos::timeline_events(self.chronos_seek.saturating_sub(30), self.chronos_seek+5);
        for (i,e) in evs.iter().rev().take(8).enumerate(){
            let y=8+i; if y>=16{break;}
            let kind=match e.kind{0=>"tick",1=>"sched",2=>"kill",3=>"spawn",4=>"key",5=>"pipe",6=>"snap",_=>"?"};
            w.set_position(2,y); w.write_str(&format!(" t{} {:5} id:{}", e.tick, kind, e.id), Color::LightGray);
        }
        if !live{
            w.set_position(1,16); w.write_str("REWIND VIEW — tasks at seek:",Color::Yellow);
            let tasks=crate::chronos::tasks_at(self.chronos_seek);
            for (i,(id,name,st)) in tasks.iter().enumerate(){
                let y=17+i; if y>=22{break;}
                w.set_position(2,y); w.write_str(&format!(" {} {} {:?}", id, name, st), Color::DarkGray);
            }
        } else {
            w.set_position(1,16); w.write_str("LIVE — press ← to rewind",Color::DarkGray);
        }
    }
    fn render_nexus(&self, w:&mut crate::vga_buffer::Writer){
        w.set_position(0,1); w.write_str("+-- NEXUS ORCHARD — WASM PACKAGES (Enter spawn) -----------+", Color::DarkGray);
        let pkgs=crate::wasm::list_pkgs();
        let (planted,harvested,gt)=crate::wasm::orchard_stats();
        w.set_position(1,3); w.write_str(&format!("Planted:{} Harvested:{} Growth:{}", planted, harvested, gt), Color::LightGray);
        w.set_position(1,4); w.write_str(" PKG            SIZE  EXPORTS         TREE",Color::DarkGray);
        for (i,m) in pkgs.iter().enumerate(){
            let y=5+i; if y>=18{break;}
            let sel=i==self.selected;
            w.set_position(0,y); w.write_str(if sel{">"}else{" "}, if sel{Color::Yellow}else{Color::DarkGray});
            let exports=if m.exports.is_empty(){"-".into()}else{m.exports.join(",")};
            let line=format!(" {:12} {:4}B {:12} ", m.name, m.size, &exports[..exports.len().min(12)]);
            w.write_str(&line, if sel{Color::White}else{Color::LightGray});
            // tree growth bar
            let grow=((self.tick as usize + i*7)%10);
            w.set_position(42,y);
            let tree=['.',':','|','Y','*'];
            for b in 0..10{ w.write_char(if b<grow{tree[4]}else{tree[0]}, if sel{Color::Green}else{Color::DarkGray}); }
            w.write_str(if m.valid{" OK"}else{" BAD"}, if m.valid{Color::Green}else{Color::Red});
        }
        w.set_position(1,19); w.write_str(" Enter: spawn wasm task | ramfs:/pkg/*.wasm",Color::DarkGray);
        w.set_position(1,20); w.write_str(" Shell: nexus list | nexus spawn fib.wasm",Color::DarkGray);
    }
    fn render_atlas(&self, w:&mut crate::vga_buffer::Writer){
        w.set_position(0,1); w.write_str("+-- ATLAS — VISIBLE PIPES (n new, f flood) ----------------+", Color::DarkGray);
        w.set_position(1,3); w.write_str(" ID  NAME          FROM->TO  FILL  SENT RECV FLOW",Color::DarkGray);
        let pipes=crate::ipc::list_pipes();
        for (i,(id,name,from,to,fill,sent,recv,flow)) in pipes.iter().skip(self.atlas_scroll).enumerate(){
            let y=4+i; if y>=16{break;}
            w.set_position(1,y);
            let line=format!(" {:2}  {:12} {:2}->{:2}  {:3}% {:4} {:4} ", id, &name[..name.len().min(12)], from,to,fill,sent,recv);
            w.write_str(&line, Color::LightGray);
            w.set_position(52,y); w.write_str("[",Color::DarkGray);
            for b in 0..10{ w.write_char(if b==(*flow as usize)%10{'o'}else if b*10 < *fill{'#'}else{'.'}, if b==(*flow as usize)%10{Color::Yellow}else{Color::Cyan}); }
            w.write_str("]",Color::DarkGray);
        }
        if pipes.is_empty(){ w.set_position(2,6); w.write_str("(no pipes — press n)",Color::DarkGray); }
        w.set_position(1,17); w.write_str(&format!(" {} pipes | flow anim @{}Hz", pipes.len(), 100), Color::DarkGray);
        w.set_position(1,18); w.write_str(" Shell: atlas | ipc flood via wasm tasks",Color::DarkGray);
        // flow legend
        w.set_position(1,19); w.write_str(" Visual: o = byte in flight, # = buffered",Color::DarkGray);
    }
    fn render_synapse(&self, w:&mut crate::vga_buffer::Writer){
        w.set_position(0,1); w.write_str("+-- SYNAPSE — LIVE HOT-PATCH LAB (no reboot) ---------------+", Color::DarkGray);
        w.set_position(1,3); w.write_str("Select PID ↑↓, then 1:blink 2:counter 3:loop or type +Enter",Color::LightGray);
        let tasks=SCHEDULER.tasks.lock();
        for (i,t) in tasks.iter().enumerate(){
            let y=5+i; if y>=14{break;}
            let sel=i==self.selected;
            w.set_position(0,y); w.write_str(if sel{">"}else{" "}, if sel{Color::Yellow}else{Color::DarkGray});
            w.write_str(&format!(" {:3} {:12} RIP:{:#x}", t.id, t.name, t.context.rip), if sel{Color::White}else{Color::LightGray});
            if t.context.rip == crate::synapse::patch_blink as *const () as u64 { w.set_position(38,y); w.write_str(" [blink]",Color::Cyan); }
            if t.context.rip == crate::synapse::patch_counter as *const () as u64 { w.set_position(38,y); w.write_str(" [counter]",Color::Green); }
        }
        w.set_position(1,14); w.write_str(&format!("EDIT: {}", self.synapse_input), Color::Yellow);
        w.set_position(1,15); w.write_str(&format!("BUF: '{}'  (type blink/counter/wasm +Enter)", crate::synapse::edit_buf()), Color::DarkGray);
        let patches=crate::synapse::list_patches();
        w.set_position(1,16); w.write_str("PATCH LOG:",Color::Yellow);
        for (i,pa) in patches.iter().rev().take(4).enumerate(){
            let y=17+i; if y>=22{break;}
            w.set_position(2,y); w.write_str(&format!(" t{} pid{} {}", pa.tick, pa.pid, pa.desc), Color::LightGray);
        }
        w.set_position(1,22); w.write_str(" Shell: synapse <pid> blink|counter|loop",Color::DarkGray);
    }
}

lazy_static!{ pub static ref DECK: Mutex<Deck> = Mutex::new(Deck::new()); }
pub fn deck_tick(t:u64){ DECK.lock().set_tick(t); }
pub fn deck_handle_keys(){ DECK.lock().drain_keyboard(); }
pub fn deck_render(){ DECK.lock().render(); }
