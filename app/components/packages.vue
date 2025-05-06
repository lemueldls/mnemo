<script setup lang="ts">
import type { Package } from "~~/server/api/list-packages";

import { UseVirtualList } from "@vueuse/components";

const props = defineProps<{ spaceId: string }>();

const { $api } = useNuxtApp();

const open = defineModel<boolean>();

const { medium } = useBreakpoints(breakpointsM3);

const search = ref("");
const containerRef = useTemplateRef<HTMLElement>("container");

watch(search, () => {
  const container = containerRef.value;
  if (container) container.scrollTo(0, 0);
});

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

const packagesItem = await useStorageItem<Package[]>(
  `spaces/${props.spaceId}/packages.json`,
  [],
);

async function installPackage(pkg: Package) {
  installTypstPackage(pkg, namespace);

  packagesItem.value.push(pkg);
}

async function uninstallPackage(pkg: Package) {
  packagesItem.value = packagesItem.value.filter(
    (pkgItem) => !comparePackage(pkg, pkgItem),
  );
}
</script>

<template>
  <md-dialog :open="open" @closed="open = false">
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

    <form slot="content" method="dialog" class="overflow-hidden">
      <UseVirtualList
        v-if="Object.keys(filteredPackages).length > 0"
        ref="container"
        :list="Object.values(filteredPackages)"
        :options="{ itemHeight: medium ? 220 : 264 }"
        height="28rem"
        class="virtual-list"
      >
        <template #default="{ data: versionedPackage }">
          <package-card
            :space-id
            :versioned-package
            class="flex flex-col gap-2"
            @install="installPackage"
            @uninstall="uninstallPackage"
          />
        </template>
      </UseVirtualList>
      <span v-else>No packages found</span>
    </form>
  </md-dialog>
</template>

<style>
.virtual-list > div {
  @apply flex flex-col gap-4;
}
</style>
