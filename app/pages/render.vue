<script setup lang="ts">
definePageMeta({ layout: "space" });

const dark = useDark();

const { d } = useI18n();

const spaceId = useRouteQuery("space");

const spaces = await useSpaces();
const space = computed(() => spaces.value[spaceId.value]!);
</script>

<template>
  <m3-theme :color="space.color" :dark="false">
    <m3-page>
      <!-- <div class="bg-m3-surface p-8 m-8 overflow-auto h-full"> -->
      <pdf-renderer />
      <!-- </div> -->
    </m3-page>
  </m3-theme>
</template>

<style lang="scss">
/* #space-page {
  @apply absolute inset-0;
} */
</style>
