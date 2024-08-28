<script setup lang="ts">

async function send() {
  await $fetch("/api/chat", {
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
