<script setup lang="ts">
const open = defineModel<boolean>();

const dark = useDark();
const name = await useStorageItem("name", "");
</script>

<template>
  <md-dialog :open="open" @closed="open = false">
    <span slot="headline">Settings</span>

    <form slot="content" method="dialog" class="flex flex-col gap-4">
      <label class="flex items-center justify-between gap-4">
        Dark Theme

        <md-switch
          aria-label="Dark Theme"
          icons
          :selected="dark"
          @change="dark = $event.target.selected"
        />
      </label>

      <label class="flex items-center justify-between gap-4">
        <md-outlined-text-field
          label="Name"
          :value="name"
          @input="name = $event.target.value"
        />
      </label>
    </form>
  </md-dialog>
</template>
