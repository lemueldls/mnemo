<script setup lang="ts">
import type { Package } from "~~/server/api/list-packages";

const props = defineProps<{ spaceId: string }>();

const { $api } = useNuxtApp();

const open = defineModel<boolean>();

const search = ref("");

const namespace = "preview" as const;
const packages = await $api("/api/list-packages", {
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
  installTypstPackage(pkg, namespace);

  const packagesItem = await useStorageItem<Package[]>(
    `spaces/${props.spaceId}/packages.json`,
    [],
  );
  packagesItem.value.push(pkg);
}
</script>

<template>
  <md-dialog :open="open" class="min-w-xl min-h-xl" @closed="open = false">
    <div slot="headline" class="flex justify-between gap-4">
      <span>Packages</span>

      <md-outlined-text-field
        :value="search"
        placeholder="Search packages"
        class="flex-1"
        @input="search = $event.target.value"
      />

      <!-- <md-icon-button @click="open = false">
        <md-icon>close</md-icon>
      </md-icon-button> -->
    </div>

    <form slot="content" method="dialog">
      <template v-if="Object.keys(filteredPackages).length > 0">
        <package-card
          v-for="(pkgs, i) in filteredPackages"
          :key="i"
          :packages="pkgs"
          class="flex flex-col gap-2"
          @install="installPackage"
        />
      </template>
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
