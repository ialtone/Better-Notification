<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow, LogicalSize } from '@tauri-apps/api/window';

const message = ref('');
// 调整窗口大小
const adjustWindowSize = async (message: string) => {
    const baseWidth = 300;
    const baseHeight = 100;
    const charWidth = 10; // 每个字符增加的宽度
    const maxWidth = 500; // 最大宽度
    const lines = message.split("\n").length; // 计算行数
    const longestLineLength = Math.max(...message.split("\n").map(line => line.length)); // 最长行的字符数
    const newWidth = Math.min(baseWidth + (longestLineLength * charWidth), maxWidth); // 限制最大宽度
    const newHeight = baseHeight + (lines * 20); // 每行增加的高度

    await appWindow.setSize(new LogicalSize(newWidth, newHeight));
};

onMounted(async () => {
    message.value = await invoke("get_str_arg");
    await adjustWindowSize(message.value);
});

const close = async () => {
    invoke("close_window");
};
</script>

<template>
    <div class="main" @contextmenu.prevent>
        <div class="close_btn" @click="close">✖</div>
        <div class="message" v-html="message.replace(/\n/g, '<br>')"></div>
    </div>
</template>

<style scoped>
.main {
    width: 100%;
    height: 100%;
    /* 居中显示 */
    display: flex;
    justify-content: center;
    align-items: center;
    text-align: center;
}

.message {
    font-size: 20px;
    font-weight: bold;
    cursor: default;
    white-space: pre-wrap;
    /* 允许换行 */
    word-wrap: break-word;
    /* 允许单词换行 */
}

.close_btn {
    position: absolute;
    top: -5px;
    right: 0;
    margin: 10px;
    cursor: pointer;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    user-select: none;
}

.close_btn:hover {
    color: red;
}
</style>