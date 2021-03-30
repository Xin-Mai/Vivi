<template>
    <div>
        Hello,world!
    <show-card></show-card>
    <h2 v-for="i in 20" v-bind:key="i">{{i}}</h2>
    </div>
</template>

<script>
import ShowCard from '../common/imgShowCard'
export default {
    name:'AppIndex',
    components:{ShowCard}
}
</script>

<style scoped>

</style>
