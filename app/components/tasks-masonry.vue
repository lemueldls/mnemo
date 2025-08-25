<script setup lang="ts">
const tasks = await useTasks();

interface TaskItemRef {
  width: number;
  height: number;
}

const taskRefs = ref<{ [id: string]: TaskItemRef }>({});

// Masonry layout configuration
const columnCount = ref(3);
const columnGap = 12;
const rowGap = 12;

const containerRef = useTemplateRef("container");
const { width: containerWidth } = useElementSize(containerRef);

watchEffect(() => {
  if (!containerWidth.value) return;

  if (containerWidth.value < 600) columnCount.value = 1;
  else if (containerWidth.value < 900) columnCount.value = 2;
  else if (containerWidth.value < 1200) columnCount.value = 3;
  else columnCount.value = 4;
});

// Sort tasks by pinned status and creation date
const sortedTasks = computed(() => {
  return Object.values(tasks.value).sort((a, b) => {
    // Pinned tasks first
    if (a.pinned && !b.pinned) return -1;
    if (!a.pinned && b.pinned) return 1;

    // Then by creation date (newest first)
    return b.createdAt - a.createdAt;
  });
});

// Calculate positions for masonry layout
const taskPositions = computed(() => {
  const taskIds = sortedTasks.value.map((task) => task.id);

  const columns = columnCount.value;
  if (!taskIds.length || !columns) return {};

  const positions: { [id: string]: { x: number; y: number; width: number } } =
    {};
  const columnHeights = Array.from({ length: columns }, () => 0);
  const width = containerWidth.value;
  const columnWidth = width
    ? (width - (columns - 1) * columnGap) / columns
    : 300;

  const refs = taskRefs.value;
  for (const id of taskIds) {
    const taskRef = refs[id];
    if (!taskRef || columnHeights.length === 0) continue;

    // Find the shortest column
    let shortestColumn = 0;
    let shortestHeight = columnHeights[0]!;

    for (let i = 1; i < columnHeights.length; i++) {
      if (columnHeights[i]! < shortestHeight) {
        shortestHeight = columnHeights[i]!;
        shortestColumn = i;
      }
    }

    // Calculate position
    const x = shortestColumn * (columnWidth + columnGap);
    const y = columnHeights[shortestColumn]!;

    positions[id] = { x, y, width: columnWidth };

    // Update column height
    columnHeights[shortestColumn] =
      columnHeights[shortestColumn]! + taskRef.height + rowGap;
  }

  return positions;
});

// Container height based on tallest column
const containerHeight = computed(() => {
  if (!Object.keys(taskPositions.value).length) return 0;

  let maxHeight = 0;
  Object.values(taskPositions.value).forEach((pos) => {
    const taskRef =
      taskRefs.value[
        Object.keys(taskPositions.value).find(
          (key) => taskPositions.value[key] === pos,
        )!
      ];
    if (taskRef) {
      const totalHeight = pos.y + taskRef.height;
      if (totalHeight > maxHeight) {
        maxHeight = totalHeight;
      }
    }
  });

  return maxHeight;
});

const handleTaskRef = (taskId: string, ref: TaskItemRef | null) => {
  if (ref) {
    taskRefs.value[taskId] = ref;
  } else {
    const { [taskId]: _, ...newRefs } = taskRefs.value;
    taskRefs.value = newRefs;
  }
};
</script>

<template>
  <div
    ref="container"
    class="relative w-full"
    :style="{ height: containerHeight > 0 ? `${containerHeight}px` : 'auto' }"
  >
    <div
      v-for="task in sortedTasks"
      :key="task.id"
      class="absolute transition-all duration-300 ease-in-out"
      :style="{
        transform: taskPositions[task.id]
          ? `translate(${taskPositions[task.id]!.x}px, ${taskPositions[task.id]!.y}px)`
          : undefined,
        width: taskPositions[task.id]?.width
          ? `${taskPositions[task.id]!.width}px`
          : '100%',
        opacity: taskPositions[task.id] ? 1 : 0,
      }"
    >
      <task-item
        :ref="(el) => handleTaskRef(task.id, el as any)"
        :task="task"
      />
    </div>

    <!-- Loading placeholder when no tasks positioned yet -->
    <div
      v-if="!Object.keys(taskPositions).length && sortedTasks.length"
      class="flex min-h-40 items-center justify-center"
    >
      <div class="text-surface-variant animate-pulse">Loading tasks...</div>
    </div>

    <!-- Empty state -->
    <span v-else-if="!sortedTasks.length" class="text-on-surface-variant">
      No tasks yet.
    </span>

    <span v-else class="text-on-surface-variant">Loading...</span>
  </div>
</template>
