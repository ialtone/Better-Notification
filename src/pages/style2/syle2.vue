<template>
    <div class="box"></div>
</template>

<script setup>
</script>


<style scoped>
.box {
    position: relative;
    width: 200px;
    height: 200px;
    background-color: #fff;
    border-radius: 10px;
}

.box::before,
.box::after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border: 2px solid #000;
    border-radius: inherit;
}

.box::before {
    top: 5px;
    left: 5px;
    right: 5px;
    bottom: 5px;
    border-width: 1px;
}

.box::after {
    top: 10px;
    left: 10px;
    right: 10px;
    bottom: 10px;
    border-width: 2px;
}
</style>