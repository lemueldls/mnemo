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

const showContent = computed(
  () => task.value.id !== editingTask.value?.id || !containerWidth.value || !containerHeight.value,
);

const spaces = await useSpaces();
const space = computed(() => spaces.value[task.value.spaceId]);

const tasks = await useTasks();
watchImmediate(space, (space) => {
  if (!space) tasks.delete(task.value.id);
});
</script>

<template>
  <mx-theme :key="task.id" :color="space!.color" harmonize>
    <div
      v-if="showContent"
      ref="container"
      class="size-full"
      :style="
        ready || !containerWidth || !containerHeight
          ? undefined
          : { width: `${containerWidth}px`, height: `${containerHeight}px` }
      "
    >
      <mx-filled-card
        :class="['relative size-full cursor-pointer p-3', { 'border-primary border': task.pinned }]"
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
      </mx-filled-card>
    </div>

    <md-outlined-card
      v-else
      :style="
        containerWidth && containerHeight
          ? { width: `${containerWidth}px`, height: `${containerHeight}px` }
          : undefined
      "
    />
  </mx-theme>
</template>
