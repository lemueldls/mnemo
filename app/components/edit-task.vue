<script setup lang="ts">
const task = useEditingTask();

const tasks = await useTasks();
const spaces = await useSpaces();

function deleteTask() {
  if (!task.value) return;
  tasks.delete(task.value.id);
  task.value = undefined;
}
</script>

<template>
  <md-dialog :open="!!task" class="w-full max-w-sm" @closed="task = undefined">
    <div slot="headline" class="flex justify-between">
      <span>
        {{ $t("components.edit-task.title") }}
      </span>

      <div v-if="task" class="flex gap-2">
        <md-icon-button @click="deleteTask">
          <md-icon>delete</md-icon>
        </md-icon-button>
        <component
          :is="task.pinned ? 'md-filled-tonal-icon-button' : 'md-icon-button'"
          @click="task.pinned = !task.pinned"
        >
          <md-icon>keep</md-icon>
        </component>
      </div>
    </div>

    <form
      id="edit-task-form"
      slot="content"
      method="dialog"
      class="flex flex-col gap-3"
      @submit.prevent="void 0"
    >
      <mx-theme v-if="task" :color="spaces[task.spaceId]!.color" harmonize>
        <md-filled-card class="p-3">
          <editor
            :space-id="task.spaceId"
            kind="task"
            class="text-on-background h-50"
            :model-value="task.id"
          />
        </md-filled-card>
      </mx-theme>
    </form>

    <!-- <div slot="actions">
      <md-text-button form="edit-task-form">Update</md-text-button>
    </div> -->
  </md-dialog>
</template>
