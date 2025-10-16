<script setup lang="ts">
const tasks = await useTasks();
const sortedTasks = computed(() => tasks.sorted);

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

    <task-item
      v-for="(task, i) in sortedTasks"
      :key="task.id"
      v-model:task="sortedTasks[i]!"
    />
  </div>
</template>
