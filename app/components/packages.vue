<script setup lang="ts">
import { UseVirtualList } from "@vueuse/components";

import type { Package } from "~~/server/api/list-packages";

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

  const searchLower = search.value.toLowerCase();

  return Object.fromEntries(
    Object.entries(packages).filter(
      ([name, [pkg]]) =>
        name.toLowerCase().includes(searchLower) ||
        pkg!.description.toLowerCase().includes(searchLower),
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
    (pkgItem) => !isSamePackage(pkg, pkgItem),
  );
}
</script>

<template>
  <md-dialog :open="open" class="size-xl" @closed="open = false">
    <div slot="headline" class="flex justify-between gap-6">
      <span>{{ $t("components.packages.title") }}</span>

      <md-outlined-text-field
        :value="search"
        :placeholder="$t('components.packages.form.search')"
        class="flex-1"
        @input="search = $event.target.value"
      />

      <!-- <md-icon-button @click="open = false">
        <md-icon>close</md-icon>
      </md-icon-button> -->
    </div>

    <form slot="content" method="dialog" class="px-0">
      <UseVirtualList
        v-if="Object.keys(filteredPackages).length > 0"
        ref="container"
        :list="Object.values(filteredPackages)"
        :options="{ itemHeight: medium ? 220 : 264 }"
        height="27rem"
        class="virtual-list px-6"
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

      <md-filled-card v-else class="label-large mx-4 p-4">
        {{ $t("components.packages.form.no-results") }}
      </md-filled-card>
    </form>
  </md-dialog>
</template>

<style>
/* .virtual-list {
  height: 29rem !important;
} */

.virtual-list > div {
  @apply flex flex-col gap-4;
}
</style>
