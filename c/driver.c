#include <lua.h>
#include <lauxlib.h>
#include <lualib.h> 


#include <stdio.h>

typedef struct {
    int background_color[4];
} MyState;


extern void change_bg_color(MyState* state, int* colors);

static int wrapper_bg_color(lua_State *L) {
   MyState* st = lua_topointer(L, -1);
   int* colors = lua_topointer(L, -2);
   change_bg_color(st, colors);
   return 1;
}


void call_lua(MyState* state, const char* script) {
    //  pwd();
    //rust_log("DEU BOM PORRA");
    lua_State *L;
    L = luaL_newstate();
    
    printf("C: loading lua script %s\n", script);
    luaL_openlibs(L);
    luaL_loadfile(L, script);
   
    lua_pushcfunction(L, wrapper_bg_color);
    lua_setglobal(L, "change_bg_color");
   
    printf("C: %d\n", sizeof(state));
    printf("C: state is %d\n", state->background_color[3]);

    lua_pushlightuserdata(L, state); // pass state to lua script
    lua_setglobal(L, "state");

    if (lua_pcall(L, 0, 0, 0))
        printf("C: falhou: %s\n", lua_tostring(L, -1));

    lua_getglobal(L, "state");

    state = lua_topointer(L, -1); // assign pointer to the lua return
    printf("C: new state is %d\n", state->background_color[3]);
    lua_close(L);
}
