/*
** $Id: lauxlib.h,v 1.88.1.1 2007/12/27 13:02:25 roberto Exp $
** Auxiliary functions for building Lua libraries
** See Copyright Notice in lua.h
*/



#[cfg(not(LUA_COMPAT_GETN))]

pub fn luaL_getn(L: *mut lua_State, i: c_int) -> c_int {
    unsafe { lua_objlen(L, i) as c_int }
}

#[cfg(not(LUA_COMPAT_GETN))]
pub fn luaL_setn(L: *mut lua_State, i: c_int, j: c_int) {
    // no op!
}

#[cfg(LUA_COMPAT_GETN)]
pub fn luaL_getn(L: *mut lua_State, i: c_int) -> c_int;

#[cfg(LUA_COMPAT_GETN)]
pub fn luaL_setn(L: *mut lua_State, i: c_int, j: c_int);

#[cfg(LUA_COMPAT_OPENLIB)]
pub fn luaI_openlib(L: *mut lua_State, libname: *const c_char, lr: *const luaL_Reg, nup: c_int) -> c_int {
    luaL_openlib(L, libname, lr, nup)
}


/* extra error code for `luaL_load' */
pub const LUA_ERRFILE: c_int = (LUA_ERRERR + 1);



struct luaL_Reg {
 name: *const c_char,
   func:lua_CFunction
}


// LUALIB_API void (luaI_openlib) (lua_State *L, const char *libname,
//                                 const luaL_Reg *l, int nup);
// LUALIB_API void (luaL_register) (lua_State *L, const char *libname,
//                                 const luaL_Reg *l);
// LUALIB_API int (luaL_getmetafield) (lua_State *L, int obj, const char *e);
// LUALIB_API int (luaL_callmeta) (lua_State *L, int obj, const char *e);
// LUALIB_API int (luaL_typerror) (lua_State *L, int narg, const char *tname);
// LUALIB_API int (luaL_argerror) (lua_State *L, int numarg, const char *extramsg);
// LUALIB_API const char *(luaL_checklstring) (lua_State *L, int numArg,
//                                                           size_t *l);
// LUALIB_API const char *(luaL_optlstring) (lua_State *L, int numArg,
//                                           const char *def, size_t *l);
// LUALIB_API lua_Number (luaL_checknumber) (lua_State *L, int numArg);
// LUALIB_API lua_Number (luaL_optnumber) (lua_State *L, int nArg, lua_Number def);

// LUALIB_API lua_Integer (luaL_checkinteger) (lua_State *L, int numArg);
// LUALIB_API lua_Integer (luaL_optinteger) (lua_State *L, int nArg,
//                                           lua_Integer def);

// LUALIB_API void (luaL_checkstack) (lua_State *L, int sz, const char *msg);
// LUALIB_API void (luaL_checktype) (lua_State *L, int narg, int t);
// LUALIB_API void (luaL_checkany) (lua_State *L, int narg);

// LUALIB_API int   (luaL_newmetatable) (lua_State *L, const char *tname);
// LUALIB_API void *(luaL_checkudata) (lua_State *L, int ud, const char *tname);

// LUALIB_API void (luaL_where) (lua_State *L, int lvl);
// LUALIB_API int (luaL_error) (lua_State *L, const char *fmt, ...);

// LUALIB_API int (luaL_checkoption) (lua_State *L, int narg, const char *def,
//                                    const char *const lst[]);

// LUALIB_API int (luaL_ref) (lua_State *L, int t);
// LUALIB_API void (luaL_unref) (lua_State *L, int t, int ref);

// LUALIB_API int (luaL_loadfile) (lua_State *L, const char *filename);
// LUALIB_API int (luaL_loadbuffer) (lua_State *L, const char *buff, size_t sz,
//                                   const char *name);
// LUALIB_API int (luaL_loadstring) (lua_State *L, const char *s);

// LUALIB_API lua_State *(luaL_newstate) (void);


// LUALIB_API const char *(luaL_gsub) (lua_State *L, const char *s, const char *p,
//                                                   const char *r);

// LUALIB_API const char *(luaL_findtable) (lua_State *L, int idx,
//                                          const char *fname, int szhint);




/*
** ===============================================================
** some useful macros
** ===============================================================
*/
// TODO copilot can autocomplete these into macros per below
// #define luaL_argcheck(L, cond,numarg,extramsg)	\
// 		((void)((cond) || luaL_argerror(L, (numarg), (extramsg))))
// #define luaL_checkstring(L,n)	(luaL_checklstring(L, (n), NULL))
// #define luaL_optstring(L,n,d)	(luaL_optlstring(L, (n), (d), NULL))
// #define luaL_checkint(L,n)	((int)luaL_checkinteger(L, (n)))
// #define luaL_optint(L,n,d)	((int)luaL_optinteger(L, (n), (d)))
// #define luaL_checklong(L,n)	((long)luaL_checkinteger(L, (n)))
// #define luaL_optlong(L,n,d)	((long)luaL_optinteger(L, (n), (d)))

// #define luaL_typename(L,i)	lua_typename(L, lua_type(L,(i)))

// #define luaL_dofile(L, fn) \
// 	(luaL_loadfile(L, fn) || lua_pcall(L, 0, LUA_MULTRET, 0))

// #define luaL_dostring(L, s) \
// 	(luaL_loadstring(L, s) || lua_pcall(L, 0, LUA_MULTRET, 0))

// #define luaL_getmetatable(L,n)	(lua_getfield(L, LUA_REGISTRYINDEX, (n)))

// #define luaL_opt(L,f,n,d)	(lua_isnoneornil(L,(n)) ? (d) : f(L,(n)))


// // TODO here

// macro_rules! luaL_argcheck {
//     ($L:expr, $cond:expr, $numarg:expr, $extramsg:expr) => {
//         if !$cond {
//             luaL_argerror!($L, $numarg, $extramsg);
//         }
//     };
// }

// macro_rules! luaL_checkstring {
//     ($L:expr, $n:expr) => {
//         luaL_checklstring!($L, $n, std::ptr::null_mut())
//     };
// }

// macro_rules! luaL_optstring {
//     ($L:expr, $n:expr, $d:expr) => {
//         luaL_optlstring!($L, $n, $d, std::ptr::null_mut())
//     };
// }

/*
** {======================================================
** Generic Buffer manipulation
** =======================================================
*/


// typedef struct luaL_Buffer {
//   char *p;			/* current position in buffer */
//   int lvl;  /* number of strings in the stack (level) */
//   lua_State *L;
//   char buffer[LUAL_BUFFERSIZE];
// } luaL_Buffer;

// the above struct to rust
pub struct luaL_Buffer {
    p: *mut c_char, /* current position in buffer */
    lvl: c_int, /* number of strings in the stack (level) */
    L: *mut lua_State,
    buffer: [c_char; LUAL_BUFFERSIZE],
}

macro_rules! luaL_addchar {
    ($B:expr, $c:expr) => {
        if $B.p < $B.buffer + LUAL_BUFFERSIZE {
            luaL_prepbuffer!($B);
        }
        *$B.p = $c;
        $B.p += 1;
    };
}




/* compatibility only */
macro_rules! luaL_putchar {
    ($B:expr, $c:expr) => {
        luaL_addchar!($B, $c);
    };
}

macro_rules! luaL_addsize {
    ($B:expr, $n:expr) => {
        $B.p += $n;
    };
}



// LUALIB_API void (luaL_buffinit) (lua_State *L, luaL_Buffer *B);
// LUALIB_API char *(luaL_prepbuffer) (luaL_Buffer *B);
// LUALIB_API void (luaL_addlstring) (luaL_Buffer *B, const char *s, size_t l);
// LUALIB_API void (luaL_addstring) (luaL_Buffer *B, const char *s);
// LUALIB_API void (luaL_addvalue) (luaL_Buffer *B);
// LUALIB_API void (luaL_pushresult) (luaL_Buffer *B);


/* }====================================================== */


/* compatibility with ref system */

/* pre-defined references */
pub const LUA_NOREF: c_int = -2;
pub const LUA_REFNIL: c_int = -1;



// #define lua_ref(L,lock) ((lock) ? luaL_ref(L, LUA_REGISTRYINDEX) : \
//       (lua_pushstring(L, "unlocked references are obsolete"), lua_error(L), 0))

// #define lua_unref(L,ref)        luaL_unref(L, LUA_REGISTRYINDEX, (ref))

// #define lua_getref(L,ref)       lua_rawgeti(L, LUA_REGISTRYINDEX, (ref))

// pub const LUA_REGISTRYINDEX: c_int = -10000;

pub fn lua_ref(L: *mut lua_State, lock: c_int) -> c_int {
    if lock != 0 {
        luaL_ref(L, LUA_REGISTRYINDEX)
    } else {
        lua_pushstring(L, "unlocked references are obsolete");
        lua_error(L);
        0
    }
}   

pub fn lua_unref(L: *mut lua_State, reff: c_int) -> c_int {
    luaL_unref(L, LUA_REGISTRYINDEX, reff)
}

pub fn lua_getref(L: *mut lua_State, reff: c_int) {
    lua_rawgeti(L, LUA_REGISTRYINDEX, reff)
}   



type luaL_reg=luaL_Reg


// YELLOW ===================================================================

/*
** $Id: lauxlib.c,v 1.159.1.3 2008/01/21 13:20:51 roberto Exp $
** Auxiliary functions for building Lua libraries
** See Copyright Notice in lua.h
*/


// #include <ctype.h>
// #include <errno.h>
// #include <stdarg.h>
// #include <stdio.h>
// #include <stdlib.h>
// #include <string.h>


/* This file uses only the official API of Lua.
** Any function declared here could be written as an application function.
*/

// #define lauxlib_c
// #define LUA_LIB

// #include "lua.h"

// #include "lauxlib.h"


pub const FREELIST_REF: c_int = 0;  /* free list of references */


/* convert a stack index to positive */
pub fn abs_index(L: *mut lua_State, i: c_int) -> c_int {
    if i > 0 || i <= LUA_REGISTRYINDEX {
        i
    } else {
        lua_gettop(L) + i + 1
    }
}


/*
** {======================================================
** Error-report functions
** =======================================================
*/


// LUALIB_API int luaL_argerror (lua_State *L, int narg, const char *extramsg) {
//   lua_Debug ar;
//   if (!lua_getstack(L, 0, &ar))  /* no stack frame? */
//     return luaL_error(L, "bad argument #%d (%s)", narg, extramsg);
//   lua_getinfo(L, "n", &ar);
//   if (strcmp(ar.namewhat, "method") == 0) {
//     narg--;  /* do not count `self' */
//     if (narg == 0)  /* error is in the self argument itself? */
//       return luaL_error(L, "calling " LUA_QS " on bad self (%s)",
//                            ar.name, extramsg);
//   }
//   if (ar.name == NULL)
//     ar.name = "?";
//   return luaL_error(L, "bad argument #%d to " LUA_QS " (%s)",
//                         narg, ar.name, extramsg);
// }

pub fn luaL_argerror(L: *mut lua_State, narg: c_int, extramsg: *const c_char) -> c_int {
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: ptr::null(),
        namewhat: ptr::null(),
        what: ptr::null(),
        source: ptr::null(),
        currentline: 0,
        nups: 0,
        linedefined: 0,
        short_src: [0; LUA_IDSIZE],
        i_ci: 0,
    };
    if lua_getstack(L, 0, &mut ar) == 0 {
        return luaL_error(L, "bad argument #%d (%s)", narg, extramsg);
    }
    lua_getinfo(L, "n", &mut ar);
    if strcmp(ar.namewhat, "method") == 0 {
        narg -= 1;
        if narg == 0 {
            //DEV LUA_QS
            return luaL_error(L, format!"calling '{}' on bad self {}", ar.name, extramsg));
        }
    }
    if ar.name == ptr::null() {
        ar.name = "?";
    }
    return luaL_error(L, format!("bad argument {} to '{}' {}", narg, ar.name, extramsg));
}


LUALIB_API int luaL_typerror (lua_State *L, int narg, const char *tname) {
  const char *msg = lua_pushfstring(L, "%s expected, got %s",
                                    tname, luaL_typename(L, narg));
  return luaL_argerror(L, narg, msg);
}


static void tag_error (lua_State *L, int narg, int tag) {
  luaL_typerror(L, narg, lua_typename(L, tag));
}


LUALIB_API void luaL_where (lua_State *L, int level) {
  lua_Debug ar;
  if (lua_getstack(L, level, &ar)) {  /* check function at level */
    lua_getinfo(L, "Sl", &ar);  /* get info about it */
    if (ar.currentline > 0) {  /* is there info? */
      lua_pushfstring(L, "%s:%d: ", ar.short_src, ar.currentline);
      return;
    }
  }
  lua_pushliteral(L, "");  /* else, no information available... */
}


LUALIB_API int luaL_error (lua_State *L, const char *fmt, ...) {
  va_list argp;
  va_start(argp, fmt);
  luaL_where(L, 1);
  lua_pushvfstring(L, fmt, argp);
  va_end(argp);
  lua_concat(L, 2);
  return lua_error(L);
}

/* }====================================================== */


LUALIB_API int luaL_checkoption (lua_State *L, int narg, const char *def,
                                 const char *const lst[]) {
  const char *name = (def) ? luaL_optstring(L, narg, def) :
                             luaL_checkstring(L, narg);
  int i;
  for (i=0; lst[i]; i++)
    if (strcmp(lst[i], name) == 0)
      return i;
  return luaL_argerror(L, narg,
                       lua_pushfstring(L, "invalid option " LUA_QS, name));
}


LUALIB_API int luaL_newmetatable (lua_State *L, const char *tname) {
  lua_getfield(L, LUA_REGISTRYINDEX, tname);  /* get registry.name */
  if (!lua_isnil(L, -1))  /* name already in use? */
    return 0;  /* leave previous value on top, but return 0 */
  lua_pop(L, 1);
  lua_newtable(L);  /* create metatable */
  lua_pushvalue(L, -1);
  lua_setfield(L, LUA_REGISTRYINDEX, tname);  /* registry.name = metatable */
  return 1;
}


LUALIB_API void *luaL_checkudata (lua_State *L, int ud, const char *tname) {
  void *p = lua_touserdata(L, ud);
  if (p != NULL) {  /* value is a userdata? */
    if (lua_getmetatable(L, ud)) {  /* does it have a metatable? */
      lua_getfield(L, LUA_REGISTRYINDEX, tname);  /* get correct metatable */
      if (lua_rawequal(L, -1, -2)) {  /* does it have the correct mt? */
        lua_pop(L, 2);  /* remove both metatables */
        return p;
      }
    }
  }
  luaL_typerror(L, ud, tname);  /* else error */
  return NULL;  /* to avoid warnings */
}


LUALIB_API void luaL_checkstack (lua_State *L, int space, const char *mes) {
  if (!lua_checkstack(L, space))
    luaL_error(L, "stack overflow (%s)", mes);
}


LUALIB_API void luaL_checktype (lua_State *L, int narg, int t) {
  if (lua_type(L, narg) != t)
    tag_error(L, narg, t);
}


LUALIB_API void luaL_checkany (lua_State *L, int narg) {
  if (lua_type(L, narg) == LUA_TNONE)
    luaL_argerror(L, narg, "value expected");
}


LUALIB_API const char *luaL_checklstring (lua_State *L, int narg, size_t *len) {
  const char *s = lua_tolstring(L, narg, len);
  if (!s) tag_error(L, narg, LUA_TSTRING);
  return s;
}


LUALIB_API const char *luaL_optlstring (lua_State *L, int narg,
                                        const char *def, size_t *len) {
  if (lua_isnoneornil(L, narg)) {
    if (len)
      *len = (def ? strlen(def) : 0);
    return def;
  }
  else return luaL_checklstring(L, narg, len);
}


LUALIB_API lua_Number luaL_checknumber (lua_State *L, int narg) {
  lua_Number d = lua_tonumber(L, narg);
  if (d == 0 && !lua_isnumber(L, narg))  /* avoid extra test when d is not 0 */
    tag_error(L, narg, LUA_TNUMBER);
  return d;
}


LUALIB_API lua_Number luaL_optnumber (lua_State *L, int narg, lua_Number def) {
  return luaL_opt(L, luaL_checknumber, narg, def);
}


LUALIB_API lua_Integer luaL_checkinteger (lua_State *L, int narg) {
  lua_Integer d = lua_tointeger(L, narg);
  if (d == 0 && !lua_isnumber(L, narg))  /* avoid extra test when d is not 0 */
    tag_error(L, narg, LUA_TNUMBER);
  return d;
}


LUALIB_API lua_Integer luaL_optinteger (lua_State *L, int narg,
                                                      lua_Integer def) {
  return luaL_opt(L, luaL_checkinteger, narg, def);
}


LUALIB_API int luaL_getmetafield (lua_State *L, int obj, const char *event) {
  if (!lua_getmetatable(L, obj))  /* no metatable? */
    return 0;
  lua_pushstring(L, event);
  lua_rawget(L, -2);
  if (lua_isnil(L, -1)) {
    lua_pop(L, 2);  /* remove metatable and metafield */
    return 0;
  }
  else {
    lua_remove(L, -2);  /* remove only metatable */
    return 1;
  }
}


LUALIB_API int luaL_callmeta (lua_State *L, int obj, const char *event) {
  obj = abs_index(L, obj);
  if (!luaL_getmetafield(L, obj, event))  /* no metafield? */
    return 0;
  lua_pushvalue(L, obj);
  lua_call(L, 1, 1);
  return 1;
}


LUALIB_API void (luaL_register) (lua_State *L, const char *libname,
                                const luaL_Reg *l) {
  luaI_openlib(L, libname, l, 0);
}


static int libsize (const luaL_Reg *l) {
  int size = 0;
  for (; l->name; l++) size++;
  return size;
}


LUALIB_API void luaI_openlib (lua_State *L, const char *libname,
                              const luaL_Reg *l, int nup) {
  if (libname) {
    int size = libsize(l);
    /* check whether lib already exists */
    luaL_findtable(L, LUA_REGISTRYINDEX, "_LOADED", 1);
    lua_getfield(L, -1, libname);  /* get _LOADED[libname] */
    if (!lua_istable(L, -1)) {  /* not found? */
      lua_pop(L, 1);  /* remove previous result */
      /* try global variable (and create one if it does not exist) */
      if (luaL_findtable(L, LUA_GLOBALSINDEX, libname, size) != NULL)
        luaL_error(L, "name conflict for module " LUA_QS, libname);
      lua_pushvalue(L, -1);
      lua_setfield(L, -3, libname);  /* _LOADED[libname] = new table */
    }
    lua_remove(L, -2);  /* remove _LOADED table */
    lua_insert(L, -(nup+1));  /* move library table to below upvalues */
  }
  for (; l->name; l++) {
    int i;
    for (i=0; i<nup; i++)  /* copy upvalues to the top */
      lua_pushvalue(L, -nup);
    lua_pushcclosure(L, l->func, nup);
    lua_setfield(L, -(nup+2), l->name);
  }
  lua_pop(L, nup);  /* remove upvalues */
}



/*
** {======================================================
** getn-setn: size for arrays
** =======================================================
*/

#if defined(LUA_COMPAT_GETN)

static int checkint (lua_State *L, int topop) {
  int n = (lua_type(L, -1) == LUA_TNUMBER) ? lua_tointeger(L, -1) : -1;
  lua_pop(L, topop);
  return n;
}


static void getsizes (lua_State *L) {
  lua_getfield(L, LUA_REGISTRYINDEX, "LUA_SIZES");
  if (lua_isnil(L, -1)) {  /* no `size' table? */
    lua_pop(L, 1);  /* remove nil */
    lua_newtable(L);  /* create it */
    lua_pushvalue(L, -1);  /* `size' will be its own metatable */
    lua_setmetatable(L, -2);
    lua_pushliteral(L, "kv");
    lua_setfield(L, -2, "__mode");  /* metatable(N).__mode = "kv" */
    lua_pushvalue(L, -1);
    lua_setfield(L, LUA_REGISTRYINDEX, "LUA_SIZES");  /* store in register */
  }
}


LUALIB_API void luaL_setn (lua_State *L, int t, int n) {
  t = abs_index(L, t);
  lua_pushliteral(L, "n");
  lua_rawget(L, t);
  if (checkint(L, 1) >= 0) {  /* is there a numeric field `n'? */
    lua_pushliteral(L, "n");  /* use it */
    lua_pushinteger(L, n);
    lua_rawset(L, t);
  }
  else {  /* use `sizes' */
    getsizes(L);
    lua_pushvalue(L, t);
    lua_pushinteger(L, n);
    lua_rawset(L, -3);  /* sizes[t] = n */
    lua_pop(L, 1);  /* remove `sizes' */
  }
}


LUALIB_API int luaL_getn (lua_State *L, int t) {
  int n;
  t = abs_index(L, t);
  lua_pushliteral(L, "n");  /* try t.n */
  lua_rawget(L, t);
  if ((n = checkint(L, 1)) >= 0) return n;
  getsizes(L);  /* else try sizes[t] */
  lua_pushvalue(L, t);
  lua_rawget(L, -2);
  if ((n = checkint(L, 2)) >= 0) return n;
  return (int)lua_objlen(L, t);
}

#endif

/* }====================================================== */



LUALIB_API const char *luaL_gsub (lua_State *L, const char *s, const char *p,
                                                               const char *r) {
  const char *wild;
  size_t l = strlen(p);
  luaL_Buffer b;
  luaL_buffinit(L, &b);
  while ((wild = strstr(s, p)) != NULL) {
    luaL_addlstring(&b, s, wild - s);  /* push prefix */
    luaL_addstring(&b, r);  /* push replacement in place of pattern */
    s = wild + l;  /* continue after `p' */
  }
  luaL_addstring(&b, s);  /* push last suffix */
  luaL_pushresult(&b);
  return lua_tostring(L, -1);
}


LUALIB_API const char *luaL_findtable (lua_State *L, int idx,
                                       const char *fname, int szhint) {
  const char *e;
  lua_pushvalue(L, idx);
  do {
    e = strchr(fname, '.');
    if (e == NULL) e = fname + strlen(fname);
    lua_pushlstring(L, fname, e - fname);
    lua_rawget(L, -2);
    if (lua_isnil(L, -1)) {  /* no such field? */
      lua_pop(L, 1);  /* remove this nil */
      lua_createtable(L, 0, (*e == '.' ? 1 : szhint)); /* new table for field */
      lua_pushlstring(L, fname, e - fname);
      lua_pushvalue(L, -2);
      lua_settable(L, -4);  /* set new table into field */
    }
    else if (!lua_istable(L, -1)) {  /* field has a non-table value? */
      lua_pop(L, 2);  /* remove table and value */
      return fname;  /* return problematic part of the name */
    }
    lua_remove(L, -2);  /* remove previous table */
    fname = e + 1;
  } while (*e == '.');
  return NULL;
}



/*
** {======================================================
** Generic Buffer manipulation
** =======================================================
*/


#define bufflen(B)	((B)->p - (B)->buffer)
#define bufffree(B)	((size_t)(LUAL_BUFFERSIZE - bufflen(B)))

#define LIMIT	(LUA_MINSTACK/2)


static int emptybuffer (luaL_Buffer *B) {
  size_t l = bufflen(B);
  if (l == 0) return 0;  /* put nothing on stack */
  else {
    lua_pushlstring(B->L, B->buffer, l);
    B->p = B->buffer;
    B->lvl++;
    return 1;
  }
}


static void adjuststack (luaL_Buffer *B) {
  if (B->lvl > 1) {
    lua_State *L = B->L;
    int toget = 1;  /* number of levels to concat */
    size_t toplen = lua_strlen(L, -1);
    do {
      size_t l = lua_strlen(L, -(toget+1));
      if (B->lvl - toget + 1 >= LIMIT || toplen > l) {
        toplen += l;
        toget++;
      }
      else break;
    } while (toget < B->lvl);
    lua_concat(L, toget);
    B->lvl = B->lvl - toget + 1;
  }
}


LUALIB_API char *luaL_prepbuffer (luaL_Buffer *B) {
  if (emptybuffer(B))
    adjuststack(B);
  return B->buffer;
}


LUALIB_API void luaL_addlstring (luaL_Buffer *B, const char *s, size_t l) {
  while (l--)
    luaL_addchar(B, *s++);
}


LUALIB_API void luaL_addstring (luaL_Buffer *B, const char *s) {
  luaL_addlstring(B, s, strlen(s));
}


LUALIB_API void luaL_pushresult (luaL_Buffer *B) {
  emptybuffer(B);
  lua_concat(B->L, B->lvl);
  B->lvl = 1;
}


LUALIB_API void luaL_addvalue (luaL_Buffer *B) {
  lua_State *L = B->L;
  size_t vl;
  const char *s = lua_tolstring(L, -1, &vl);
  if (vl <= bufffree(B)) {  /* fit into buffer? */
    memcpy(B->p, s, vl);  /* put it there */
    B->p += vl;
    lua_pop(L, 1);  /* remove from stack */
  }
  else {
    if (emptybuffer(B))
      lua_insert(L, -2);  /* put buffer before new value */
    B->lvl++;  /* add new value into B stack */
    adjuststack(B);
  }
}


LUALIB_API void luaL_buffinit (lua_State *L, luaL_Buffer *B) {
  B->L = L;
  B->p = B->buffer;
  B->lvl = 0;
}

/* }====================================================== */


LUALIB_API int luaL_ref (lua_State *L, int t) {
  int ref;
  t = abs_index(L, t);
  if (lua_isnil(L, -1)) {
    lua_pop(L, 1);  /* remove from stack */
    return LUA_REFNIL;  /* `nil' has a unique fixed reference */
  }
  lua_rawgeti(L, t, FREELIST_REF);  /* get first free element */
  ref = (int)lua_tointeger(L, -1);  /* ref = t[FREELIST_REF] */
  lua_pop(L, 1);  /* remove it from stack */
  if (ref != 0) {  /* any free element? */
    lua_rawgeti(L, t, ref);  /* remove it from list */
    lua_rawseti(L, t, FREELIST_REF);  /* (t[FREELIST_REF] = t[ref]) */
  }
  else {  /* no free elements */
    ref = (int)lua_objlen(L, t);
    ref++;  /* create new reference */
  }
  lua_rawseti(L, t, ref);
  return ref;
}


LUALIB_API void luaL_unref (lua_State *L, int t, int ref) {
  if (ref >= 0) {
    t = abs_index(L, t);
    lua_rawgeti(L, t, FREELIST_REF);
    lua_rawseti(L, t, ref);  /* t[ref] = t[FREELIST_REF] */
    lua_pushinteger(L, ref);
    lua_rawseti(L, t, FREELIST_REF);  /* t[FREELIST_REF] = ref */
  }
}



/*
** {======================================================
** Load functions
** =======================================================
*/

typedef struct LoadF {
  int extraline;
  FILE *f;
  char buff[LUAL_BUFFERSIZE];
} LoadF;


static const char *getF (lua_State *L, void *ud, size_t *size) {
  LoadF *lf = (LoadF *)ud;
  (void)L;
  if (lf->extraline) {
    lf->extraline = 0;
    *size = 1;
    return "\n";
  }
  if (feof(lf->f)) return NULL;
  *size = fread(lf->buff, 1, sizeof(lf->buff), lf->f);
  return (*size > 0) ? lf->buff : NULL;
}


static int errfile (lua_State *L, const char *what, int fnameindex) {
  const char *serr = strerror(errno);
  const char *filename = lua_tostring(L, fnameindex) + 1;
  lua_pushfstring(L, "cannot %s %s: %s", what, filename, serr);
  lua_remove(L, fnameindex);
  return LUA_ERRFILE;
}


LUALIB_API int luaL_loadfile (lua_State *L, const char *filename) {
  LoadF lf;
  int status, readstatus;
  int c;
  int fnameindex = lua_gettop(L) + 1;  /* index of filename on the stack */
  lf.extraline = 0;
  if (filename == NULL) {
    lua_pushliteral(L, "=stdin");
    lf.f = stdin;
  }
  else {
    lua_pushfstring(L, "@%s", filename);
    lf.f = fopen(filename, "r");
    if (lf.f == NULL) return errfile(L, "open", fnameindex);
  }
  c = getc(lf.f);
  if (c == '#') {  /* Unix exec. file? */
    lf.extraline = 1;
    while ((c = getc(lf.f)) != EOF && c != '\n') ;  /* skip first line */
    if (c == '\n') c = getc(lf.f);
  }
  if (c == LUA_SIGNATURE[0] && filename) {  /* binary file? */
    lf.f = freopen(filename, "rb", lf.f);  /* reopen in binary mode */
    if (lf.f == NULL) return errfile(L, "reopen", fnameindex);
    /* skip eventual `#!...' */
   while ((c = getc(lf.f)) != EOF && c != LUA_SIGNATURE[0]) ;
    lf.extraline = 0;
  }
  ungetc(c, lf.f);
  status = lua_load(L, getF, &lf, lua_tostring(L, -1));
  readstatus = ferror(lf.f);
  if (filename) fclose(lf.f);  /* close file (even in case of errors) */
  if (readstatus) {
    lua_settop(L, fnameindex);  /* ignore results from `lua_load' */
    return errfile(L, "read", fnameindex);
  }
  lua_remove(L, fnameindex);
  return status;
}


typedef struct LoadS {
  const char *s;
  size_t size;
} LoadS;


static const char *getS (lua_State *L, void *ud, size_t *size) {
  LoadS *ls = (LoadS *)ud;
  (void)L;
  if (ls->size == 0) return NULL;
  *size = ls->size;
  ls->size = 0;
  return ls->s;
}


LUALIB_API int luaL_loadbuffer (lua_State *L, const char *buff, size_t size,
                                const char *name) {
  LoadS ls;
  ls.s = buff;
  ls.size = size;
  return lua_load(L, getS, &ls, name);
}


LUALIB_API int (luaL_loadstring) (lua_State *L, const char *s) {
  return luaL_loadbuffer(L, s, strlen(s), s);
}



/* }====================================================== */


static void *l_alloc (void *ud, void *ptr, size_t osize, size_t nsize) {
  (void)ud;
  (void)osize;
  if (nsize == 0) {
    free(ptr);
    return NULL;
  }
  else
    return realloc(ptr, nsize);
}


static int panic (lua_State *L) {
  (void)L;  /* to avoid warnings */
  fprintf(stderr, "PANIC: unprotected error in call to Lua API (%s)\n",
                   lua_tostring(L, -1));
  return 0;
}


LUALIB_API lua_State *luaL_newstate (void) {
  lua_State *L = lua_newstate(l_alloc, NULL);
  if (L) lua_atpanic(L, &panic);
  return L;
}

