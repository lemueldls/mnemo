<script setup lang="ts">
import type { Package } from "~~/server/api/list-packages";
import { PackageFile } from "mnemo-wasm";

const open = defineModel();
const typst = useTypst();

// await $fetch("/api/clear-cache");

const search = ref("");

const namespace = "preview" as const;
const packages = await $fetch("/api/list-packages", {
  query: { namespace },
});

const filteredPackages = computed(() => {
  if (search.value === "") {
    return packages;
  }

  return Object.fromEntries(
    Object.entries(packages).filter(([name]) =>
      name.toLowerCase().includes(search.value.toLowerCase()),
    ),
  );
});

async function installPackage(pkg: Package) {
  const { spec, files } = await $fetch("/api/get-package", {
    query: { namespace, name: pkg.name, version: pkg.version },
  });

  console.log({ spec, files });

  (await typst).installPackage(
    spec,
    files.map(
      (file) => new PackageFile(file.path, Uint8Array.from(file.content!)),
    ),
  );
}
</script>

<template>
  <md-dialog :open="open" @closed="open = false" class="min-w-xl min-h-xl">
    <div class="flex gap-4 justify-between" slot="headline">
      <span>Packages</span>

      <md-outlined-text-field
        :value="search"
        @input="search = $event.target.value"
        placeholder="Search packages"
        class="flex-1"
      ></md-outlined-text-field>

      <!-- <md-icon-button @click="open = false">
        <md-icon>close</md-icon>
      </md-icon-button> -->
    </div>

    <form slot="content" method="dialog">
      <m3-elevated-card
        v-if="Object.keys(filteredPackages).length > 0"
        v-for="[pkg] in filteredPackages"
        :key="pkg.name"
        class="flex flex-col gap-2"
      >
        <h1 class="text-m3-on-surface-variant m3-title-large">
          {{ pkg.name }}
        </h1>

        <span class="text-m3-on-surface-variant m3-body-large">
          Version: {{ pkg.version }}
        </span>

        <div class="flex-1 flex flex-col gap-2">
          <span class="text-m3-on-surface-variant m3-body-large">
            {{ pkg.description }}
          </span>
        </div>

        <div class="flex gap-2">
          <div class="flex-[2]" />
          <md-outlined-button
            class="flex-[3]"
            @click.prevent="installPackage(pkg)"
          >
            Install
          </md-outlined-button>
        </div>
      </m3-elevated-card>
      <span v-else>No packages found</span>
    </form>
  </md-dialog>
</template>

<style>
form {
  @apply grid gap-4;

  grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));
}
</style>
