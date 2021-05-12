import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

export default new Vuex.Store({
        state:{
            user:{
                username:window.localStorage.getItem('user'||'[]')==null?'':JSON.parse(window.localStorage.getItem('user'||'[]')).username,
                id:window.localStorage.getItem('user'||'[]')==null?'':JSON.parse(window.localStorage.getItem('user'||'[]')).id,
            },
            token:window.localStorage.getItem('token'||'[]')==null?'':JSON.parse(window.localStorage.getItem('token'||'[]')),
        },
        mutations:{
            login(state,info){
                console.log('info',info);
                state.user.username = info.username;
                state.user.id = info.id;
                state.token = info.token;
                window.localStorage.setItem('user',JSON.stringify({
                    username: info.username,
                    id: info.id,
                }));
                window.localStorage.setItem('token',JSON.stringify(info.token));
            },
            logout(state){
                state.user = {
                    username: '',
                    id: '',
                    password:''
                };
                state.token = "";
                window.localStorage.removeItem('user');
                window.localStorage.removeItem('token');
            }
        }  
})