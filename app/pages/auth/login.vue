<script setup lang="ts">
definePageMeta({ layout: "empty" });

const session = useRouteQuery("session");

window.open(`mnemo://auth/login?session=${session.value}`, "_self");
await navigateTo("/");
</script>

<template>
  <div />
</template>
