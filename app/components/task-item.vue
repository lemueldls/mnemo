<script setup lang="ts">
const task = defineModel<Task>("task", { required: true });

const ready = ref(true);
const editingTask = useEditingTask();

const container = useTemplateRef("container");
const { width, height } = useElementSize(container);

const containerWidth = ref(0);
const containerHeight = ref(0);

watchImmediate([width, ready], ([width, ready]) => {
  if (width && ready) containerWidth.value = width;
});

watchImmediate([height, ready], ([height, ready]) => {
  if (height && ready) containerHeight.value = height;
});

watchImmediate(editingTask, (editingTask) => {
  if (editingTask && editingTask.id === task.value.id) ready.value = false;
});

defineExpose({
  width: readonly(containerWidth),
  height: readonly(containerHeight),
});

const spaces = await useSpaces();

const showContent = computed(() => task.value.id !== editingTask.value?.id);
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
          : { width: `${containerWidth}px`, height: `${containerHeight}px` }
      "
    >
      <md-filled-card
        class="relative flex size-full cursor-pointer flex-col gap-3 p-2 transition-all duration-200 hover:shadow-md"
        @click="editingTask = task"
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

    <md-outlined-card
      v-else-if="containerWidth && containerHeight"
      :style="{ width: `${containerWidth}px`, height: `${containerHeight}px` }"
    />
  </mx-theme>
</template>
