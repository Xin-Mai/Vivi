<template>
    <div>
        <box class="flexbox">
        </box>
        <div class="tri">
            <div class="left">
                <ol>
                <li>元素1</li>
                <li>元素2</li>
                <li>元素3</li>
                </ol>
            </div>
            <div class="middle">
                <ol>
                <li>元素1</li>
                <li>元素2</li>
                <li>元素3</li>
                </ol>
            </div>
            <div class="right">
                <ol>
                <li>元素1</li>
                <li>元素2</li>
                <li>元素3</li>
                </ol>
            </div>
        </div>
        <box class="flexbox">
            <h3>Hello again!</h3>
            <h3>Goodbye!</h3>
        </box>
    </div>
</template>

<script>
import ItemCard from '../common/itemCard.vue'
export default {
    name:'FollowIndex',
    components:{ItemCard},
    data(){
        return{
        }
    }
}
</script>

<style scoped>
    .flexbox{
        background-color: brown;
        display: flex;
        flex-direction: column;
        height: 100px;
        justify-content: center;
        align-items: center;

    }
    .tri{
        display: flex;
        flex: row;
        justify-content: flex-start;
        align-items: center;
    }
    .left{
        background-color: cadetblue;
        flex-shrink: 0;
        width: 300px;
    }
    .middle{
        background-color: cornflowerblue;
        width: 50%;
    }
    .right{
        background-color: crimson;
        width: 50%;
    }
</style>