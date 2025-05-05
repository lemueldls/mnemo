<script setup lang="ts">
import { createId } from "@paralleldrive/cuid2";

const open = defineModel<boolean>();

// const router = useRouter();

const spaces = await useSpaces();
const space = ref({
  name: "",
  icon: "",
  color: "#16161d", // Eigengrau
  order: spaces.value.length,
});

function createSpace() {
  const id = createId();
  spaces.value[id] = space.value;

  // void router.push(`/space?id=${id}`);
  open.value = false;
}
</script>

<template>
  <m3-theme :color="space.color" harmonize>
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
