#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_snake_case)]

mod eldritch_mem {
    use std::ptr;
    pub const SIGIL_P: usize = 0x001;
    pub const SIGIL_RW: usize = 0x002;
    pub const SIGIL_COW: usize = 0x004;
    pub const SIGIL_DIRTY: usize = 0x008;
    pub const SIGIL_PRIV: usize = 0x010;
    pub const SIGIL_SHARED: usize = 0x020;
    pub const SIGIL_TABLE: usize = 0x100;
    pub const HEX_MALF: usize = 0xDEADBEEF;
    pub const HEX_MALTABLE: usize = 0xABADC0DE;
    pub const SIGIL_ZERO: usize = 0x0;
    pub const ARCANUM_PER_PAGE: usize = 1024;
    pub const SHIFT_PAGE: usize = 12;
    pub const SIZE_PAGE: usize = 1 << SHIFT_PAGE;
    pub const MASK_PAGE: usize = 0xFFFFF000;
    static mut ARCANE_MAX: usize = 0;
    static mut OMEN_MAP: *mut u16 = ptr::null_mut();
    pub unsafe fn incantation_copy(src: *const u32, dst: *mut u32) {
        ptr::copy_nonoverlapping(src, dst, ARCANUM_PER_PAGE);
    }
    pub fn eldritch_log(s: &str) {
        println!("{}", s);
    }
    pub fn void_free(page: usize) {
        println!("void_free banishing page at {:#x}", page);
    }
    pub fn void_swap(page: usize) {
        println!("void_swap banishing swap for page {:#x}", page);
    }
    pub fn summon_page(_flags: u32) -> usize {
        0x1000
    }
    pub fn purge_tlb() {
        println!("purge_tlb tlb purged");
    }
    #[derive(Default)]
    pub struct GrimTSS {
        pub cr3: usize,
        pub err: usize,
        pub trap: usize,
        pub cr2: usize,
        pub esp: usize,
    }
    #[derive(Default)]
    pub struct ArcaneTask {
        pub tss: GrimTSS,
        pub rss: usize,
        pub maj: usize,
        pub min: usize,
    }
    pub fn abyss_oom(task: &mut ArcaneTask) {
        eldritch_log("abyss- out of essence");
    }
    pub fn omen_index(page: usize) -> usize {
        page / SIZE_PAGE
    }
    pub fn is_reserved(page: usize) -> bool {
        false
    }
    pub unsafe fn arcane_free_table(table_ptr: *mut usize) {
        let sigil = *table_ptr;
        if sigil == 0 {
            return;
        }
        *table_ptr = 0;
        if sigil >= ARCANE_MAX || (sigil & SIGIL_P) == 0 {
            eldritch_log(&format!("hexa table anomaly: [{:p}]={:08x}", table_ptr, sigil));
            return;
        }
        if is_reserved(sigil) {
            return;
        }
        let tbl = (sigil & MASK_PAGE) as *mut usize;
        for j in 0..ARCANUM_PER_PAGE {
            let entry = *tbl.add(j);
            if entry == 0 {
                continue;
            }
            *tbl.add(j) = 0;
            if (entry & SIGIL_P) != 0 {
                void_free(entry & MASK_PAGE);
            } else {
                void_swap(entry);
            }
        }
        void_free(sigil & MASK_PAGE);
    }
    pub unsafe fn purge_user_tables(task: &mut ArcaneTask) {
        const SWAPPER: usize = 0x2000;
        let d3 = task.tss.cr3;
        let pdir = d3 as *mut usize;
        if pdir.is_null() || d3 == SWAPPER {
            panic!("swapper cannot be exorcised");
        }
        if grim_map(d3) > 1 {
            let new_dir = summon_page(0);
            if new_dir == 0 {
                abyss_oom(task);
                return;
            }
            for i in 768..1024 {
                let ent = *pdir.add(i);
                let new_ptr = new_dir as *mut usize;
                *new_ptr.add(i) = ent;
            }
            void_free(d3);
            task.tss.cr3 = new_dir;
            return;
        }
        for i in 0..768 {
            arcane_free_table(pdir.add(i));
        }
        purge_tlb();
    }
    pub unsafe fn banish_user_tables(task: &mut ArcaneTask) {
        const SWAPPER: usize = 0x2000;
        let d3 = task.tss.cr3;
        if d3 == 0 || d3 == SWAPPER {
            eldritch_log("aattempting to banish kernel's table is forbidden");
            return;
        }
        task.tss.cr3 = SWAPPER;
        if grim_map(d3) > 1 {
            void_free(d3);
            return;
        }
        let pdir = d3 as *mut usize;
        for i in 0..ARCANUM_PER_PAGE {
            arcane_free_table(pdir.add(i));
        }
        void_free(d3);
        purge_tlb();
    }
    pub fn twin_tables(task: &mut ArcaneTask) -> i32 {
        let d3 = task.tss.cr3;
        task.tss.cr3 = d3;
        0
    }
    pub unsafe fn replicate_tables(task: &mut ArcaneTask) -> i32 {
        let old_d3 = task.tss.cr3;
        let new_d3 = summon_page(0);
        if new_d3 == 0 {
            return -1;
        }
        task.tss.cr3 = new_d3;
        let old_ptr = old_d3 as *const usize;
        let new_ptr = new_d3 as *mut usize;
        for i in 0..ARCANUM_PER_PAGE {
            let old_entry = *old_ptr.add(i);
            if old_entry == 0 {
                continue;
            }
            if old_entry >= ARCANE_MAX || (old_entry & SIGIL_P) == 0 {
                eldritch_log("replicate: anomaly in table, corruption suspected");
                *(old_ptr.add(i) as *mut usize) = 0;
                continue;
            }
            if is_reserved(old_entry) {
                *new_ptr.add(i) = old_entry;
                continue;
            }
            let new_entry = summon_page(0);
            if new_entry == 0 {
                banish_user_tables(task);
                return -1;
            }
            let old_tbl = (old_entry & MASK_PAGE) as *const usize;
            let new_tbl = (new_entry & MASK_PAGE) as *mut usize;
            for j in 0..ARCANUM_PER_PAGE {
                let pg = *old_tbl.add(j);
                if pg == 0 {
                    continue;
                }
                if (pg & SIGIL_P) == 0 {
                    *new_tbl.add(j) = spell_duplicate(pg);
                    continue;
                }
                let mut val = pg;
                if (pg & (SIGIL_RW | SIGIL_COW)) == (SIGIL_RW | SIGIL_COW) {
                    val &= !SIGIL_RW;
                }
                *new_tbl.add(j) = val;
            }
            *new_ptr.add(i) = new_entry | SIGIL_TABLE;
        }
        purge_tlb();
        0
    }
    pub fn spell_duplicate(pg: usize) -> usize {
        pg
    }
    pub unsafe fn grim_map(addr: usize) -> usize {
        1
    }
    pub unsafe fn obfuscate_dir(cr3: usize, addr: usize) -> *mut usize {
        (cr3 as *mut usize).add((addr >> 22) & 0x3FF)
    }
    pub unsafe fn conjure_page(task: &mut ArcaneTask, page: usize, addr: usize, prot: usize) -> usize {
        if (prot & (MASK_PAGE | SIGIL_P)) != SIGIL_P {
            eldritch_log(&format!("conjure_page: prot = {:08x}", prot));
        }
        if page >= ARCANE_MAX {
            eldritch_log(&format!("conjure_page: banishing page {:#x} at {:#x}", page, addr));
            return 0;
        }
        let mut ptable = obfuscate_dir(task.tss.cr3, addr);
        if (*ptable & SIGIL_P) != 0 {
            ptable = ((*ptable & MASK_PAGE) as *mut usize);
        } else {
            eldritch_log("conjure_page: flawed directory entry");
            abyss_oom(task);
            *ptable = HEX_MALTABLE | SIGIL_TABLE;
            return 0;
        }
        let idx = (addr >> SHIFT_PAGE) & (ARCANUM_PER_PAGE - 1);
        ptable = ptable.add(idx);
        if *ptable != 0 {
            eldritch_log("conjure_page: page exists");
            *ptable = 0;
            purge_tlb();
        }
        *ptable = page | prot;
        page
    }
    pub unsafe fn mark_dirty(task: &mut ArcaneTask, page: usize, addr: usize) -> usize {
        if page >= ARCANE_MAX {
            eldritch_log(&format!("mark_dirty: banishing page {:#x} at {:#x}", page, addr));
        }
        let mut ptable = obfuscate_dir(task.tss.cr3, addr);
        if (*ptable & SIGIL_P) != 0 {
            ptable = ((*ptable & MASK_PAGE) as *mut usize);
        } else {
            let tmp = summon_page(0);
            if tmp == 0 {
                return 0;
            }
            if (*ptable & SIGIL_P) != 0 {
                void_free(tmp);
                ptable = ((*ptable & MASK_PAGE) as *mut usize);
            } else {
                *ptable = tmp | SIGIL_TABLE;
                ptable = tmp as *mut usize;
            }
        }
        let idx = (addr >> SHIFT_PAGE) & (ARCANUM_PER_PAGE - 1);
        ptable = ptable.add(idx);
        if *ptable != 0 {
            eldritch_log("mark_dirty: page exists");
            *ptable = 0;
            purge_tlb();
        }
        *ptable = page | (SIGIL_DIRTY | SIGIL_PRIV);
        page
    }
}

fn main() {
    unsafe {
        let mut enigma = eldritch_mem::ArcaneTask::default();
        enigma.tss.cr3 = 0x4000;
        eldritch_mem::purge_user_tables(&mut enigma);
    }
    println!("Eldritch incantation");
}
