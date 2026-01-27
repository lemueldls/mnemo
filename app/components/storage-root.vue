<script setup lang="ts">
defineProps<{ keys: Record<string, StoragePath> }>();
</script>

<template>
  <div v-for="(path, name) in keys" :key="path.key">
    <storage-item :path :name />
  </div>
</template>
