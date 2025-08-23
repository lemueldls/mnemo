<script setup lang="ts">
const tasks = await useTasks();

const newTaskOpen = ref(false);

const route = useRoute();
const isSpace = route.name === "space";
const spaceId = isSpace ? usePageRouteQuery("id") : undefined;
</script>

<template>
  <div class="flex flex-col gap-3">
    <md-filled-tonal-button @click="newTaskOpen = true">
      <md-icon slot="icon">add</md-icon>
      New Task
    </md-filled-tonal-button>

    <task-item v-for="(task, i) in tasks" :key="i" v-model:task="tasks[i]!" />
  </div>

  <new-task v-model:open="newTaskOpen" :space-id="spaceId" />
</template>
