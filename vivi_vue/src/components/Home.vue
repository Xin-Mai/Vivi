<template>
    <div>
        <nav-menu class="menu"></nav-menu>
        <router-view class="router-view"/>
    </div>
    
</template>

<script>
    import NavMenu from './common/NavMenu.vue'
    export default {
        name:'Home',
        components:{NavMenu}
    }
</script>

<style scoped>
*{
    background: #E4E7ED;
}
.menu{
    position: sticky;
    top: 0;
}
.router-view{
    padding:0px 10px 0px 10px;
    width: 92%;
    margin: 0 auto;
    background:white;
}
</style>