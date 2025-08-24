<script setup lang="ts">
const tasks = await useTasks();

const newTaskOpen = useNewTaskOpen();

const containerRef = useTemplateRef("container");
const scroll = useScroll(containerRef);

// const scrollX = useState("tasks:scroll-x", () => 0);
// watch(scroll.x, (x) => (scrollX.value = x));
const scrollY = useState("tasks:scroll-y", () => 0);
watch(scroll.y, (y) => (scrollY.value = y));

onMounted(() => {
  // scroll.x.value = scrollX.value;
  scroll.y.value = scrollY.value;
});
</script>

<template>
  <div ref="container" class="flex h-full flex-col gap-3 overflow-y-auto">
    <md-filled-tonal-button @click="newTaskOpen = true">
      <md-icon slot="icon">add</md-icon>
      New Task
    </md-filled-tonal-button>

    <task-item v-for="(task, i) in tasks" :key="i" v-model:task="tasks[i]!" />
  </div>
</template>
