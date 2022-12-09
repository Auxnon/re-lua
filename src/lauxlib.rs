/*
** $Id: lauxlib.h $
** Auxiliary functions for building Lua libraries
** See Copyright Notice in lua.h
*/
use std::mem::size_of;

/* global table */
pub const LUA_GNAME: &'static str = "_G";

pub struct luaL_Buffer {
    b: *mut c_char,
    size: size_t,
    n: size_t,
    L: *mut lua_State,
    initb: [lua_char; LUAL_BUFFERSIZE],
}


/* extra error code for 'luaL_loadfilex' */
pub const LUA_ERRFILE: i32 = LUA_ERRERR + 1;    


/* key, in the registry, for table of loaded modules */
pub const LUA_LOADED_TABLE: &'static str = "_LOADED";


/* key, in the registry, for table of preloaded loaders */
pub const LUA_PRELOAD_TABLE: &'static str = "_PRELOAD";

pub struct luaL_Reg {
    name: *const lua_char,
    func: lua_CFunction,
}

pub const LUAL_NUMSIZES: usize = (size_of::<lua_Integer>() * 16) + size_of::<lua_Number>();


// LUALIB_API void (luaL_checkversion_) (lua_State *L, lua_Number ver, size_t sz);
// #define luaL_checkversion(L)  \
// 	  luaL_checkversion_(L, LUA_VERSION_NUM, LUAL_NUMSIZES)
// TODO jsut make it an OPTION
pub fn luaL_checkversion(L: *mut lua_State) {
    luaL_checkversion_(L, LUA_VERSION_NUM, LUAL_NUMSIZES);
}


// LUALIB_API int (luaL_getmetafield) (lua_State *L, int obj, const char *e);
// LUALIB_API int (luaL_callmeta) (lua_State *L, int obj, const char *e);
// LUALIB_API const char *(luaL_tolstring) (lua_State *L, int idx, size_t *len);
// LUALIB_API int (luaL_argerror) (lua_State *L, int arg, const char *extramsg);
// LUALIB_API int (luaL_typeerror) (lua_State *L, int arg, const char *tname);
// LUALIB_API const char *(luaL_checklstring) (lua_State *L, int arg,
//                                                           size_t *l);
// LUALIB_API const char *(luaL_optlstring) (lua_State *L, int arg,
//                                           const char *def, size_t *l);
// LUALIB_API lua_Number (luaL_checknumber) (lua_State *L, int arg);
// LUALIB_API lua_Number (luaL_optnumber) (lua_State *L, int arg, lua_Number def);

// LUALIB_API lua_Integer (luaL_checkinteger) (lua_State *L, int arg);
// LUALIB_API lua_Integer (luaL_optinteger) (lua_State *L, int arg,
//                                           lua_Integer def);

// LUALIB_API void (luaL_checkstack) (lua_State *L, int sz, const char *msg);
// LUALIB_API void (luaL_checktype) (lua_State *L, int arg, int t);
// LUALIB_API void (luaL_checkany) (lua_State *L, int arg);

// LUALIB_API int   (luaL_newmetatable) (lua_State *L, const char *tname);
// LUALIB_API void  (luaL_setmetatable) (lua_State *L, const char *tname);
// LUALIB_API void *(luaL_testudata) (lua_State *L, int ud, const char *tname);
// LUALIB_API void *(luaL_checkudata) (lua_State *L, int ud, const char *tname);

// LUALIB_API void (luaL_where) (lua_State *L, int lvl);
// LUALIB_API int (luaL_error) (lua_State *L, const char *fmt, ...);

// LUALIB_API int (luaL_checkoption) (lua_State *L, int arg, const char *def,
//                                    const char *const lst[]);

// LUALIB_API int (luaL_fileresult) (lua_State *L, int stat, const char *fname);
// LUALIB_API int (luaL_execresult) (lua_State *L, int stat);


/* predefined references */
pub const LUA_NOREF: i32 = -2;
pub const LUA_REFNIL: i32 = -1;

// LUALIB_API int (luaL_ref) (lua_State *L, int t);
// LUALIB_API void (luaL_unref) (lua_State *L, int t, int ref);

// LUALIB_API int (luaL_loadfilex) (lua_State *L, const char *filename,
//                                                const char *mode);
// TODO jsut make it an OPTION
// #define luaL_loadfile(L,f)	luaL_loadfilex(L,f,NULL)

// LUALIB_API int (luaL_loadbufferx) (lua_State *L, const char *buff, size_t sz,
//                                    const char *name, const char *mode);
// LUALIB_API int (luaL_loadstring) (lua_State *L, const char *s);

// LUALIB_API lua_State *(luaL_newstate) (void);

// LUALIB_API lua_Integer (luaL_len) (lua_State *L, int idx);

// LUALIB_API void (luaL_addgsub) (luaL_Buffer *b, const char *s,
//                                      const char *p, const char *r);
// LUALIB_API const char *(luaL_gsub) (lua_State *L, const char *s,
//                                     const char *p, const char *r);

// LUALIB_API void (luaL_setfuncs) (lua_State *L, const luaL_Reg *l, int nup);

// LUALIB_API int (luaL_getsubtable) (lua_State *L, int idx, const char *fname);

// LUALIB_API void (luaL_traceback) (lua_State *L, lua_State *L1,
//                                   const char *msg, int level);

// LUALIB_API void (luaL_requiref) (lua_State *L, const char *modname,
//                                  lua_CFunction openf, int glb);

/*
** ===============================================================
** some useful macros
** ===============================================================
*/

pub fn luaL_newlibtable(L: *mut lua_State, l: *const luaL_Reg) {
    lua_createtable(L, 0, (sizeof(l) / sizeof((l)[0])) - 1);
}


    pub fn luaL_newlib(L: *mut lua_State, l: *const luaL_Reg) {
        luaL_checkversion(L);
        luaL_newlibtable(L, l);
        luaL_setfuncs(L, l, 0);
    }


    pub fn luaL_argcheck(L: *mut lua_State, cond: i32, arg: i32, extramsg: *const c_char) {
        // DEV do we not need luai_likely ?
        if !luai_likely(cond) {
            luaL_argerror(L, arg, extramsg);
        }
    }   


pub fn luaL_argexpected(L: *mut lua_State, cond: i32, arg: i32, tname: *const c_char) {
    if !luai_likely(cond) {
        luaL_typeerror(L, arg, tname);
    }
}
    

pub fn luaL_checkstring(L: *mut lua_State, arg: i32) -> *const c_char {
    luaL_checklstring(L, arg, ptr::null_mut())
}

// TODO luaL_optstring, jsut make this an OPTION param
// #define luaL_optstring(L,n,d)	(luaL_optlstring(L, (n), (d), NULL))

// TODO luaL_typename
// #define luaL_typename(L,i)	lua_typename(L, lua_type(L,(i)))


// TODO check 
// #define luaL_dofile(L, fn) \
// 	(luaL_loadfile(L, fn) || lua_pcall(L, 0, LUA_MULTRET, 0))

    pub fn luaL_dofile(L: *mut lua_State, func: *const c_char) -> i32 {
        if luaL_loadfile(L, func) != 0 {
            return 1;
        }
        if lua_pcall(L, 0, LUA_MULTRET, 0) != 0 {
            return 1;
        }
        return 0;
    }   

//TODO #define luaL_dostring(L, s) \
// 	(luaL_loadstring(L, s) || lua_pcall(L, 0, LUA_MULTRET, 0))


//TODO #define luaL_getmetatable(L,n)	(lua_getfield(L, LUA_REGISTRYINDEX, (n)))

//TODO #define luaL_opt(L,f,n,d)	(lua_isnoneornil(L,(n)) ? (d) : f(L,(n)))

//TODO #define luaL_loadbuffer(L,s,sz,n)	luaL_loadbufferx(L,s,sz,n,NULL)


/*
** Perform arithmetic operations on lua_Integer values with wrap-around
** semantics, as the Lua core does.
*/
// #define luaL_intop(op,v1,v2)  \
// 	((lua_Integer)((lua_Unsigned)(v1) op (lua_Unsigned)(v2)))
    pub fn luaL_intop(v1: lua_Integer, v2: lua_Integer) -> lua_Integer {
        //DEV supposed to pass operator as a param, only see + used so we'll cheat
        (v1 as lua_Unsigned) + (v2 as lua_Unsigned) as lua_Integer
    }


/* push the value used to represent failure/error */
pub fn luaL_pushfail(L: *mut lua_State) {
    lua_pushnil(L);
}


/*
** Internal assertions for in-house debugging
*/
// DEV maybe uneeded?
// #if !defined(lua_assert)

// #if defined LUAI_ASSERT
//   #include <assert.h>
//   #define lua_assert(c)		assert(c)
// #else
//   #define lua_assert(c)		((void)0)
// #endif

// #endif



/*
** {======================================================
** Generic Buffer manipulation
** =======================================================
*/
// TODO are we building this luaL_Buffer per the below commented code from elsewhere?

/*
@@ LUAL_BUFFERSIZE is the buffer size used by the lauxlib buffer system.
*/
// #define LUAL_BUFFERSIZE   ((int)(16 * sizeof(void*) * sizeof(lua_Number)))

// /*
// @@ LUAI_MAXALIGN defines fields that, when used in a union, ensure
// ** maximum alignment for the other items in that union.
// */
// #define LUAI_MAXALIGN  lua_Number n; double u; void *s; lua_Integer i; long l

// struct luaL_Buffer {
//   char *b;  /* buffer address */
//   size_t size;  /* buffer size */
//   size_t n;  /* number of characters in buffer */
//   lua_State *L;
//   union {
//     LUAI_MAXALIGN;  /* ensure maximum alignment for buffer */
//     char b[LUAL_BUFFERSIZE];  /* initial buffer */
//   } init;
// };

// LUALIB_API void luaL_buffinit (lua_State *L, luaL_Buffer *B) {
//     B->L = L;
//     B->b = B->init.b;
//     B->n = 0;
//     B->size = LUAL_BUFFERSIZE;
//     lua_pushlightuserdata(L, (void*)B);  /* push placeholder */
//   }
pub struct luaL_Buffer {
    b: *mut c_char,
    size: size_t,
    n: size_t,
    L: *mut lua_State,
    init: [u8; LUAL_BUFFERSIZE],
}

pub fn luaL_bufflen(bf: *mut luaL_Buffer) -> size_t {
    bf.n
}

pub fn luaL_buffaddr(bf: *mut luaL_Buffer) -> *mut c_char {
    bf.b
}

    pub fn luaL_addchar(bf: *mut luaL_Buffer, c: c_char) {
         if bf.n < bf.size {
              luaL_prepbuffsize(bf, 1);
         }
         bf.b[bf.n] = c;
         bf.n += 1;
    }

pub fn luaL_addsize(bf: *mut luaL_Buffer, s: size_t) {
    bf.n += s;
}


pub fn luaL_buffsub(bf: *mut luaL_Buffer, s: size_t) {
    bf.n -= s;
}

// LUALIB_API void (luaL_buffinit) (lua_State *L, luaL_Buffer *B);
// LUALIB_API char *(luaL_prepbuffsize) (luaL_Buffer *B, size_t sz);
// LUALIB_API void (luaL_addlstring) (luaL_Buffer *B, const char *s, size_t l);
// LUALIB_API void (luaL_addstring) (luaL_Buffer *B, const char *s);
// LUALIB_API void (luaL_addvalue) (luaL_Buffer *B);
// LUALIB_API void (luaL_pushresult) (luaL_Buffer *B);
// LUALIB_API void (luaL_pushresultsize) (luaL_Buffer *B, size_t sz);
// LUALIB_API char *(luaL_buffinitsize) (lua_State *L, luaL_Buffer *B, size_t sz);

pub fn luaL_prepbuffer(bf: *mut luaL_Buffer) -> *mut c_char {
    luaL_prepbuffsize(bf, LUAL_BUFFERSIZE)
}

/* }====================================================== */



/*
** {======================================================
** File handles for IO library
** =======================================================
*/

/*
** A file handle is a userdata with metatable 'LUA_FILEHANDLE' and
** initial structure 'luaL_Stream' (it may contain other fields
** after that initial structure).
*/

pub const LUA_FILEHANDLE: &'static str = "FILE*";

// convert above to rust
pub struct luaL_Stream {
    f: *mut FILE, /* stream (NULL for incompletely created streams) */
    closef: lua_CFunction, /* to close stream (NULL for closed streams) */
}

/* }====================================================== */

/*
** {==================================================================
** "Abstraction Layer" for basic report of messages and errors
** ===================================================================
*/

/* print a string */
pub fn lua_writestring(s: *mut c_char, l: size_t) {
    //TODO fwrite((s), sizeof(char), (l), stdout)
    fwrite(s, 1, l, stdout);
}

/* print a newline and flush the output */
pub fn lua_writeline() {
    lua_writestring("\n", 1);
    fflush(stdout);
}

/* print an error message */
pub fn lua_writestringerror(s: *mut c_char, p: *mut c_char) {
    fprintf(stderr, s, p);
    fflush(stderr);
}

/* }================================================================== */


/*
** {============================================================
** Compatibility with deprecated conversions
** =============================================================
*/
#[cfg(LUA_COMPAT_APIINTCASTS)]
pub fn luaL_checkunsigned(L: *mut lua_State, a: c_int) -> lua_Unsigned {
    luaL_checkinteger(L, a) as lua_Unsigned
}

#[cfg(LUA_COMPAT_APIINTCASTS)]
pub fn luaL_optunsigned(L: *mut lua_State, a: c_int, d: lua_Integer) -> lua_Unsigned {
    luaL_optinteger(L, a, d) as lua_Unsigned
}

#[cfg(LUA_COMPAT_APIINTCASTS)]
pub fn luaL_checkint(L: *mut lua_State, n: c_int) -> c_int {
    luaL_checkinteger(L, n) as c_int
}

#[cfg(LUA_COMPAT_APIINTCASTS)]
pub fn luaL_optint(L: *mut lua_State, n: c_int, d: lua_Integer) -> c_int {
    luaL_optinteger(L, n, d) as c_int
}

#[cfg(LUA_COMPAT_APIINTCASTS)]
pub fn luaL_checklong(L: *mut lua_State, n: c_int) -> c_long {
    luaL_checkinteger(L, n) as c_long
}

#[cfg(LUA_COMPAT_APIINTCASTS)]
pub fn luaL_optlong(L: *mut lua_State, n: c_int, d: lua_Integer) -> c_long {
    luaL_optinteger(L, n, d) as c_long
}
/* }============================================================ */



// YELLOW C

/*
** $Id: lauxlib.c $
** Auxiliary functions for building Lua libraries
** See Copyright Notice in lua.h
*/

// #include "lprefix.h"


// #include <errno.h>
// #include <stdarg.h>
// #include <stdio.h>
// #include <stdlib.h>
// #include <string.h>


/*
** This file uses only the official API of Lua.
** Any function declared here could be written as an application function.
*/

// #include "lua.h"

// #include "lauxlib.h"


/* maximum value for size_t */
pub const MAX_SIZET: size_t = !0;

/*
** {======================================================
** Traceback
** =======================================================
*/


pub const LEVELS1: c_int = 10; /* size of the first part of the stack */
pub const LEVELS2: c_int = 11; /* size of the second part of the stack */


/*
** Search for 'objidx' in table at index -1. ('objidx' must be an
** absolute index.) Return 1 + string at top if it found a good name.
*/
fn findfield(L: *mut lua_State, objidx: c_int, level: c_int) -> c_int {
    if level == 0 || !lua_istable(L, -1) {
        return 0; /* not found */
    }
    lua_pushnil(L); /* start 'next' loop */
    while lua_next(L, -2) { /* for each pair in table */
        if lua_type(L, -2) == LUA_TSTRING { /* ignore non-string keys */
            if lua_rawequal(L, objidx, -1) { /* found object? */    
                lua_pop(L, 1); /* remove value (but keep name) */
                return 1; 
            } else if findfield(L, objidx, level - 1) { /* try recursively */
                /* stack: lib_name, lib_table, field_name (top) */
                lua_pushliteral(L, "."); /* place '.' between the two names */
                lua_replace(L, -3); /* (in the slot occupied by table) */
                lua_concat(L, 3); /* lib_name.field_name */
                return 1;
            }
        }
        lua_pop(L, 1); /* remove value */
    }
    return 0; /* not found */
}   


/*
** Search for a name for a function in all loaded modules
*/
pub fn pushglobalfuncname(L: *mut lua_State, ar: *mut lua_Debug) -> c_int {
    let top = lua_gettop(L);
    lua_getinfo(L, "f", ar);  /* push function */
    lua_getfield(L, LUA_REGISTRYINDEX, LUA_LOADED_TABLE);
    if findfield(L, top + 1, 2) != 0 {
        let name = lua_tostring(L, -1);
        if strncmp(name, LUA_GNAME ".", 3) == 0 {  /* name start with '_G.'? */
            lua_pushstring(L, name + 3);  /* push name without prefix */
            lua_remove(L, -2);  /* remove original name */
        }
        lua_copy(L, -1, top + 1);  /* copy name to proper place */
        lua_settop(L, top + 1);  /* remove table "loaded" and name copy */
        return 1;
    }
    else {
        lua_settop(L, top);  /* remove function and global table */
        return 0;
    }
}


    fn pushfuncname(L: *mut lua_State, ar: *mut lua_Debug) {
        if pushglobalfuncname(L, ar) != 0 { /* try first a global name */
            lua_pushfstring(L, "function '%s'", lua_tostring(L, -1));
            lua_remove(L, -2); /* remove name */
        } else if *ar.namewhat != 0 { /* is there a name from code? */
            lua_pushfstring(L, "%s '%s'", ar.namewhat, ar.name); /* use it */
        } else if *ar.what == 'm' { /* main? */
            lua_pushliteral(L, "main chunk");
        } else if *ar.what != 'C' { /* for Lua functions, use <file:line> */
            lua_pushfstring(L, "function <%s:%d>", ar.short_src, ar.linedefined);
        } else { /* nothing left... */
            lua_pushliteral(L, "?");
        }
    }   

    

      
        fn lastlevel(L: *mut lua_State) -> c_int {
            // TODO smaller debug init
            let mut ar: lua_Debug = lua_Debug {
                event: 0,
                name: ptr::null(),
                namewhat: ptr::null(),
                what: ptr::null(),
                source: ptr::null(),
                currentline: 0,
                nups: 0,
                linedefined: 0,
                lastlinedefined: 0,
                short_src: [0; LUA_IDSIZE],
                i_ci: 0,
            };
            let mut li = 1;
            let mut le = 1;
            /* find an upper bound */
            while lua_getstack(L, le, &mut ar) != 0 {
                li = le;
                le *= 2;
            }
            /* do a binary search */
            while li < le {
                let m = (li + le) / 2;
                if lua_getstack(L, m, &mut ar) != 0 {
                    li = m + 1;
                } else {
                    le = m;
                }
            }
            le - 1  
        }
      

        pub fn luaL_traceback(L: *mut lua_State, L1: *mut lua_State, msg: *const c_char, level: c_int) {
            // TODO smaller debug init
            let mut ar: lua_Debug = lua_Debug {
                event: 0,
                name: ptr::null(),
                namewhat: ptr::null(),
                what: ptr::null(),
                source: ptr::null(),
                currentline: 0,
                nups: 0,
                linedefined: 0,
                lastlinedefined: 0,
                short_src: [0; LUA_IDSIZE],
                i_ci: 0,
            };
            let mut b = luaL_Buffer {
                p: ptr::null_mut(),
                lvl: 0,
                L: ptr::null_mut(),
                buffer: [0; LUAL_BUFFERSIZE],
            };
            let last = lastlevel(L1);
            let mut limit2show = if last - level > LEVELS1 + LEVELS2 { LEVELS1 } else { -1 };
            luaL_buffinit(L, &mut b);
            if !msg.is_null() {
                luaL_addstring(&mut b, msg);
                luaL_addchar(&mut b, '\n' as c_int);
            }
            luaL_addstring(&mut b, "stack traceback:");
            while lua_getstack(L1, level, &mut ar) != 0 {
                level += 1;
                if limit2show == 0 { /* too many levels? */
                    let n = last - level - LEVELS2 + 1; /* number of levels to skip */
                    lua_pushfstring(L, "\n\t...\t(skipping %d levels)", n); 
                    luaL_addvalue(&mut b); /* add warning about skip */
                    level += n; /* and skip to last levels */
                } else {
                    lua_getinfo(L1, "Slnt", &mut ar);
                    if ar.currentline <= 0 {
                        lua_pushfstring(L, "\n\t%s: in ", ar.short_src);
                    } else {
                        lua_pushfstring(L, "\n\t%s:%d: in ", ar.short_src, ar.currentline);
                    }
                    luaL_addvalue(&mut b);
                    pushfuncname(L, &mut ar);
                    luaL_addvalue(&mut b);
                    if ar.istailcall != 0 {
                        luaL_addstring(&mut b, "\n\t(...tail calls...)");
                    }
                }
            }
            luaL_pushresult(&mut b);
        }
      
      /* }====================================================== */
      
      
      /*
      ** {======================================================
      ** Error-report functions
      ** =======================================================
      */

      
        pub fn luaL_argerror(L: *mut lua_State, arg: c_int, extramsg: *const c_char) -> c_int {
            // TODO smaller debug init  
            let mut ar: lua_Debug = lua_Debug {
                event: 0,
                name: ptr::null(),
                namewhat: ptr::null(),
                what: ptr::null(),
                source: ptr::null(),
                currentline: 0,
                nups: 0,
                linedefined: 0,
                lastlinedefined: 0,
                short_src: [0; LUA_IDSIZE],
                i_ci: 0,
            };
            //TODO if (!lua_getstack(L, 0, &ar)) 
            if lua_getstack(L, 0, &mut ar) == 0 { /* no stack frame? */
                return luaL_error(L, "bad argument #%d (%s)", arg, extramsg);
            }
            lua_getinfo(L, "n", &mut ar);
            if strcmp(ar.namewhat, "method") == 0 {
                arg -= 1; /* do not count 'self' */
                if arg == 0 { /* error is in the self argument itself? */
                    return luaL_error(L, "calling '%s' on bad self (%s)", ar.name, extramsg);
                }
            }
            if ar.name.is_null() {
                ar.name = if pushglobalfuncname(L, &mut ar) != 0 {
                    lua_tostring(L, -1)
                } else {
                   "?"
                };
            }
            luaL_error(L, "bad argument #%d to '%s' (%s)", arg, ar.name, extramsg);
        }

        LUALIB_API int luaL_typeerror (lua_State *L, int arg, const char *tname) {
            const char *msg;
            const char *typearg;  /* name for the type of the actual argument */
            if (luaL_getmetafield(L, arg, "__name") == LUA_TSTRING)
              typearg = lua_tostring(L, -1);  /* use the given type name */
            else if (lua_type(L, arg) == LUA_TLIGHTUSERDATA)
              typearg = "light userdata";  /* special name for messages */
            else
              typearg = luaL_typename(L, arg);  /* standard name */
            msg = lua_pushfstring(L, "%s expected, got %s", tname, typearg);
            return luaL_argerror(L, arg, msg);
          }
    // convert above to rust
    pub fn luaL_typeerror(L: *mut lua_State, arg: c_int, tname: *const c_char) -> c_int {
        let mut msg: *const c_char = ptr::null();
        let mut typearg: *const c_char = ptr::null(); /* name for the type of the actual argument */
        if luaL_getmetafield(L, arg, "__name") == LUA_TSTRING {
            typearg = lua_tostring(L, -1); /* use the given type name */
        } else if lua_type(L, arg) == LUA_TLIGHTUSERDATA {
            typearg = "light userdata"; /* special name for messages */
        } else {
            typearg = luaL_typename(L, arg); /* standard name */
        }
        msg = lua_pushfstring(L, "%s expected, got %s", tname, typearg);
        luaL_argerror(L, arg, msg)
    }


        pub fn tag_error(L: *mut lua_State, arg: c_int, tag: c_int) {
            luaL_typeerror(L, arg, lua_typename(L, tag));
        }

      
        /*
** The use of 'lua_pushfstring' ensures this function does not
** need reserved stack space when called.
*/
        pub fn luaL_where(L: *mut lua_State, lvl: c_int) {
            // TODO smaller debug init
            let mut ar: lua_Debug = lua_Debug {
                event: 0,
                name: ptr::null(),
                namewhat: ptr::null(),
                what: ptr::null(),
                source: ptr::null(),
                currentline: 0,
                nups: 0,
                linedefined: 0,
                lastlinedefined: 0,
                short_src: [0; LUA_IDSIZE],
                i_ci: 0,
            };
            if lua_getstack(L, lvl, &mut ar) != 0 { /* check function at level */
                lua_getinfo(L, "Sl", &mut ar); /* get info about it */
                if ar.currentline > 0 { /* is there info? */
                    lua_pushfstring(L, "%s:%d: ", ar.short_src, ar.currentline);
                    return;
                }
            }
            lua_pushliteral(L, ""); /* else, no information available... */
        }
      
        /*
** Again, the use of 'lua_pushvfstring' ensures this function does
** not need reserved stack space when called. (At worst, it generates
** an error with "stack overflow" instead of the given message.)
*/

    pub fn luaL_error(L: *mut lua_State, fmt: *const c_char, ...) -> c_int {
        let mut argp: va_list = ptr::null();
        va_start(argp, fmt);
        luaL_where(L, 1);
        lua_pushvfstring(L, fmt, argp);
        va_end(argp);
        lua_concat(L, 2);
        lua_error(L)
    }
        
    pub fn luaL_fileresult(L: *mut lua_State, stat: c_int, fname: *const c_char) -> c_int {
        let mut en: c_int = errno; /* calls to Lua API may change this value */
        if stat != 0 {
            lua_pushboolean(L, 1);
            return 1;
        } else {
            luaL_pushfail(L);
            if !fname.is_null() {
                lua_pushfstring(L, "%s: %s", fname, strerror(en));
            } else {
                lua_pushstring(L, strerror(en));
            }
            lua_pushinteger(L, en);
            return 3;
        }
    }

    /*
** use appropriate macros to interpret 'pclose' return status
*/

    #[cfg(LUA_USE_POSIX)]
    pub fn l_inspectstat(stat: c_int, what: *mut *const c_char) {
        if WIFEXITED(stat) != 0 {
            stat = WEXITSTATUS(stat);
        } else if WIFSIGNALED(stat) != 0 {
            stat = WTERMSIG(stat);
            *what = "signal";
        }
    }   

    #[cfg(not(LUA_USE_POSIX))]
    pub fn l_inspectstat(stat: c_int, what: *mut *const c_char) {
        // if (stat & 0x7f) != 0 {
        //     stat = stat & 0x7f;
        //     *what = "signal";
        // }
        /* no op */
    }


      pub fn luaL_execresult(L: *mut lua_State, stat: c_int) -> c_int {
        if stat != 0 && errno != 0 {
            return luaL_fileresult(L, 0, ptr::null());
        } else {
            // DEV ??
            let mut what: *const c_char = b"exit\0" as *const u8 as *const c_char; /* type of termination */
            l_inspectstat(stat, &mut what); /* interpret result */
            if *what == b'e' as c_char && stat == 0 { /* successful termination? */
                lua_pushboolean(L, 1);
            } else {
                luaL_pushfail(L);
            }
            lua_pushstring(L, what);
            lua_pushinteger(L, stat);
            return 3; /* return true/fail,what,code */
        }
    }


/* }====================================================== */



/*
** {======================================================
** Userdata's metatable manipulation
** =======================================================
*/


    pub fn luaL_newmetatable(L: *mut lua_State, tname: *const c_char) -> c_int {
        if luaL_getmetatable(L, tname) != LUA_TNIL { /* name already in use? */
            return 0; /* leave previous value on top, but return 0 */
        }
        lua_pop(L, 1);
        lua_createtable(L, 0, 2); /* create metatable */
        lua_pushstring(L, tname);
        lua_setfield(L, -2, "__name"); /* metatable.__name = tname */
        lua_pushvalue(L, -1);
        lua_setfield(L, LUA_REGISTRYINDEX, tname); /* registry.name = metatable */
        return 1;
    }

    pub fn luaL_setmetatable(L: *mut lua_State, tname: *const c_char) {
        luaL_getmetatable(L, tname);
        lua_setmetatable(L, -2);
    }

    pub fn luaL_testudata(L: *mut lua_State, ud: c_int, tname: *const c_char) -> *mut c_void {
        let mut p: *mut c_void = lua_touserdata(L, ud);
        if !p.is_null() { /* value is a userdata? */
            if lua_getmetatable(L, ud) != 0 { /* does it have a metatable? */
                luaL_getmetatable(L, tname); /* get correct metatable */
                if lua_rawequal(L, -1, -2) == 0 { /* not the same? */
                    p = ptr::null_mut(); /* value is a userdata with wrong metatable */
                }
                lua_pop(L, 2); /* remove both metatables */
                return p;
            }
        }
        return ptr::null_mut(); /* value is not a userdata with a metatable */  
    }

    pub fn luaL_checkudata(L: *mut lua_State, ud: c_int, tname: *const c_char) -> *mut c_void {
        let mut p: *mut c_void = luaL_testudata(L, ud, tname);
       luaL_argexpected(L, p != ptr::null_mut(), ud, tname);
        return p;
    }

    /* }====================================================== */


/*
** {======================================================
** Argument check functions
** =======================================================
*/

    pub fn luaL_checkoption(L: *mut lua_State, arg: c_int, def: *const c_char, lst: *const *const c_char) -> c_int {
        let name: *const c_char = if !def.is_null() {
            luaL_optstring(L, arg, def)
        } else {
            luaL_checkstring(L, arg)
        };
        for (i, l) in lst.iter().enumerate() {
            if strcmp(name, *l) == 0 {
                return i;
            }
        }
        // DEV ???
        return luaL_argerror(L, arg, lua_pushfstring(L, b"invalid option '%s'\0" as *const u8 as *const c_char, name));
    }

       


    /*
** Ensures the stack has at least 'space' extra slots, raising an error
** if it cannot fulfill the request. (The error handling needs a few
** extra slots to format the error message. In case of an error without
** this extra space, Lua will generate the same 'stack overflow' error,
** but without 'msg'.)
*/

    pub fn luaL_checkstack(L: *mut lua_State, space: c_int, msg: *const c_char) {
        if l_unlikely(lua_checkstack(L, space) == 0) {
            if !msg.is_null() {
                //DEV ???
                luaL_error(L, b"stack overflow (%s)\0" as *const u8 as *const c_char, msg);
            } else {
                luaL_error(L, b"stack overflow\0" as *const u8 as *const c_char);
            }
        }
    }


    pub fn luaL_checktype(L: *mut lua_State, arg: c_int, t: c_int) {
        if l_unlikely(lua_type(L, arg) != t) {
            tag_error(L, arg, t);
        }
    }

    pub fn luaL_checkany(L: *mut lua_State, arg: c_int) {
        if l_unlikely(lua_type(L, arg) == LUA_TNONE) {
            // DEV ???
            luaL_argerror(L, arg, b"value expected\0" as *const u8 as *const c_char);
        }
    }

    pub fn luaL_checklstring(L: *mut lua_State, arg: c_int, len: *mut size_t) -> *const c_char {
        let s: *const c_char = lua_tolstring(L, arg, len);
        if l_unlikely(s.is_null()) {
            tag_error(L, arg, LUA_TSTRING);
        }
         s
    }

    pub fn luaL_optlstring(L: *mut lua_State, arg: c_int, def: *const c_char, len: *mut size_t) -> *const c_char {
        if lua_isnoneornil(L, arg) {
            if !len.is_null() {
                *len = if !def.is_null() { strlen(def) } else { 0 };
            }
            return def;
        } else {
            return luaL_checklstring(L, arg, len);
        }
    }

    pub fn luaL_checknumber(L: *mut lua_State, arg: c_int) -> lua_Number {
        let mut isnum: c_int = 0;
        let d: lua_Number = lua_tonumberx(L, arg, &mut isnum);
        if l_unlikely(isnum == 0) {
            tag_error(L, arg, LUA_TNUMBER);
        }
        return d;
    }

    pub fn luaL_optnumber(L: *mut lua_State, arg: c_int, def: lua_Number) -> lua_Number {
        return luaL_opt(L, luaL_checknumber, arg, def);
    }

    pub fn interror(L: *mut lua_State, arg: c_int) {
       if lua_isnumber(L, arg) != 0 {
            // DEV ???
            luaL_argerror(L, arg, b"number has no integer representation\0" as *const u8 as *const c_char);
        } else {
            tag_error(L, arg, LUA_TNUMBER);
        }
    }

    pub fn luaL_checkinteger(L: *mut lua_State, arg: c_int) -> lua_Integer {
        let mut isnum: c_int = 0;
        let d: lua_Integer = lua_tointegerx(L, arg, &mut isnum);
        if l_unlikely(isnum == 0) {
            interror(L, arg);
        }
        return d;
    }

    pub fn luaL_optinteger(L: *mut lua_State, arg: c_int, def: lua_Integer) -> lua_Integer {
        return luaL_opt(L, luaL_checkinteger, arg, def);
    }

/* }====================================================== */


/*
** {======================================================
** Generic Buffer manipulation
** =======================================================
*/

/* userdata to box arbitrary data */
struct UBox{
    bbox: *mut c_void,
    bsize: size_t,   
}


    fn resizebox(L: *mut lua_State, idx: c_int, newsize: size_t) -> *mut c_void {
        let ud: *mut c_void = ptr::null_mut();
        let allocf: lua_Alloc = lua_getallocf(L, &mut ud);
        let bbox: *mut UBox = lua_touserdata(L, idx) as *mut UBox;
        let temp: *mut c_void = allocf(ud, (*bbox).bbox, (*bbox).bsize, newsize);
        if l_unlikely(temp.is_null() && newsize > 0) {
            // DEV ???
            lua_pushliteral(L, b"not enough memory\0" as *const u8 as *const c_char);
            lua_error(L);
        }
        (*bbox).bbox = temp;
        (*bbox).bsize = newsize;
        return temp;
    }




     fn boxgc(L: *mut lua_State) -> c_int {
        resizebox(L, 1, 0);
        return 0;
      }

//TODO static const luaL_Reg boxmt[] = {  /* box metamethods */
// {"__gc", boxgc},
// {"__close", boxgc},
// {NULL, NULL}
// };
static boxmt: [luaL_Reg; 3] = [ /* box metamethods */
    luaL_Reg{ name: b"__gc\0" as *const u8 as *const c_char, func: boxgc },
    luaL_Reg{ name: b"__close\0" as *const u8 as *const c_char, func: boxgc },
    luaL_Reg{ name: ptr::null(), func: None },
];



    pub fn newbox(L: *mut lua_State) {
        let bbox: *mut UBox = lua_newuserdatauv(L, sizeof(UBox), 0) as *mut UBox;
        (*bbox).bbox = ptr::null_mut();
        (*bbox).bsize = 0;
        if luaL_newmetatable(L, b"LuaBox\0" as *const u8 as *const c_char) != 0 { /* creating metatable? */
            luaL_setfuncs(L, boxmt.as_ptr(), 0); /* set its functions */
            // lua_pushvalue(L, -1);
            // lua_setfield(L, -2, b"__index\0" as *const u8 as *const c_char);
        }
        lua_setmetatable(L, -2);
    }


    /*
** check whether buffer is using a userdata on the stack as a temporary
** buffer
*/
fn buffonstack(B: *mut luaL_Buffer) -> bool {
    (*B).b != (*B).init.b
}

/*
** Whenever buffer is accessed, slot 'idx' must either be a box (which
** cannot be NULL) or it is a placeholder for the buffer.
*/
fn checkbufferlevel(B: *mut luaL_Buffer, idx: c_int) {
    // TODO is this the same as #define checkbufferlevel(B,idx)  \
//   lua_assert(buffonstack(B) ? lua_touserdata(B->L, idx) != NULL  \
//   : lua_touserdata(B->L, idx) == (void*)B)
    if buffonstack(B) {
        lua_assert(lua_isnoneornil(B.L, idx));
    } else {
        lua_assert(lua_isuserdata(B.L, idx));
    }
}

/*
** Compute new size for buffer 'B', enough to accommodate extra 'sz'
** bytes.
*/
fn newbuffsize(B: *mut luaL_Buffer, sz: size_t) -> size_t {
    let mut newsize: size_t = (*B).size * 2;  /* double buffer size */
    if l_unlikely(MAX_SIZET - sz < (*B).n) {  /* overflow in (B->n + sz)? */
        // DEV ???
        return luaL_error(B.L, b"buffer too large\0" as *const u8 as *const c_char);
    }
    if newsize < (*B).n + sz {  /* double is not big enough? */
        newsize = (*B).n + sz;
    }
    return newsize;
}

/*
** Returns a pointer to a free area with at least 'sz' bytes in buffer
** 'B'. 'boxidx' is the relative position in the stack where is the
** buffer's box or its placeholder.
*/

    fn prepbuffsize(B: *mut luaL_Buffer, sz: size_t, boxidx: c_int) -> *mut c_char {
        checkbufferlevel(B, boxidx);
        if (*B).size - (*B).n >= sz {  /* enough space? */
            return (*B).b.offset((*B).n as isize);
        } else {
            let L: *mut lua_State = (*B).L;
            let mut newbuff: *mut c_char = ptr::null_mut();
            let newsize: size_t = newbuffsize(B, sz);
            /* create larger buffer */
            if buffonstack(B) {  /* buffer already has a box? */
                newbuff = resizebox(L, boxidx, newsize) as *mut c_char;  /* resize it */
            } else {  /* no box yet */
                lua_remove(L, boxidx);  /* remove placeholder */
                newbox(L);  /* create a new box */
                lua_insert(L, boxidx);  /* move box to its intended position */
                lua_toclose(L, boxidx);
                newbuff = resizebox(L, boxidx, newsize) as *mut c_char;
                memcpy(newbuff as *mut c_void, (*B).b as *const c_void, (*B).n * sizeof(c_char));  /* copy original content */
            }
            (*B).b = newbuff;
            (*B).size = newsize;
            return newbuff.offset((*B).n as isize);
        }
    }

    /*
** returns a pointer to a free area with at least 'sz' bytes
*/
    pub fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: size_t) -> *mut c_char {
        return prepbuffsize(B, sz, -1);
    }

        

  
        pub fn luaL_addlstring(B: *mut luaL_Buffer, s: *const c_char, l: size_t) {
            if l > 0 {  /* avoid 'memcpy' when 's' can be NULL */
            let b: *mut c_char = prepbuffsize(B, l, -1);
            memcpy(b as *mut c_void, s as *const c_void, l * sizeof(c_char));
            luaL_addsize(B, l);
            }
        }

            pub fn luaL_addstring(B: *mut luaL_Buffer, s: *const c_char) {
                luaL_addlstring(B, s, strlen(s));
            }


              // convert above to rust
              pub fn luaL_pushresult(B: *mut luaL_Buffer) {
                let L: *mut lua_State = (*B).L;
                checkbufferlevel(B, -1);
                lua_pushlstring(L, (*B).b, (*B).n);
                if buffonstack(B) {
                    lua_closeslot(L, -2);  /* close the box */
                }
                lua_remove(L, -2);  /* remove box or placeholder from the stack */
              }