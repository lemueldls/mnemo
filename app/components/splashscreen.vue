<script setup lang="ts">
const emit = defineEmits<{ (e: "ready", isReady: boolean): void }>();

const progress = ref(0);
const totalProgress = 4;

onMounted(() => {
  progress.value++;

  const typstState = useTypst();
  typstState.then(() => progress.value++);

  useTimeoutFn(() => {
    progress.value++;
  }, 750);
});

onNuxtReady(async () => {
  progress.value++;
});

const ready = computed(() => progress.value >= totalProgress);
whenever(ready, () => emit("ready", true));
</script>

<template>
  <m3-theme id="splashscreen" color="#16161d" dark :class="{ ready }">
    <div class="w-sm flex flex-col gap-2">
      <div class="m3-label-large flex justify-between">
        <span class="font-bold">Loading...</span>

        <span>{{ progress }} / {{ totalProgress }}</span>
      </div>

      <md-linear-progress :value="progress / totalProgress" />
    </div>
  </m3-theme>
</template>

<style>
#splashscreen {
  @apply text-m3-on-background bg-m3-background z-12 absolute inset-0 flex h-full w-full items-center justify-center overflow-y-auto overflow-x-hidden opacity-100 transition-opacity;

  background-color: #3f4178;
  color: #c0c1ff;

  &.ready {
    @apply pointer-events-none select-none opacity-0;
  }
}
</style>
