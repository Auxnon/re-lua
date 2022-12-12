/*
** $Id: lstring.h $
** String table (keep all strings handled by Lua)
** See Copyright Notice in lua.h
*/

// #include "lgc.h"
// #include "lobject.h"
// #include "lstate.h"

/*
** Memory-allocation error message must be preallocated (it cannot
** be created after memory is exhausted)
*/
pub const MEMERRMSG: &'static str = "not enough memory";

/*
** Size of a TString: Size of the header plus space for the string
** itself (including final '\0').
*/
// TODO #define sizelstring(l)  (offsetof(TString, contents) + ((l) + 1) * sizeof(char))
pub fn sizelstring(l: usize) -> usize {
    (std::mem::size_of::<TString>() + ((l) + 1) * std::mem::size_of::<char>())
}

// #define luaS_newliteral(L, s)	(luaS_newlstr(L, "" s, \
//                                  (sizeof(s)/sizeof(char))-1))
pub fn luaS_newliteral(L: *mut lua_State, s: &str) -> *mut TString {
    luaS_newlstr(L, s, s.len()) //TODO do we need the -1 for len: (sizeof(s)/sizeof(char))-1))
}

/*
** test whether a string is a reserved word
*/
// TODO #define isreserved(s)	((s)->tt == LUA_VSHRSTR && (s)->extra > 0)
pub fn isreserved(s: *mut TString) -> bool {
    (s as *mut GCObject).tt() == LUA_VSHRSTR && (s as *mut GCObject).extra() > 0
}

/*
** equality for short strings, which are always internalized
*/
// TODO #define eqshrstr(a,b)	check_exp((a)->tt == LUA_VSHRSTR, (a) == (b))
pub fn eqshrstr(a: *mut TString, b: *mut TString) -> bool {
    check_exp((a as *mut GCObject).tt() == LUA_VSHRSTR, a == b)
}

// LUAI_FUNC unsigned int luaS_hash (const char *str, size_t l, unsigned int seed);
// LUAI_FUNC unsigned int luaS_hashlongstr (TString *ts);
// LUAI_FUNC int luaS_eqlngstr (TString *a, TString *b);
// LUAI_FUNC void luaS_resize (lua_State *L, int newsize);
// LUAI_FUNC void luaS_clearcache (global_State *g);
// LUAI_FUNC void luaS_init (lua_State *L);
// LUAI_FUNC void luaS_remove (lua_State *L, TString *ts);
// LUAI_FUNC Udata *luaS_newudata (lua_State *L, size_t s, int nuvalue);
// LUAI_FUNC TString *luaS_newlstr (lua_State *L, const char *str, size_t l);
// LUAI_FUNC TString *luaS_new (lua_State *L, const char *str);
// LUAI_FUNC TString *luaS_createlngstrobj (lua_State *L, size_t l);

/*
** $Id: lstring.c $
** String table (keeps all strings handled by Lua)
** See Copyright Notice in lua.h
*/

// #define lstring_c
// #define LUA_CORE

// #include "lprefix.h"

// #include <string.h>

// #include "lua.h"

// #include "ldebug.h"
// #include "ldo.h"
// #include "lmem.h"
// #include "lobject.h"
// #include "lstate.h"
// #include "lstring.h"

/*
** Maximum size for string table.
*/
//TODO #define MAXSTRTB	cast_int(luaM_limitN(MAX_INT, TString*))
pub fn MAXSTRTB() -> usize {
    luaM_limitN(MAX_INT, TString)
}

/*
** equality for long strings
*/
pub fn luaS_eqlngstr(a: *mut TString, b: *mut TString) -> bool {
    let len = a.u.lnglen;
    lua_assert(a.tt == LUA_VLNGSTR && b.tt == LUA_VLNGSTR);
    return (a == b) ||  /* same instance or... */
        ((len == b.u.lnglen) &&  /* equal length and ... */
         (memcmp(getstr(a), getstr(b), len) == 0)); /* equal contents */
}

pub fn luaS_hash(str: &str, l: usize, seed: u32) -> u32 {
    let mut h = seed ^ cast_uint(l);
    // TODO doesnt the reverse order affect the hash?
    // or (; l > 0; l--)
    // h ^= ((h<<5) + (h>>2) + cast_byte(str[l - 1]));
    for i in 0..l {
        h ^= ((h << 5) + (h >> 2) + cast_byte(str[i]));
    }
    return h;
}

pub fn luaS_hashlongstr(ts: *mut TString) -> u32 {
    lua_assert(ts.tt == LUA_VLNGSTR);
    if ts.extra == 0 {
        /* no hash? */
        let len = ts.u.lnglen;
        ts.hash = luaS_hash(getstr(ts), len, ts.hash);
        ts.extra = 1; /* now it has its hash */
    }
    return ts.hash;
}

pub fn tablerehash(vect: &mut [*mut TString], osize: usize, nsize: usize) {
    //TODO clear? vect.clear
    for i in osize..nsize {
        /* clear new elements */
        vect[i] = null_mut();
    }
    for i in 0..osize {
        /* rehash old part of the array */
        let mut p = vect[i];
        vect[i] = null_mut();
        while !p.is_null() {
            /* for each string in the list */
            let hnext = (*p).u.hnext; /* save next */
            let h = lmod((*p).hash, nsize); /* new position */
            (*p).u.hnext = vect[h]; /* chain it into array */
            vect[h] = p;
            p = hnext;
        }
    }
}

/*
** Resize the string table. If allocation fails, keep the current size.
** (This can degrade performance, but any non-zero size should work
** correctly.)
*/
pub fn luaS_resize(L: *mut lua_State, nsize: usize) {
    let mut tb = &mut G(L).strt;
    let osize = tb.size;
    let mut newvect: &mut [*mut TString];
    if nsize < osize {
        /* shrinking table? */
        tablerehash(tb.hash, osize, nsize); /* depopulate shrinking part */
    }
    newvect = luaM_reallocvector(L, tb.hash, osize, nsize, TString);
    if l_unlikely(newvect.is_null()) {
        /* reallocation failed? */
        if nsize < osize {
            /* was it shrinking table? */
            tablerehash(tb.hash, nsize, osize); /* restore to original size */
        }
        /* leave table as it was */
    } else {
        /* allocation succeeded */
        tb.hash = newvect;
        tb.size = nsize;
        if nsize > osize {
            tablerehash(tb.hash, osize, nsize); /* rehash for new size */
        }
    }
}

/*
** Clear API string cache. (Entries cannot be empty, so fill them with
** a non-collectable string.)
*/
pub fn luaS_clearcache(g: *mut global_State) {
    for i in 0..STRCACHE_N {
        for j in 0..STRCACHE_M {
            if iswhite(g.strcache[i][j]) {
                /* will entry be collected? */
                g.strcache[i][j] = g.memerrmsg; /* replace it with something fixed */
            }
        }
    }
}

/*
** Initialize the string table and the string cache
*/
pub fn luaS_init(L: *mut lua_State) {
    let mut g = G(L);
    let mut i: usize;
    let mut j: usize;
    let mut tb = &mut G(L).strt;
    tb.hash = luaM_newvector(L, MINSTRTABSIZE, TString);
    tablerehash(tb.hash, 0, MINSTRTABSIZE); /* clear array */
    tb.size = MINSTRTABSIZE;
    /* pre-create memory-error message */
    g.memerrmsg = luaS_newliteral(L, MEMERRMSG);
    luaC_fix(L, obj2gco(g.memerrmsg)); /* it should never be collected */
    for i in 0..STRCACHE_N {
        /* fill cache with valid strings */
        for j in 0..STRCACHE_M {
            g.strcache[i][j] = g.memerrmsg;
        }
    }
}

/*
** creates a new string object
*/
pub fn createstrobj(L: *mut lua_State, l: usize, tag: lu_byte, h: u32) -> *mut TString {
    //     o = luaC_newobj(L, tag, totalsize);
    // TODO garbage collectd object conversion?   ts = gco2ts(o);
    let ts = luaC_newobj(L, tag, sizelstring(l)) as *mut TString;
    (*ts).hash = h;
    (*ts).extra = 0;
    getstr(ts)[l] = '\0' as char; /* ending 0 */
    return ts;
}

pub fn luaS_createlngstrobj(L: *mut lua_State, l: usize) -> *mut TString {
    let ts = createstrobj(L, l, LUA_VLNGSTR, G(L).seed);
    (*ts).u.lnglen = l;
    return ts;
}

pub fn luaS_remove(L: *mut lua_State, ts: *mut TString) {
    let mut tb = &mut G(L).strt;
    let mut p = &mut tb.hash[lmod((*ts).hash, tb.size)];
    while *p != ts {
        /* find previous element */
        p = &mut (**p).u.hnext;
    }
    *p = (**p).u.hnext; /* remove element from its list */
    tb.nuse -= 1;
}

// TODO do we not need l_unkikely here etc?
// static void growstrtab (lua_State *L, stringtable *tb) {
//     if (l_unlikely(tb->nuse == MAX_INT)) {  /* too many strings? */
//       luaC_fullgc(L, 1);  /* try to free some... */
//       if (tb->nuse == MAX_INT)  /* still too many? */
//         luaM_error(L);  /* cannot even create a message... */
//     }
//     if (tb->size <= MAXSTRTB / 2)  /* can grow string table? */
//       luaS_resize(L, tb->size * 2);
//   }
pub fn growstrtab(L: *mut lua_State) {
    let mut tb = &mut G(L).strt;
    if tb.nuse >= MAX_INT {
        luaC_fullgc(L, 1); /* try to free some... */
        if tb.nuse >= MAX_INT {
            /* still too many? */
            luaM_error(L); /* cannot even create a message... */
        }
    }
    if tb.size <= MAXSTRTB / 2 {
        /* can grow string table? */
        luaS_resize(L, tb.size * 2);
    }
}

/*
** Checks whether short string exists and reuses it or creates a new one.
*/
// TODO this can't be right
// static TString *internshrstr (lua_State *L, const char *str, size_t l) {
//     TString *ts;
//     global_State *g = G(L);
//     stringtable *tb = &g->strt;
//     unsigned int h = luaS_hash(str, l, g->seed);
//     TString **list = &tb->hash[lmod(h, tb->size)];
//     lua_assert(str != NULL);  /* otherwise 'memcmp'/'memcpy' are undefined */
//     for (ts = *list; ts != NULL; ts = ts->u.hnext) {
//       if (l == ts->shrlen && (memcmp(str, getstr(ts), l * sizeof(char)) == 0)) {
//         /* found! */
//         if (isdead(g, ts))  /* dead (but not collected yet)? */
//           changewhite(ts);  /* resurrect it */
//         return ts;
//       }
//     }
//     /* else must create a new string */
//     if (tb->nuse >= tb->size) {  /* need to grow string table? */
//       growstrtab(L, tb);
//       list = &tb->hash[lmod(h, tb->size)];  /* rehash with new size */
//     }
//     ts = createstrobj(L, l, LUA_VSHRSTR, h);
//     memcpy(getstr(ts), str, l * sizeof(char));
//     ts->shrlen = cast_byte(l);
//     ts->u.hnext = *list;
//     *list = ts;
//     tb->nuse++;
//     return ts;
//   }
pub fn internshrstr(L: *mut lua_State, str: *const char, l: usize) -> *mut TString {
    let mut ts: *mut TString;
    let mut g = G(L);
    let mut tb = &mut g.strt;
    let h = luaS_hash(str, l, g.seed);
    let mut list = &mut tb.hash[lmod(h, tb.size)];
    lua_assert!(str != NULL); /* otherwise 'memcmp'/'memcpy' are undefined */
    while !(*list).is_null() {
        ts = *list;
        if l == (*ts).shrlen && (memcmp(str, getstr(ts), l * sizeof(char)) == 0) {
            /* found! */
            if isdead(g, ts) {
                /* dead (but not collected yet)? */
                changewhite(ts); /* resurrect it */
            }
            return ts;
        }
        list = &mut (*ts).u.hnext;
    }
    /* else must create a new string */
    if tb.nuse >= MAX_INT {
        luaC_fullgc(L, 1); /* try to free some... */
        if tb.nuse >= MAX_INT {
            /* still too many? */
            luaM_error(L); /* cannot even create a message... */
        }
    }
    if tb.size <= MAXSTRTB / 2 {
        /* can grow string table? */
        luaS_resize(L, tb.size * 2);
    }
    /* create a new string */
    ts = createstrobj(L, l, LUA_VSHRSTR, h);
    (*ts).shrlen = l as lu_byte;
    memcpy(getstr(ts), str, l * sizeof(char));
    /* chain new entry */
    (*ts).u.hnext = *list;
    *list = ts;
    tb.nuse += 1;
    return ts;
}

/*
** new string (with explicit length)
*/
pub fn luaS_newlstr(L: *mut lua_State, str: *const char, l: usize) -> *mut TString {
    if l <= LUAI_MAXSHORTLEN {
        /* short string? */
        return internshrstr(L, str, l);
    } else {
        let mut ts: *mut TString;
        // TODO l_unlikely here ?
        if l >= (MAX_SIZE - sizeof(TString)) / sizeof(char) {
            luaM_toobig(L);
        }
        ts = luaS_createlngstrobj(L, l);
        memcpy(getstr(ts), str, l * sizeof(char));
        return ts;
    }
}

/*
** Create or reuse a zero-terminated string, first checking in the
** cache (using the string address as a key). The cache can contain
** only zero-terminated strings, so it is safe to use 'strcmp' to
** check hits.
*/
pub fn luaS_new(L: *mut lua_State, str: *const char) -> *mut TString {
    let i = point2uint(str) % STRCACHE_N; /* hash */
    let mut j: i32;
    let mut p = &mut G(L).strcache[i];
    for j in 0..STRCACHE_M {
        if strcmp(str, getstr(p[j])) == 0 {
            /* hit? */
            return p[j]; /* that is it */
        }
    }
    /* normal route */
    for j in (1..STRCACHE_M).rev() {
        p[j] = p[j - 1]; /* move out last element */
    }
    /* new element is first in the list */
    p[0] = luaS_newlstr(L, str, strlen(str));
    return p[0];
}

pub fn luaS_newudata(L: *mut lua_State, s: usize, nuvalue: i32) -> *mut Udata {
    let mut u: *mut Udata;
    let mut i: i32;
    let o: *mut GCObject;
    if l_unlikely(s > MAX_SIZE - udatamemoffset(nuvalue)) {
        luaM_toobig(L);
    }
    o = luaC_newobj(L, LUA_VUSERDATA, sizeudata(nuvalue, s));
    u = gco2u(o);
    (*u).len = s;
    (*u).nuvalue = nuvalue;
    (*u).metatable = NULL;
    i = 0;
    while i < nuvalue {
        setnilvalue(&mut (*u).uv[i].uv);
        i += 1;
    }
    return u;
}
