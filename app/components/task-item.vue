<script setup lang="ts">
const task = defineModel<Task>("task", { required: true });

const ready = ref(true);
const editingTask = useEditingTask();
const container = useTemplateRef("container");
const { width, height } = useElementSize(container);

watchEffect(() => {
  console.log({ ready: ready.value });
});

const containerWidth = ref(0);
const containerHeight = ref(0);

watch(
  [width, ready],
  ([newWidth, ready]) => {
    if (newWidth && ready) containerWidth.value = newWidth;
  },
  { immediate: true },
);

watch(
  [height, ready],
  ([newHeight, ready]) => {
    if (newHeight && ready) containerHeight.value = newHeight;
  },
  { immediate: true },
);

defineExpose({
  width: readonly(containerWidth),
  height: readonly(containerHeight),
});

const showContent = computed(() => task.value.id !== editingTask.value?.id);

function editTask() {
  editingTask.value = task.value;
  ready.value = false;
}

const spaces = await useSpaces();
</script>

<template>
  <mx-theme :key="task.id" :color="spaces[task.spaceId]!.color" harmonize>
    <div
      v-if="showContent"
      ref="container"
      class="w-full"
      :style="
        ready
          ? undefined
          : {
              width: `${containerWidth}px`,
              height: `${containerHeight}px`,
            }
      "
    >
      <md-filled-card
        class="relative flex size-full cursor-pointer flex-col gap-3 p-2 transition-all duration-200 hover:shadow-md"
        @click="editTask"
      >
        <md-ripple />

        <editor
          kind="task"
          :space-id="task.spaceId"
          :model-value="task.id"
          locked
          readonly
          @ready="ready = true"
        />
      </md-filled-card>
    </div>

    <div
      v-else-if="containerWidth && containerHeight"
      class="border-outline bg-surface-container-low flex items-center justify-center rounded-lg border-2 border-dashed p-4"
      :style="{ width: `${containerWidth}px`, height: `${containerHeight}px` }"
    />
  </mx-theme>
</template>
