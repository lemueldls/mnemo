<script setup lang="ts">
const tasks = await useTasks();

interface TaskItemRef {
  width: number;
  height: number;
}

// Masonry layout configuration
const containerRef = useTemplateRef<HTMLDivElement>("container");
const taskRefs = ref<{ [id: string]: TaskItemRef }>({});
const columnCount = ref(3);
const columnGap = 12;
const rowGap = 12;

// Responsive column count based on container width
const { width: containerWidth } = useElementSize(containerRef);

watchEffect(() => {
  if (!containerWidth.value) return;

  if (containerWidth.value < 600) {
    columnCount.value = 1;
  } else if (containerWidth.value < 900) {
    columnCount.value = 2;
  } else if (containerWidth.value < 1200) {
    columnCount.value = 3;
  } else {
    columnCount.value = 4;
  }
});

const taskIds = computed(() => Object.keys(tasks.value));

// Calculate positions for masonry layout
const taskPositions = computed(() => {
  if (!taskIds.value.length || !columnCount.value) return {};

  const positions: { [id: string]: { x: number; y: number; width: number } } =
    {};
  const columnHeights = Array.from({ length: columnCount.value }, () => 0);
  const columnWidth = containerWidth.value
    ? (containerWidth.value - (columnCount.value - 1) * columnGap) /
      columnCount.value
    : 300;

  taskIds.value.forEach((id) => {
    const taskRef = taskRefs.value[id];
    if (!taskRef || columnHeights.length === 0) return;

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

    positions[id] = {
      x,
      y,
      width: columnWidth,
    };

    // Update column height
    columnHeights[shortestColumn] =
      columnHeights[shortestColumn]! + taskRef.height + rowGap;
  });

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

// Handle task ref updates
const handleTaskRef = (taskId: string, ref: TaskItemRef | null) => {
  if (ref) {
    taskRefs.value[taskId] = ref;
  } else {
    const { [taskId]: _, ...newRefs } = taskRefs.value;
    taskRefs.value = newRefs;
  }
};

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
      v-if="!Object.keys(taskPositions).length && taskIds.length"
      class="flex min-h-40 items-center justify-center"
    >
      <div class="text-surface-variant animate-pulse">Loading tasks...</div>
    </div>

    <!-- Empty state -->
    <span v-else-if="!taskIds.length" class="text-on-surface-variant">
      No tasks yet.
    </span>

    <span v-else class="text-on-surface-variant">Loading...</span>
  </div>
</template>
