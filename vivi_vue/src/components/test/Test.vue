<template>
    <div class="header-container">
        <h1 id="title">title</h1>
        <div id="background"></div>
        <div id="mountain"></div>
        <h1>test</h1>
        <div>hello</div>
    </div>
</template>

<script>
export default{
    name:'test',
}

</script>

<style scoped>
#header-container{
    width:80%;
    height: 200px;
}
#background{
    background: black;
}
#title{
    position: fixed;
    top: 50px;
}
#mountain{
    background: url("~@/assets/img/mountain1.png");
    background-size: cover;
    width: 100%;
    height: 800px;
}
</style>
