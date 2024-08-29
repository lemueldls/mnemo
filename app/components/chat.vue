<script setup lang="ts">
const { $api } = useNuxtApp();

async function send() {
  await $api("/api/chat", {
    method: "post",
    body: { message: "hi" },
  });
}
</script>

<template>
  <div class="flex-1 flex flex-col gap-2">
    <div class="flex-1">
    </div>

    <md-filled-button class="w-full" @click="send">
      Send
    </md-filled-button>
  </div>
</template>
