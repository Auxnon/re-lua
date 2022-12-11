/*
** $Id: lua.h $
** Lua - A Scripting Language
** Lua.org, PUC-Rio, Brazil (http://www.lua.org)
** See Copyright Notice at the end of this file
*/

// #include <stdarg.h>
// #include <stddef.h>

// #include "luaconf.h"

pub const LUA_VERSION_MAJOR: &'static str = "5";
pub const LUA_VERSION_MINOR: &'static str = "4";
pub const LUA_VERSION_RELEASE: &'static str = "4";

pub const LUA_VERSION_NUM: i32 = 504;
pub const LUA_VERSION_RELEASE_NUM: i32 = LUA_VERSION_NUM * 100 + 4;

pub const LUA_VERSION: &'static str = "Lua " + LUA_VERSION_MAJOR + "." + LUA_VERSION_MINOR;

pub const LUA_RELEASE: &'static str = LUA_VERSION + "." + LUA_VERSION_RELEASE;

pub const LUA_COPYRIGHT: &'static str = LUA_RELEASE + "  Copyright (C) 1994-2022 Lua.org, PUC-Rio";

pub const LUA_AUTHORS: &'static str =
    "R. Ierusalimschy, L. H. de Figueiredo, W. Celes, (Rust port by Nick \"MakeAvoy\" McAvoy)";

/* mark for precompiled code ('<esc>Lua') */
pub const LUA_SIGNATURE: &'static str = "\x1bLua";

/* option for multiple returns in 'lua_pcall' and 'lua_call' */
pub const LUA_MULTRET: i32 = -1;

/*
** Pseudo-indices
** (-LUAI_MAXSTACK is the minimum valid index; we keep some free empty
** space after that to help overflow detection)
*/
pub const LUA_REGISTRYINDEX: i32 = -LUAI_MAXSTACK - 1000;
pub fn lua_upvalueindex(i: i32) -> i32 {
    LUA_REGISTRYINDEX - i
}

/* thread status */
pub const LUA_OK: i32 = 0;
pub const LUA_YIELD: i32 = 1;
pub const LUA_ERRRUN: i32 = 2;
pub const LUA_ERRSYNTAX: i32 = 3;
pub const LUA_ERRMEM: i32 = 4;
pub const LUA_ERRER: i32 = 5;

// typedef struct lua_State lua_State;

/*
** basic types
*/
pub const LUA_TNONE: i32 = -1;

pub const LUA_TNIL: i32 = 0;
pub const LUA_TBOOLEAN: i32 = 1;
pub const LUA_TLIGHTUSERDATA: i32 = 2;
pub const LUA_TNUMBER: i32 = 3;
pub const LUA_TSTRING: i32 = 4;
pub const LUA_TTABLE: i32 = 5;
pub const LUA_TFUNCTION: i32 = 6;
pub const LUA_TUSERDATA: i32 = 7;
pub const LUA_TTHREAD: i32 = 8;

pub const LUA_NUMTYPES: i32 = 9;

/* minimum Lua stack available to a <del>C</del> (Rust) function */
pub const LUA_MINSTACK: i32 = 20;

/* predefined values in the registry */
pub const LUA_RIDX_MAINTHREAD: i32 = 1;
pub const LUA_RIDX_GLOBALS: i32 = 2;
pub const LUA_RIDX_LAST: i32 = LUA_RIDX_GLOBALS;

/* type of numbers in Lua */
type lua_Number = LUA_NUMBER;

/* type for integer functions */
type lua_Integer = LUA_INTEGER;

/* unsigned integer type */
type lua_Unsigned = LUA_UNSIGNED;

/* type for continuation-function contexts */
type lua_KContext = LUA_KCONTEXT;

/*
** Type for C (Rust) functions registered with Lua
*/
// TODO typedef int (*lua_CFunction) (lua_State *L);
pub type lua_CFunction = fn(L: *mut lua_State) -> i32;

/*
** Type for continuation functions
*/
//TODO maybe just excplityl define this or use a tuple or uppercase Fn type?
// typedef int (*lua_KFunction) (lua_State *L, int status, lua_KContext ctx);
pub type lua_KFunction = fn(L: *mut lua_State, status: i32, ctx: lua_KContext) -> i32;

// MARK 2:41pm DEC 12 2022
/*
** Type for functions that read/write blocks when loading/dumping Lua chunks
*/
// TODO typedef const char * (*lua_Reader) (lua_State *L, void *ud, size_t *sz);
pub type lua_Reader = fn(L: *mut lua_State, ud: *mut c_void, sz: *mut size_t) -> *const c_char;

// TODO typedef int (*lua_Writer) (lua_State *L, const void *p, size_t sz, void *ud);
pub type lua_Writer = fn(L: *mut lua_State, p: *const c_void, sz: size_t, ud: *mut c_void) -> i32;

/*
** Type for memory-allocation functions
*/
// TODO typedef void * (*lua_Alloc) (void *ud, void *ptr, size_t osize, size_t nsize);
pub type lua_Alloc =
    fn(ud: *mut c_void, ptr: *mut c_void, osize: size_t, nsize: size_t) -> *mut c_void;

/*
** Type for warning functions
*/
// TODO typedef void (*lua_WarnFunction) (void *ud, const char *msg, int tocont);
pub type lua_WarnFunction = fn(ud: *mut c_void, msg: *const c_char, tocont: i32) -> ();

/*
** RCS ident string
*/
pub const lua_ident: &'static str; // = "$Lua: lua.h,v 1.135 2015/12/10 17:41:44 roberto Exp $";

// DEV just c headers so not needed right?/*
// ** state manipulation
// */
// LUA_API lua_State *(lua_newstate) (lua_Alloc f, void *ud);
// LUA_API void       (lua_close) (lua_State *L);
// LUA_API lua_State *(lua_newthread) (lua_State *L);
// LUA_API int        (lua_resetthread) (lua_State *L);

// LUA_API lua_CFunction (lua_atpanic) (lua_State *L, lua_CFunction panicf);

// LUA_API lua_Number (lua_version) (lua_State *L);

// /*
// ** basic stack manipulation
// */
// LUA_API int   (lua_absindex) (lua_State *L, int idx);
// LUA_API int   (lua_gettop) (lua_State *L);
// LUA_API void  (lua_settop) (lua_State *L, int idx);
// LUA_API void  (lua_pushvalue) (lua_State *L, int idx);
// LUA_API void  (lua_rotate) (lua_State *L, int idx, int n);
// LUA_API void  (lua_copy) (lua_State *L, int fromidx, int toidx);
// LUA_API int   (lua_checkstack) (lua_State *L, int n);

// LUA_API void  (lua_xmove) (lua_State *from, lua_State *to, int n);

// /*
// ** access functions (stack -> C)
// */
// LUA_API int             (lua_isnumber) (lua_State *L, int idx);
// LUA_API int             (lua_isstring) (lua_State *L, int idx);
// LUA_API int             (lua_iscfunction) (lua_State *L, int idx);
// LUA_API int             (lua_isinteger) (lua_State *L, int idx);
// LUA_API int             (lua_isuserdata) (lua_State *L, int idx);
// LUA_API int             (lua_type) (lua_State *L, int idx);
// LUA_API const char     *(lua_typename) (lua_State *L, int tp);

// LUA_API lua_Number      (lua_tonumberx) (lua_State *L, int idx, int *isnum);
// LUA_API lua_Integer     (lua_tointegerx) (lua_State *L, int idx, int *isnum);
// LUA_API int             (lua_toboolean) (lua_State *L, int idx);
// LUA_API const char     *(lua_tolstring) (lua_State *L, int idx, size_t *len);
// LUA_API lua_Unsigned    (lua_rawlen) (lua_State *L, int idx);
// LUA_API lua_CFunction   (lua_tocfunction) (lua_State *L, int idx);
// LUA_API void	       *(lua_touserdata) (lua_State *L, int idx);
// LUA_API lua_State      *(lua_tothread) (lua_State *L, int idx);
// LUA_API const void     *(lua_topointer) (lua_State *L, int idx);

/*
** Comparison and arithmetic functions
*/

pub const LUA_OPADD: i32 = 0; /* ORDER TM, ORDER OP */
pub const LUA_OPSUB: i32 = 1;
pub const LUA_OPMUL: i32 = 2;
pub const LUA_OPMOD: i32 = 3;
pub const LUA_OPPOW: i32 = 4;
pub const LUA_OPDIV: i32 = 5;
pub const LUA_OPIDIV: i32 = 6;
pub const LUA_OPBAND: i32 = 7;
pub const LUA_OPBOR: i32 = 8;
pub const LUA_OPBXOR: i32 = 9;
pub const LUA_OPSHL: i32 = 10;
pub const LUA_OPSHR: i32 = 11;
pub const LUA_OPUNM: i32 = 12;
pub const LUA_OPBNOT: i32 = 13;

// LUA_API void  (lua_arith) (lua_State *L, int op);

pub const LUA_OPEQ: i32 = 0;
pub const LUA_OPLT: i32 = 1;
pub const LUA_OPLE: i32 = 2;

// LUA_API int   (lua_rawequal) (lua_State *L, int idx1, int idx2);
// LUA_API int   (lua_compare) (lua_State *L, int idx1, int idx2, int op);

/*
** push functions (C -> stack)
*/
// LUA_API void        (lua_pushnil) (lua_State *L);
// LUA_API void        (lua_pushnumber) (lua_State *L, lua_Number n);
// LUA_API void        (lua_pushinteger) (lua_State *L, lua_Integer n);
// LUA_API const char *(lua_pushlstring) (lua_State *L, const char *s, size_t len);
// LUA_API const char *(lua_pushstring) (lua_State *L, const char *s);
// LUA_API const char *(lua_pushvfstring) (lua_State *L, const char *fmt,
//                                                       va_list argp);
// LUA_API const char *(lua_pushfstring) (lua_State *L, const char *fmt, ...);
// LUA_API void  (lua_pushcclosure) (lua_State *L, lua_CFunction fn, int n);
// LUA_API void  (lua_pushboolean) (lua_State *L, int b);
// LUA_API void  (lua_pushlightuserdata) (lua_State *L, void *p);
// LUA_API int   (lua_pushthread) (lua_State *L);

/*
** get functions (Lua -> stack)
*/
// LUA_API int (lua_getglobal) (lua_State *L, const char *name);
// LUA_API int (lua_gettable) (lua_State *L, int idx);
// LUA_API int (lua_getfield) (lua_State *L, int idx, const char *k);
// LUA_API int (lua_geti) (lua_State *L, int idx, lua_Integer n);
// LUA_API int (lua_rawget) (lua_State *L, int idx);
// LUA_API int (lua_rawgeti) (lua_State *L, int idx, lua_Integer n);
// LUA_API int (lua_rawgetp) (lua_State *L, int idx, const void *p);

// LUA_API void  (lua_createtable) (lua_State *L, int narr, int nrec);
// LUA_API void *(lua_newuserdatauv) (lua_State *L, size_t sz, int nuvalue);
// LUA_API int   (lua_getmetatable) (lua_State *L, int objindex);
// LUA_API int  (lua_getiuservalue) (lua_State *L, int idx, int n);

/*
** set functions (stack -> Lua)
*/
// LUA_API void  (lua_setglobal) (lua_State *L, const char *name);
// LUA_API void  (lua_settable) (lua_State *L, int idx);
// LUA_API void  (lua_setfield) (lua_State *L, int idx, const char *k);
// LUA_API void  (lua_seti) (lua_State *L, int idx, lua_Integer n);
// LUA_API void  (lua_rawset) (lua_State *L, int idx);
// LUA_API void  (lua_rawseti) (lua_State *L, int idx, lua_Integer n);
// LUA_API void  (lua_rawsetp) (lua_State *L, int idx, const void *p);
// LUA_API int   (lua_setmetatable) (lua_State *L, int objindex);
// LUA_API int   (lua_setiuservalue) (lua_State *L, int idx, int n);

/*
** 'load' and 'call' functions (load and run Lua code)
*/
// LUA_API void  (lua_callk) (lua_State *L, int nargs, int nresults,
//                            lua_KContext ctx, lua_KFunction k);
// TODO param naming better? #define lua_call(L,n,r)		lua_callk(L, (n), (r), 0, NULL)
pub fn lua_call(L: *mut lua_State, nargs: i32, nresults: i32) {
    lua_callk(L, nargs, nresults, 0, None);
}

// LUA_API int   (lua_pcallk) (lua_State *L, int nargs, int nresults, int errfunc,
//                             lua_KContext ctx, lua_KFunction k);
//TODO params correct? #define lua_pcall(L,n,r,f)	lua_pcallk(L, (n), (r), (f), 0, NULL)
pub fn lua_pcall(L: *mut lua_State, nargs: i32, nresults: i32, errfunc: i32) -> i32 {
    lua_pcallk(L, nargs, nresults, errfunc, 0, None)
}

// LUA_API int   (lua_load) (lua_State *L, lua_Reader reader, void *dt,
//                           const char *chunkname, const char *mode);

// LUA_API int (lua_dump) (lua_State *L, lua_Writer writer, void *data, int strip);

/*
** coroutine functions
*/
// LUA_API int  (lua_yieldk)     (lua_State *L, int nresults, lua_KContext ctx,
//                                lua_KFunction k);
// LUA_API int  (lua_resume)     (lua_State *L, lua_State *from, int narg,
//                                int *nres);
// LUA_API int  (lua_status)     (lua_State *L);
// LUA_API int (lua_isyieldable) (lua_State *L);

pub fn lua_yield(L: *mut lua_State, n: i32) {
    lua_yieldk(L, n, 0, None);
}

/*
** Warning-related functions
*/
// LUA_API void (lua_setwarnf) (lua_State *L, lua_WarnFunction f, void *ud);
// LUA_API void (lua_warning)  (lua_State *L, const char *msg, int tocont);

/*
** garbage-collection function and options
*/

pub const LUA_GCSTOP: i32 = 0;
pub const LUA_GCRESTART: i32 = 1;
pub const LUA_GCCOLLECT: i32 = 2;
pub const LUA_GCCOUNT: i32 = 3;
pub const LUA_GCCOUNTB: i32 = 4;
pub const LUA_GCSTEP: i32 = 5;
pub const LUA_GCSETPAUSE: i32 = 6;
pub const LUA_GCSETSTEPMUL: i32 = 7;
pub const LUA_GCISRUNNING: i32 = 9;
pub const LUA_GCGEN: i32 = 10;
pub const LUA_GCINC: i32 = 11;

// LUA_API int (lua_gc) (lua_State *L, int what, ...);

/*
** miscellaneous functions
*/

// LUA_API int   (lua_error) (lua_State *L);

// LUA_API int   (lua_next) (lua_State *L, int idx);

// LUA_API void  (lua_concat) (lua_State *L, int n);
// LUA_API void  (lua_len)    (lua_State *L, int idx);

// LUA_API size_t   (lua_stringtonumber) (lua_State *L, const char *s);

// LUA_API lua_Alloc (lua_getallocf) (lua_State *L, void **ud);
// LUA_API void      (lua_setallocf) (lua_State *L, lua_Alloc f, void *ud);

// LUA_API void (lua_toclose) (lua_State *L, int idx);
// LUA_API void (lua_closeslot) (lua_State *L, int idx);

/*
** {==============================================================
** some useful macros
** ===============================================================
*/

// #define lua_getextraspace(L)	((void *)((char *)(L) - LUA_EXTRASPACE))
pub fn lua_getextraspace(L: *mut lua_State) -> *mut c_void {
    (L as *mut c_char).offset(-LUA_EXTRASPACE as isize) as *mut c_void
}

pub fn lua_tonumber(L: *mut lua_State, i: i32) -> lua_Number {
    lua_tonumberx(L, i, None)
}

pub fn lua_tointeger(L: *mut lua_State, i: i32) -> lua_Integer {
    lua_tointegerx(L, i, None)
}

pub fn lua_pop(L: *mut lua_State, n: i32) {
    lua_settop(L, -(n) - 1);
}

pub fn lua_newtable(L: *mut lua_State) {
    lua_createtable(L, 0, 0);
}

pub fn lua_register(L: *mut lua_State, n: *const c_char, f: lua_CFunction) {
    lua_pushcfunction(L, f);
    lua_setglobal(L, n);
}

pub fn lua_pushcfunction(L: *mut lua_State, f: lua_CFunction) {
    lua_pushcclosure(L, f, 0);
}

pub fn lua_isfunction(L: *mut lua_State, n: i32) -> bool {
    lua_type(L, n) == LUA_TFUNCTION
}
pub fn lua_istable(L: *mut lua_State, n: i32) -> bool {
    lua_type(L, n) == LUA_TTABLE
}

pub fn lua_islightuserdata(L: *mut lua_State, n: i32) -> bool {
    lua_type(L, n) == LUA_TLIGHTUSERDATA
}

pub fn lua_isnil(L: *mut lua_State, n: i32) -> bool {
    lua_type(L, n) == LUA_TNIL
}
pub fn lua_isboolean(L: *mut lua_State, n: i32) -> bool {
    lua_type(L, n) == LUA_TBOOLEAN
}
pub fn lua_isthread(L: *mut lua_State, n: i32) -> bool {
    lua_type(L, n) == LUA_TTHREAD
}
pub fn lua_isnone(L: *mut lua_State, n: i32) -> bool {
    lua_type(L, n) == LUA_TNONE
}
pub fn lua_isnoneornil(L: *mut lua_State, n: i32) -> bool {
    lua_type(L, n) <= 0
}

pub fn lua_pushliteral(L: *mut lua_State, s: *const c_char) {
    lua_pushstring(L, s);
}

pub fn lua_pushglobaltable(L: *mut lua_State) {
    lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS);
}
pub fn lua_tostring(L: *mut lua_State, i: i32) -> *const c_char {
    lua_tolstring(L, i, None)
}

pub fn lua_insert(L: *mut lua_State, idx: i32) {
    lua_rotate(L, idx, 1);
}

pub fn lua_remove(L: *mut lua_State, idx: i32) {
    lua_rotate(L, idx, -1);
    lua_pop(L, 1);
}

pub fn lua_replace(L: *mut lua_State, idx: i32) {
    lua_copy(L, -1, idx);
    lua_pop(L, 1);
}

/* }============================================================== */

/*
** {==============================================================
** compatibility macros
** ===============================================================
*/

#[cfg(LUA_COMPAT_APIINTCASTS)]
pub fn lua_pushunsigned(L: *mut lua_State, n: lua_Unsigned) {
    lua_pushinteger(L, n as lua_Integer);
}

#[cfg(LUA_COMPAT_APIINTCASTS)]
pub fn lua_tounsignedx(L: *mut lua_State, i: i32, is: *mut i32) -> lua_Unsigned {
    lua_tointegerx(L, i, is) as lua_Unsigned
}

#[cfg(LUA_COMPAT_APIINTCASTS)]
pub fn lua_tounsigned(L: *mut lua_State, i: i32) -> lua_Unsigned {
    lua_tounsignedx(L, i, None)
}

pub fn lua_newuserdata(L: *mut lua_State, s: size_t) -> *mut c_void {
    lua_newuserdatauv(L, s, 1)
}

pub fn lua_getuservalue(L: *mut lua_State, idx: i32) {
    lua_getiuservalue(L, idx, 1)
}

pub fn lua_setuservalue(L: *mut lua_State, idx: i32) {
    lua_setiuservalue(L, idx, 1)
}
pub const LUA_NUMTAGS: i32 = LUA_NUMTYPES;

/* }============================================================== */

/*
** {======================================================================
** Debug API
** =======================================================================
*/

/*
** Event codes
*/
pub const LUA_HOOKCALL: i32 = 0;
pub const LUA_HOOKRET: i32 = 1;
pub const LUA_HOOKLINE: i32 = 2;
pub const LUA_HOOKCOUNT: i32 = 3;
pub const LUA_HOOKTAILCALL: i32 = 4;

/*
** Event masks
*/
//TODO can bitshifting i32 work the same as c_int?
// #define LUA_MASKCALL	(1 << LUA_HOOKCALL)
// #define LUA_MASKRET	(1 << LUA_HOOKRET)
// #define LUA_MASKLINE	(1 << LUA_HOOKLINE)
// #define LUA_MASKCOUNT	(1 << LUA_HOOKCOUNT)
pub const LUA_MASKCALL: i32 = 1 << LUA_HOOKCALL;
pub const LUA_MASKRET: i32 = 1 << LUA_HOOKRET;
pub const LUA_MASKLINE: i32 = 1 << LUA_HOOKLINE;
pub const LUA_MASKCOUNT: i32 = 1 << LUA_HOOKCOUNT;

/* Functions to be called by the debugger in specific events */
// TODO extern? typedef void (*lua_Hook) (lua_State *L, lua_Debug *ar);
pub type lua_Hook = fn(L: *mut lua_State, ar: *mut lua_Debug);

// LUA_API int (lua_getstack) (lua_State *L, int level, lua_Debug *ar);
// LUA_API int (lua_getinfo) (lua_State *L, const char *what, lua_Debug *ar);
// LUA_API const char *(lua_getlocal) (lua_State *L, const lua_Debug *ar, int n);
// LUA_API const char *(lua_setlocal) (lua_State *L, const lua_Debug *ar, int n);
// LUA_API const char *(lua_getupvalue) (lua_State *L, int funcindex, int n);
// LUA_API const char *(lua_setupvalue) (lua_State *L, int funcindex, int n);

// LUA_API void *(lua_upvalueid) (lua_State *L, int fidx, int n);
// LUA_API void  (lua_upvaluejoin) (lua_State *L, int fidx1, int n1,
//                                                int fidx2, int n2);

// LUA_API void (lua_sethook) (lua_State *L, lua_Hook func, int mask, int count);
// LUA_API lua_Hook (lua_gethook) (lua_State *L);
// LUA_API int (lua_gethookmask) (lua_State *L);
// LUA_API int (lua_gethookcount) (lua_State *L);

// LUA_API int (lua_setcstacklimit) (lua_State *L, unsigned int limit);

pub struct lua_Debug {
    pub event: i32,
    pub name: *const c_char,             /* (n) */
    pub namewhat: *const c_char,         /* (n) 'global', 'local', 'field', 'method' */
    pub what: *const c_char,             /* (S) 'Lua', 'C', 'main', 'tail' */
    pub source: *const c_char,           /* (S) */
    pub srclen: size_t,                  /* (S) */
    pub currentline: i32,                /* (l) */
    pub linedefined: i32,                /* (S) */
    pub lastlinedefined: i32,            /* (S) */
    pub nups: u8,                        /* (u) number of upvalues */
    pub nparams: u8,                     /* (u) number of parameters */
    pub isvararg: c_char,                /* (u) */
    pub istailcall: c_char,              /* (t) */
    pub ftransfer: u16,                  /* (r) index of first value transferred */
    pub ntransfer: u16,                  /* (r) number of transferred values */
    pub short_src: [c_char; LUA_IDSIZE], /* (S) */
    /* private part */
    // TODO struct CallInfo *i_ci;
    pub i_ci: i32, /* active function */
}

/* }====================================================================== */

/******************************************************************************
* Copyright (C) 1994-2022 Lua.org, PUC-Rio.
*
* Permission is hereby granted, free of charge, to any person obtaining
* a copy of this software and associated documentation files (the
* "Software"), to deal in the Software without restriction, including
* without limitation the rights to use, copy, modify, merge, publish,
* distribute, sublicense, and/or sell copies of the Software, and to
* permit persons to whom the Software is furnished to do so, subject to
* the following conditions:
*
* The above copyright notice and this permission notice shall be
* included in all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
* EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
* MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
* IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
* CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
* TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
* SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
******************************************************************************/

// #define lua_c

// #include "lprefix.h"

// #include <stdio.h>
// #include <stdlib.h>
// #include <string.h>

// #include <signal.h>

// #include "lua.h"

// #include "lauxlib.h"
// #include "lualib.h"

use std::os::raw::c_char;

pub const LUA_PROGNAME: &'static str = "lua";

pub const LUA_INIT_VAR: &'static str = "LUA_INIT";

pub const LUA_INITVARVERSION: &'static str = LUA_INIT_VAR + LUA_VERSUFFIX;

pub const globalL: *mut lua_State = 0 as *mut lua_State;

pub const progname: *const char = LUA_PROGNAME as *const char;

// #if defined(LUA_USE_POSIX)   /* { */
/*
** Use 'sigaction' when available.
*/
// TODO we dont need posix but it defaults to signal from signal.h
// #define setsignal            signal

pub fn lstop(L: *mut lua_State, ar: *mut lua_Debug) {
    lua_sethook(L, None, 0, 0);
    luaL_error(L, "interrupted!");
}

/*
** Function to be called at a C signal. Because a C signal cannot
** just change a Lua state (as there is no proper synchronization),
** this function only sets a hook that, when called, will stop the
** interpreter.
*/
pub fn laction(i: i32) {
    let flag = LUA_MASKCALL | LUA_MASKRET | LUA_MASKLINE | LUA_MASKCOUNT;
    setsignal(i, SIG_DFL); /* if another SIGINT happens, terminate process */
    lua_sethook(globalL, lstop, flag, 1);
}

fn print_usage(badoption: &str) {
    lua_writestringerror("%s: ", progname);
    if badoption[1] == 'e' || badoption[1] == 'l' {
        lua_writestringerror("'%s' needs argument\n", badoption);
    } else {
        lua_writestringerror("unrecognized option '%s'\n", badoption);
    }
    lua_writestringerror(
        "usage: %s [options] [script [args]]\n
        Available options are:\n
          -e stat   execute string 'stat'\n
          -i        enter interactive mode after executing 'script'\n
          -l mod    require library 'mod' into global 'mod'\n
          -l g=mod  require library 'mod' into global 'g'\n
          -v        show version information\n
          -E        ignore environment variables\n
          -W        turn warnings on\n
          --        stop handling options\n
          -         stop handling options and execute stdin\n",
        progname,
    );
}

/*
** Prints an error message, adding the program name in front of it
** (if present)
*/
pub fn l_message(pname: &str, msg: &str) {
    if pname != "" {
        lua_writestringerror("%s: ", pname);
    }
    lua_writestringerror("%s\n", msg);
}

/*
** Check whether 'status' is not OK and, if so, prints the error
** message on the top of the stack. It assumes that the error object
** is a string, as it was either generated by Lua or by 'msghandler'.
*/

pub fn report(L: *mut lua_State, status: i32) -> i32 {
    if status != LUA_OK {
        let msg = lua_tostring(L, -1);
        l_message(progname, msg);
        lua_pop(L, 1); /* remove message */
    }
    return status;
}

/*
** Message handler used to run all chunks
*/
pub fn msghandler(L: *mut lua_State) -> i32 {
    let msg = lua_tostring(L, 1);
    if msg.is_null() {
        /* is error object not a string? */
        if luaL_callmeta(L, 1, "__tostring") && lua_type(L, -1) == LUA_TSTRING {
            /* does it have a metamethod */
            /* that produces a string? */
            return 1; /* that is the message */
        } else {
            msg = lua_pushfstring(L, "(error object is a %s value)", luaL_typename(L, 1));
        }
    }
    luaL_traceback(L, L, msg, 1); /* append a standard traceback */
    return 1; /* return the traceback */
}

/*
** Interface to 'lua_pcall', which sets appropriate message function
** and C-signal handler. Used to run all chunks.
*/
pub fn docall(L: *mut lua_State, narg: i32, nres: i32) -> i32 {
    let status: i32;
    let base: i32 = lua_gettop(L) - narg; /* function index */
    lua_pushcfunction(L, msghandler); /* push message handler */
    lua_insert(L, base); /* put it under function and args */
    globalL = L; /* to be available to 'laction' */
    setsignal(SIGINT, laction); /* set C-signal handler */
    status = lua_pcall(L, narg, nres, base);
    setsignal(SIGINT, SIG_DFL); /* reset C-signal handler */
    lua_remove(L, base); /* remove message handler from the stack */
    return status;
}

pub fn print_version() {
    lua_writestring(LUA_COPYRIGHT, strlen(LUA_COPYRIGHT));
    lua_writeline();
}

/*
** Create the 'arg' table, which stores all arguments from the
** command line ('argv'). It should be aligned so that, at index 0,
** it has 'argv[script]', which is the script name. The arguments
** to the script (everything after 'script') go to positive indices;
** other arguments (before the script name) go to negative indices.
** If there is no script name, assume interpreter's name as base.
*/
pub fn createargtable(L: *mut lua_State, argv: *mut *mut c_char, argc: i32, script: i32) {
    let i: i32;
    let narg: i32;
    if script == argc {
        script = 0;
    }
    narg = argc - (script + 1);
    lua_createtable(L, narg, script + 1);
    for i in 0..argc {
        lua_pushstring(L, *argv.offset(i as isize));
        lua_rawseti(L, -2, i - script);
    }
    lua_setglobal(L, "arg");
}

pub fn dochunk(L: *mut lua_State, status: i32) -> i32 {
    if status == LUA_OK {
        status = docall(L, 0, 0);
    }
    return report(L, status);
}

pub fn dofile(L: *mut lua_State, name: &str) -> i32 {
    return dochunk(L, luaL_loadfile(L, name));
}

pub fn dostring(L: *mut lua_State, s: &str, name: &str) -> i32 {
    return dochunk(L, luaL_loadbuffer(L, s, s.len(), name));
}

/*
** Receives 'globname[=modname]' and runs 'globname = require(modname)'.
*/
pub fn dolibrary(L: *mut lua_State, globname: &str) -> i32 {
    let status: i32;
    let mut modname = match globname.find('=') {
        Some(i) => &globname[i + 1..], /* module name starts after the '=' */
        None => globname,              /* module name is equal to global name */
    };
    lua_getglobal(L, "require");
    lua_pushstring(L, modname);
    status = docall(L, 1, 1); /* call 'require(modname)' */
    if status == LUA_OK {
        lua_setglobal(L, globname); /* globname = require(modname) */
    }
    return report(L, status);
}

/*
** Push on the stack the contents of table 'arg' from 1 to #arg
*/
pub fn pushargs(L: *mut lua_State) -> i32 {
    // let mut n: i32;
    if lua_getglobal(L, "arg") != LUA_TTABLE {
        luaL_error(L, "'arg' is not a table");
    }
    let n = luaL_len(L, -1);
    luaL_checkstack(L, n + 3, "too many arguments to script");
    for i in 1..n {
        lua_rawgeti(L, -i, i);
    }
    lua_remove(L, -i); /* remove table from the stack */
    return n;
}
// MARK lua.c here

pub fn handle_script(L: *mut lua_State, argv: *mut *mut c_char) -> i32 {
    let status: i32;
    let fname = *argv; // argv[0];
    if strcmp(fname, "-") == 0 && strcmp(*argv.offset(-1), "--") != 0 {
        fname = NULL; /* stdin */
    }
    status = luaL_loadfile(L, fname);
    if status == LUA_OK {
        let n = pushargs(L); /* push arguments to script */
        status = docall(L, n, LUA_MULTRET);
    }
    return report(L, status);
}

/* bits of various argument indicators in 'args' */
const has_error: i32 = 1; // bad option
const has_i: i32 = 2; // -i
const has_v: i32 = 4; // -v
const has_e: i32 = 8; // -e
const has_E: i32 = 16; // -E

/*
** Traverses all arguments from 'argv', returning a mask with those
** needed before running any Lua code (or an error code if it finds
** any invalid argument). 'first' returns the first not-handled argument
** (either the script name or a bad argument in case of error).
*/
pub fn collectargs(argv: *mut *mut c_char, first: *mut i32) -> i32 {
    let mut args: i32 = 0;
    let mut i: i32 = 1;
    while !(*argv.offset(i)).is_null() {
        *first = i;
        if (*argv.offset(i)).offset(0) != '-' as c_char {
            return args;
        }
        match *(*argv.offset(i)).offset(1) {
            '-' => {
                if *(*argv.offset(i)).offset(2) != '\0' as c_char {
                    return has_error;
                }
                *first = i + 1;
                return args;
            }
            '\0' => {
                return args;
            }
            'E' => {
                if *(*argv.offset(i)).offset(2) != '\0' as c_char {
                    return has_error;
                }
                args |= has_E;
            }
            'W' => {
                if *(*argv.offset(i)).offset(2) != '\0' as c_char {
                    return has_error;
                }
            }
            'i' => {
                args |= has_i;
                args |= has_v;
            }
            'v' => {
                if *(*argv.offset(i)).offset(2) != '\0' as c_char {
                    return has_error;
                }
                args |= has_v;
            }
            'e' => {
                args |= has_e;
            }
            'l' => {
                if *(*argv.offset(i)).offset(2) == '\0' as c_char {
                    i += 1;
                    if (*argv.offset(i)).is_null() || *(*argv.offset(i)).offset(0) == '-' {
                        return has_error;
                    }
                }
            }
            _ => {
                return has_error;
            }
        }
        i += 1;
    }
    *first = i;
    return args;
}

/*
** Processes options 'e' and 'l', which involve running Lua code, and
** 'W', which also affects the state.
** Returns 0 if some code raises an error.
*/
pub fn runargs(L: *mut lua_State, argv: *mut *mut c_char, n: i32) -> i32 {
    let mut i: i32 = 1;
    while i < n {
        let option = *(*argv.offset(i)).offset(1);
        lua_assert(*(*argv.offset(i)).offset(0) == '-' as c_char); // already checked
        match option {
            'e' | 'l' => {
                let mut extra = *(*argv.offset(i)).offset(2); // both options need an argument
                if extra == '\0' as c_char {
                    i += 1;
                    extra = *(*argv.offset(i));
                }
                lua_assert(!extra.is_null());
                let status = if option == 'e' as c_char {
                    dostring(L, extra, "=(command line)")
                } else {
                    dolibrary(L, extra)
                };
                if status != LUA_OK {
                    return 0;
                }
            }
            'W' => {
                lua_warning(L, "@on", 0); // warnings on
            }
            _ => {}
        }
        i += 1;
    }
    return 1;
}

pub fn handle_luainit(L: *mut lua_State) -> i32 {
    let name = "=" + LUA_INITVARVERSION;
    let init = getenv(name.offset(1)); // DEV why not just LUA_INITVARVERSION
    if init.is_null() {
        name = "=" + LUA_INIT_VAR;
        init = getenv(name.offset(1)); /* try alternative name */
    }
    if init.is_null() {
        return LUA_OK;
    } else if *init.offset(0) == '@' as c_char {
        return dofile(L, init.offset(1));
    } else {
        return dostring(L, init, name);
    }
}

/*
** {==================================================================
** Read-Eval-Print Loop (REPL)
** ===================================================================
*/

pub const LUA_PROMPT: str = "> ";
pub const LUA_PROMPT2: str = ">> ";

pub const LUA_MAXINPUT: usize = 512;

/*
** lua_stdin_is_tty detects whether the standard input is a 'tty' (that
** is, whether we're running lua interactively).
*/

// TODO
// #if defined(LUA_USE_POSIX)	/* { */
// #include <unistd.h>
// #define lua_stdin_is_tty()	isatty(0)

// #elif defined(LUA_USE_WINDOWS)	/* }{ */
// #include <io.h>
// #include <windows.h>

// #define lua_stdin_is_tty()	_isatty(_fileno(stdin))

// #else				/* }{ */
// /* ISO C definition */
// #define lua_stdin_is_tty()	1  /* assume stdin is a tty */
// #endif				/* } */
// #endif				/* } */
/*
** lua_readline defines how to show a prompt and then read a line from
** the standard input.
** lua_saveline defines how to "save" a read line in a "history".
** lua_freeline defines how to free a line read by lua_readline.
*/
#[cfg(LUA_USE_READLINE)]
pub fn lua_initreadline(L: *mut lua_State) {
    rl_readline_name = "lua";
}
#[cfg(LUA_USE_READLINE)]
pub fn lua_readline(L: *mut lua_State, b: *mut c_char, p: *const c_char) -> i32 {
    let mut b = readline(p);
    if b.is_null() {
        return 0;
    } else {
        return 1;
    }
}
#[cfg(LUA_USE_READLINE)]
pub fn lua_saveline(L: *mut lua_State, line: *mut c_char) {
    add_history(line);
}
#[cfg(LUA_USE_READLINE)]
pub fn lua_freeline(L: *mut lua_State, b: *mut c_char) {
    free(b);
}

#[cfg(not(LUA_USE_READLINE))]
pub fn lua_initreadline(L: *mut lua_State) {}

// TODO #define lua_readline(L,b,p) \
//         ((void)L, fputs(p, stdout), fflush(stdout),  /* show prompt */ \
//         fgets(b, LUA_MAXINPUT, stdin) != NULL)  /* get line */
#[cfg(not(LUA_USE_READLINE))]
pub fn lua_readline(L: *mut lua_State, b: *mut c_char, p: *const c_char) -> i32 {
    let mut b = readline(p);
    if b.is_null() {
        return 0;
    } else {
        return 1;
    }
}

#[cfg(not(LUA_USE_READLINE))]
pub fn lua_saveline(L: *mut lua_State, line: *mut c_char) {}
#[cfg(not(LUA_USE_READLINE))]
pub fn lua_freeline(L: *mut lua_State, b: *mut c_char) {}

/*
** Return the string to be used as a prompt by the interpreter. Leave
** the string (or nil, if using the default value) on the stack, to keep
** it anchored.
*/
pub fn get_prompt(L: *mut lua_State, firstline: i32) -> *const c_char {
    if lua_getglobal(L, if firstline { "_PROMPT" } else { "_PROMPT2" }) == LUA_TNIL {
        return (if firstline { LUA_PROMPT } else { LUA_PROMPT2 }); /* use the default */
    } else {
        /* apply 'tostring' over the value */
        let p = luaL_tolstring(L, -1, NULL);
        lua_remove(L, -2); /* remove original value */
        return p;
    }
}

/* mark in error messages for incomplete statements */
//TODO #define EOFMARK		"<eof>"
pub const EOFMARK: [c_char; 6] = [b'<', b'e', b'o', b'f', b'>', b'\0'];

// #define marklen		(sizeof(EOFMARK)/sizeof(char) - 1)
pub fn marklen() -> usize {
    return EOFMARK.len() - 1;
}

/*
** Check whether 'status' signals a syntax error and the error
** message at the top of the stack ends with the above mark for
** incomplete statements.
*/
pub fn incomplete(L: *mut lua_State, status: i32) -> i32 {
    if status == LUA_ERRSYNTAX {
        let lmsg: size_t = 0;
        let msg = lua_tolstring(L, -1, &lmsg);
        if lmsg >= marklen() && strcmp(msg + lmsg - marklen(), EOFMARK) == 0 {
            lua_pop(L, 1);
            return 1;
        }
    }
    return 0; /* else... */
}

/*
** Prompt the user, read a line, and push it into the Lua stack.
*/
pub fn pushline(L: *mut lua_State, firstline: i32) -> i32 {
    let mut buffer: [c_char; LUA_MAXINPUT] = [0; LUA_MAXINPUT];
    let mut b = buffer;
    let mut l: size_t = 0;
    let prmt = get_prompt(L, firstline);
    let readstatus = lua_readline(L, b, prmt);
    if readstatus == 0 {
        return 0; /* no input (prompt will be popped by caller) */
    }
    lua_pop(L, 1); /* remove prompt */
    l = strlen(b);
    if l > 0 && b[l - 1] == '\n' {
        /* line ends with newline? */
        b[l - 1] = '\0'; /* remove it */
    }
    if firstline && b[0] == '=' {
        /* for compatibility with 5.2, ... */
        lua_pushfstring(L, "return %s", b + 1); /* change '=' to 'return' */
    } else {
        lua_pushlstring(L, b, l);
    }
    lua_freeline(L, b);
    return 1;
}

/*
** Try to compile line on the stack as 'return <line>;'; on return, stack
** has either compiled chunk or original line (if compilation failed).
*/
pub fn addreturn(L: *mut lua_State) -> i32 {
    let line = lua_tostring(L, -1); /* original line */
    let retline = lua_pushfstring(L, "return %s;", line);
    let status = luaL_loadbuffer(L, retline, strlen(retline), "=stdin");
    if status == LUA_OK {
        lua_remove(L, -2); /* remove modified line */
        if line[0] != '\0' {
            /* non empty? */
            lua_saveline(L, line); /* keep history */
        }
    } else {
        lua_pop(L, 2); /* pop result from 'luaL_loadbuffer' and modified line */
    }
    return status;
}

/*
** Read multiple lines until a complete Lua statement
*/
pub fn multiline(L: *mut lua_State) -> i32 {
    loop {
        let mut len: size_t = 0;
        let line = lua_tolstring(L, 1, &mut len); /* get what it has */
        let status = luaL_loadbuffer(L, line, len, "=stdin"); /* try it */
        if incomplete(L, status) == 0 || pushline(L, 0) == 0 {
            lua_saveline(L, line); /* keep history */
            return status; /* cannot or should not try to add continuation line */
        }
        lua_pushliteral(L, "\n"); /* add newline... */
        lua_insert(L, -2); /* ...between the two lines */
        lua_concat(L, 3); /* join them */
    }
}

/*
** Read a line and try to load (compile) it first as an expression (by
** adding "return " in front of it) and second as a statement. Return
** the final status of load/call with the resulting function (if any)
** in the top of the stack.
*/
pub fn loadline(L: *mut lua_State) -> i32 {
    let mut status: i32 = 0;
    lua_settop(L, 0);
    if pushline(L, 1) == 0 {
        return -1; /* no input */
    }
    status = addreturn(L);
    if status != LUA_OK {
        /* 'return ...' did not work? */
        status = multiline(L); /* try as command, maybe with continuation lines */
    }
    lua_remove(L, 1); /* remove line from the stack */
    lua_assert(lua_gettop(L) == 1);
    return status;
}

/*
** Prints (calling the Lua 'print' function) any values on the stack
*/
pub fn l_print(L: *mut lua_State) {
    let n = lua_gettop(L);
    if n > 0 {
        /* any result to be printed? */
        luaL_checkstack(L, LUA_MINSTACK, "too many results to print");
        lua_getglobal(L, "print");
        lua_insert(L, 1);
        if lua_pcall(L, n, 0, 0) != LUA_OK {
            l_message(
                progname,
                lua_pushfstring(L, "error calling 'print' (%s)", lua_tostring(L, -1)),
            );
        }
    }
}

/*
** Do the REPL: repeatedly read (load) a line, evaluate (call) it, and
** print any results.
*/
pub fn doREPL(L: *mut lua_State) {
    let mut status: i32 = 0;
    let oldprogname = progname; // TODO  const char *oldprogname = progname;
    progname = NULL; /* no 'progname' on errors in interactive mode */
    lua_initreadline(L);
    while {
        status = loadline(L);
        status != -1
    } {
        if status == LUA_OK {
            status = docall(L, 0, LUA_MULTRET);
        }
        if status == LUA_OK {
            l_print(L);
        } else {
            report(L, status);
        }
    }
    lua_settop(L, 0); /* clear stack */
    lua_writeline();
    progname = oldprogname;
}

/* }================================================================== */

/*
** Main body of stand-alone interpreter (to be called in protected mode).
** Reads the options and handles them all.
*/
pub fn pmain(L: *mut lua_State) -> i32 {
    let argc = lua_tointeger(L, 1) as i32;
    let argv = lua_touserdata(L, 2) as *mut *mut c_char;
    let mut script: i32 = 0;
    let args = collectargs(argv, &mut script);
    luaL_checkversion(L); /* check that interpreter has correct version */
    if !(*argv).is_null() && **argv != 0 {
        progname = *argv;
    }
    if args == has_error {
        /* bad arg? */
        print_usage(*argv.offset(script as isize)); /* 'script' has index of bad arg. */
        return 0;
    }
    if args & has_v != 0 {
        /* option '-v'? */
        print_version();
    }
    if args & has_E != 0 {
        /* option '-E'? */
        lua_pushboolean(L, 1); /* signal for libraries to ignore env. vars. */
        lua_setfield(L, LUA_REGISTRYINDEX, "LUA_NOENV");
    }
    luaL_openlibs(L); /* open standard libraries */
    createargtable(L, argv, argc, script); /* create table 'arg' */
    lua_gc(L, LUA_GCGEN, 0, 0); /* GC in generational mode */
    if args & has_E == 0 {
        /* no option '-E'? */
        if handle_luainit(L) != LUA_OK {
            /* run LUA_INIT */
            return 0; /* error running LUA_INIT */
        }
    }
    if !runargs(L, argv, script) {
        /* execute arguments -e and -l */
        return 0; /* something failed */
    }
    if script < argc && /* execute main script (if there is one) */
    handle_script(L, argv.offset(script as isize)) != LUA_OK
    {
        return 0;
    }
    if args & has_i != 0 {
        /* -i option? */
        doREPL(L); /* do read-eval-print loop */
    } else if script == argc && (args & (has_e | has_v) == 0) {
        /* no arguments? */
        if lua_stdin_is_tty() {
            /* running in interactive mode? */
            print_version();
            doREPL(L); /* do read-eval-print loop */
        } else {
            dofile(L, ptr::null()); /* executes stdin as a file */
        }
    }
    lua_pushboolean(L, 1); /* signal no errors */
    return 1;
}

pub fn main_main() {
    let mut status: i32 = 0;
    let mut result: i32 = 0;
    let L = luaL_newstate(); /* create state */
    if L.is_null() {
        l_message(*argv.offset(0), "cannot create state: not enough memory");
        return EXIT_FAILURE;
    }
    lua_pushcfunction(L, pmain); /* to call 'pmain' in protected mode */
    lua_pushinteger(L, argc); /* 1st argument */
    lua_pushlightuserdata(L, argv as *mut c_void); /* 2nd argument */
    status = lua_pcall(L, 2, 1, 0); /* do the call */
    result = lua_toboolean(L, -1); /* get result */
    report(L, status);
    lua_close(L);
    // TODO return (result && status == LUA_OK) ? EXIT_SUCCESS : EXIT_FAILURE;
    return (result != 0 && status == LUA_OK) as i32;
}
