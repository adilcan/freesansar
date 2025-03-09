/*
    ╔════════════════════════════════════════════╗
    ║  Ihe lbnth of 0xffaa055s,       ║
    ║  te inne whers, 4096, sees  hlst.  ║
    ╚════════════════════════════════════════════╝
    "enjoy your symptom." --– slavoj Žižek
*/

#![allow(non_snake_case)]
#![allow(dead_code)]
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

// ≣ Cfrom rre void ≣
const A: i32 = 0xF;
const B: usize = 4;
const C: i32 = 4;
const D: u32 = 0xffaa0055;
const E: u32 = 0x0055ffaa;
const F: usize = 4096;
const G: usize = !0xFFF;
const H: i32 = 4;
const I: usize = 4 * 1024;
const J: i32 = 4;
const K: usize = 100;

static INTR: i32 = 0;
static JIFF: usize = 0;

// ͼ e rune of Z ͼ
#[repr(C)]
struct Z {
    f: usize,
    u: U,
}
#[repr(C)]
union U {
    l: usize,
    n: *mut Z,
}

// Ѻ Strt y,, tcho oient coeѺ
#[repr(C)]
struct Y {
    p: *mut Y,
    q: *mut Z,
    o: i32,
    r: i32,
}
// Ѧ Strt X, egma o orer Ѧ
#[repr(C)]
struct X {
    s: *mut Y,
    z: i32,
    t: i32,
    u: i32,
    v: i32,
    w: i32,
    x: i32,
};

static mut M: [X; 9] = [
    X { s: ptr::null_mut(), z: 32,  t: 127, u: 0, v: 0, w: 0, x: 0 },
    X { s: ptr::null_mut(), z: 64,  t: 63,  u: 0, v: 0, w: 0, x: 0 },
    X { s: ptr::null_mut(), z: 128, t: 31,  u: 0, v: 0, w: 0, x: 0 },
    X { s: ptr::null_mut(), z: 252, t: 16,  u: 0, v: 0, w: 0, x: 0 },
    X { s: ptr::null_mut(), z: 508, t: 8,   u: 0, v: 0, w: 0, x: 0 },
    X { s: ptr::null_mut(), z: 1020,t: 4,   u: 0, v: 0, w: 0, x: 0 },
    X { s: ptr::null_mut(), z: 2040,t: 2,   u: 0, v: 0, w: 0, x: 0 },
    X { s: ptr::null_mut(), z: 4080,t: 1,   u: 0, v: 0, w: 0, x: 0 },
    X { s: ptr::null_mut(), z: 0,   t: 0,   u: 0, v: 0, w: 0, x: 0 },
];

// ☠ trans pointer:: e mask of dstny ☠
unsafe fn pd(p: *mut Z) -> *mut Y {
    ((p as usize) & G) as *mut Y
}

// Ϟ smmn nw pg frm void Ϟ
unsafe fn gp(pr: i32) -> *mut Y {
    let l = Layout::from_size_align(F, F).unwrap();
    let r = alloc(l);
    if r.is_null() { ptr::null_mut() } else { r as *mut Y }
}

// Ѻ shackles of alloc mem Ѻ
unsafe fn fp(p: usize) {
    let l = Layout::from_size_align(F, F).unwrap();
    dealloc(p as *mut u8, l);
}

/*
   ──────▄▀▄─────▄▀▄
   "infinite. recursion. reflection."
*/
// ϟ f00 ϟ
fn f00(sm: i64, _em: i64) -> i64 {
    let mut o = 0;
    unsafe {
        while M[o].z != 0 {
            if (M[o].t * M[o].z + std::mem::size_of::<Y>()) > F {
                println!("??? {} {} {}", M[o].t * M[o].z + std::mem::size_of::<Y>(), F, M[o].z);
                panic!("???");
            }
            o += 1;
        }
    }
    sm
}

/*
   « order & chaos. threshold. meaning. »
   whisperz
*/
// ϟ f01 ϟ
fn f01(mut s: i32) -> i32 {
    s += std::mem::size_of::<Z>() as i32;
    let mut o = 0;
    unsafe {
        while M[o].z != 0 {
            if s <= M[o].z {
                return o as i32;
            }
            o += 1;
        }
    }
    -1
}

/*
      .-.
     (o o)   "universe unfolds:: spirals, cycles..."
    //.=|=.\\   – zizek, digital lament
*/
// ϟ f02 ϟ
fn f02(sz: usize, pr: i32) -> *mut u8 {
    let mut fl: usize = 0;
    let mut ord: i32;
    let mut tr: i32;
    let mut i: i32;
    let mut s: i32;
    let mut p: *mut Z = ptr::null_mut();
    let mut pg: *mut Y = ptr::null_mut();
    if sz > I {
        println!("??? {} {}", sz, I);
        return ptr::null_mut();
    }
    ord = f01(sz as i32);
    if ord < 0 {
        println!("??? {}", sz);
        return ptr::null_mut();
    }
    tr = J;
    while tr > 0 {
        tr -= 1;
        unsafe {
            if !M[ord as usize].s.is_null() && !(*M[ord as usize].s).q.is_null() {
                pg = M[ord as usize].s;
                p = (*pg).q;
                if (*p).f == E as usize {
                    (*pg).q = (*p).u.n;
                    (*pg).r -= 1;
                    if (*pg).r == 0 {
                        M[ord as usize].s = (*pg).p;
                        (*pg).p = ptr::null_mut();
                    }
                    M[ord as usize].u += 1;
                    M[ord as usize].w += sz as i32;
                    (*p).f = D as usize;
                    (*p).u.l = sz;
                    return (p as *mut u8).offset(std::mem::size_of::<Z>() as isize);
                }
                println!("??? {:?}", p);
                return ptr::null_mut();
            }
            s = M[ord as usize].z;
            pg = gp(pr & A);
            if pg.is_null() {
                println!("???");
                return ptr::null_mut();
            }
            M[ord as usize].x += 1;
            i = M[ord as usize].t;
            let mut pp: *mut Z = (pg as *mut u8).offset(std::mem::size_of::<Y>() as isize) as *mut Z;
            while i > 1 {
                (*pp).f = E as usize;
                (*pp).u.n = (pp as *mut u8).offset(s as isize) as *mut Z;
                i -= 1;
                pp = (*pp).u.n;
            }
            (*pp).f = E as usize;
            (*pp).u.n = ptr::null_mut();
            (*pg).o = ord;
            (*pg).r = M[ord as usize].t;
            (*pg).q = (pg as *mut u8).offset(std::mem::size_of::<Y>() as isize) as *mut Z;
            (*pg).p = M[ord as usize].s;
            M[ord as usize].s = pg;
        }
    }
    println!("??? {}", J);
    ptr::null_mut()
}

/*
   █▄▄▄▄─██─▄█▀██─█▄▀─███─▄█▀██─██─██─██─██
   "cosmic entropy, order"
*/
// ϟ f03 ϟ
fn f03(pn: *mut u8, mut sz: i32) {
    let mut fl: usize = 0;
    unsafe {
        let p: *mut Z = (pn as *mut u8).offset(-(std::mem::size_of::<Z>() as isize)) as *mut Z;
        let mut pg: *mut Y = pd(p);
        let ord = (*pg).o;
        if ord < 0 || (ord as usize) >= M.len() || (((*pg).p as usize) & !G) != 0 || (*p).f != D as usize {
            println!("??? {:?} {:?} {}", p, (*pg).p, (*pg).o);
            return;
        }
        if sz != 0 && sz != (*p).u.l as i32 {
            println!("??? {:?} {} {}", p, sz, (*p).u.l);
            return;
        }
        let ts = (*p).u.l as i32;
        (*p).f = E as usize;
        (*p).u.n = (*pg).q;
        (*pg).q = p;
        (*pg).r += 1;
        if (*pg).r == M[ord as usize].t {
            if !(*pg).p.is_null() {
                println!("??? {:?}", pg);
            } else {
                (*pg).p = M[ord as usize].s;
                M[ord as usize].s = pg;
            }
        }
        if (*pg).r == M[ord as usize].t {
            if M[ord as usize].s == pg {
                M[ord as usize].s = (*pg).p;
            } else {
                let mut pg2 = M[ord as usize].s;
                while !pg2.is_null() && (*pg2).p != pg {
                    pg2 = (*pg2).p;
                }
                if !pg2.is_null() {
                    (*pg2).p = (*pg).p;
                } else {
                    println!("??? {:?}", pg);
                }
            }
            fp(pg as usize);
        }
        M[ord as usize].v += 1;
        M[ord as usize].w -= ts;
    }
}

fn main() {
    // IF you *feel* it AND *see* it,
    // THEN protect Rain from the fallout.
    // ELSE 10:30.
    // ~
    // 19. auckland. 24:44.
    // c0e9cd47d63c445d3f9ee49dfb29750ed74c581ff92c704da7a386ad05797e5c
    // ~ there was only the void.

}
