<script setup lang="ts">
import { createId } from "@paralleldrive/cuid2";

const open = useNewTaskOpen();

const route = useRoute();
const isSpace = route.name === "space";
const selectedSpaceId = isSpace ? usePageRouteQuery("id") : ref<string>();

const spaces = await useSpaces();

const tasks = await useTasks();
const newSpaceOpen = useNewSpaceOpen();
const editingTask = useEditingTask();

function createTask() {
  open.value = false;

  const id = createId();
  const spaceId = selectedSpaceId!.value!;
  const task = {
    id,
    spaceId,
    pinned: false,
    createdAt: Date.now(),
  };

  tasks.set(id, task);
  editingTask.value = task;
}
</script>

<template>
  <md-dialog :open class="w-full max-w-lg" @closed="open = false">
    <span slot="headline">{{ $t("components.new-task.title") }}</span>

    <form
      id="new-task-form"
      slot="content"
      method="dialog"
      class="grid grid-cols-2 gap-3"
    >
      <mx-theme
        v-for="(space, id) in spaces"
        :key="id"
        :color="space.color"
        harmonize
      >
        <md-outlined-card
          class="flex flex-col gap-2 p-3"
          @click="selectedSpaceId = id as string"
        >
          <md-ripple />

          <div class="flex flex-row items-center justify-between gap-2">
            <div class="h-6">
              <md-icon v-if="space.icon" class="text-primary">
                {{ space.icon }}
              </md-icon>
            </div>

            <md-radio
              name="space"
              :value="id"
              :checked="id === selectedSpaceId"
              required
            />
          </div>
          <span class="title-large line-clamp-1 flex-1" :title="space.name">
            {{ space.name }}
          </span>
        </md-outlined-card>
      </mx-theme>

      <md-elevated-card
        v-if="Object.keys(spaces).length < 1"
        class="cursor-pointer p-4"
        @click="newSpaceOpen = true"
      >
        <md-ripple />

        <div class="flex flex-row items-center justify-between gap-2">
          <md-icon class="text-primary">add</md-icon>
        </div>
        <span class="title-large flex-1">Create a New Space</span>
      </md-elevated-card>
    </form>

    <div slot="actions">
      <md-text-button form="new-task-form" @click="createTask">
        Create
      </md-text-button>
    </div>
  </md-dialog>
</template>
