import store from '@/store'
import axios from 'axios';
const baseURL = 'https://vlive.uniqueandroid.com:10005/api';
//const baseURL = 'http://localhost:8080/api';
const defaultUrl = "https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png";
export default{
    async getAvatar(uid){
        if (uid == ''){
            return defaultUrl;
        }
        let avatar;
        //console.log('get avatar');
        //如果是自己的头像
        //console.log(store.state.avatar);
        //sessionStrorage中有存
        if (uid == store.state.user.id && store.state.avatar){
            //console.log('get store avata');      
            return store.state.avatar;
        }
        else{
            avatar = await this.getAxios(uid).then((rsp)=>{
                return(rsp.data)
            });
            //console.log(avatar);
            if (uid == store.state.user.id && avatar){
                //存起来
                store.commit('setAvatar',avatar);
                console.log('cmmit avatar');
            }
        }
        return avatar;
    },
    getAxios(uid){
        return new Promise((resolve)=>{
            resolve(axios.post(baseURL+'/avatar',{'id':uid}));
        });
    }
}
