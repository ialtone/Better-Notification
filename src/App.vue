<template>
  <RouterView />
</template>

<script setup>
import { onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/tauri';

const router = useRouter();

onMounted(async () => {
  try {
    const route = await invoke('get_route_arg');
    if (route) {
      router.push(route);
    } else {
      router.push('/');
    }
  } catch (error) {
    console.error('Failed to get route argument:', error);
    router.push('/');
  }
});
</script>

<style scoped></style>