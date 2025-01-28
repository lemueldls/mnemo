<script setup lang="ts">
import { createId } from "@paralleldrive/cuid2";

const open = defineModel();

const dark = useDark();

const spaces = await useSpaces();
const space = ref({
  name: "",
  icon: "",
  color: "#16161d", // Eigengrau
  order: spaces.value.length,
});

function createSpace() {
  // const id = createId();
  // spaces.value[id] = space;

  console.log(spaces.value, space.value);
  spaces.value.push(space.value);
}
</script>

<template>
  <m3-theme :color="space.color" harmonize :dark="dark">
    <md-dialog :open="open" @closed="open = false">
      <span slot="headline">New Space</span>

      <form slot="content" method="dialog" class="flex flex-col gap-8">
        <edit-space v-model="space">
          <template #actions>
            <md-text-button @click="createSpace">Create</md-text-button>
          </template>
        </edit-space>
      </form>
    </md-dialog>
  </m3-theme>
</template>
