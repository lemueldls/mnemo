<script setup lang="ts">
import { useVirtualizer } from "@tanstack/vue-virtual";

import type { Package } from "~~/server/api/list-packages.get";

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

const packages = ref<{ [name: string]: Package[] }>();

async function loadPackages() {
  try {
    packages.value = await $api("/api/list-packages", {
      query: { namespace },
    });
  } catch (error) {
    console.error("Error loading package list:", error);
  }
}

loadPackages();

const filteredPackages = computed(() => {
  const packageList = packages.value;
  if (!packageList) return;

  if (search.value === "") {
    return Object.values(packageList);
  }

  const searchLower = search.value.toLowerCase();

  return Object.entries(packageList)
    .filter(
      ([name, [pkg]]) =>
        name.toLowerCase().includes(searchLower) ||
        pkg!.description.toLowerCase().includes(searchLower),
    )
    .map(([_, pkgs]) => pkgs);
});

const packagesVirtualizer = useVirtualizer(
  computed(() => ({
    // count: filteredPackages.value?.length || 0,
    count: 5,
    getScrollElement: () => containerRef.value,
    estimateSize: () => (medium.value ? 220 : 264),
  })),
);

const virtualPackages = computed(() =>
  packagesVirtualizer.value.getVirtualItems(),
);
// const totalSize = computed(() => packagesVirtualizer.value.getTotalSize())
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

    <form
      ref="container"
      slot="content"
      method="dialog"
      class="flex h-full flex-col gap-4 px-6"
    >
      <template
        v-if="filteredPackages && filteredPackages.length > 0"
        v-for="{ key, index } in virtualPackages"
        :key="filteredPackages[index]?.[0]?.name || key"
      >
        <package-card
          v-if="filteredPackages[index]"
          :space-id
          :namespace
          :versioned-package="filteredPackages[index]"
          class="flex flex-col gap-2"
        />
      </template>

      <md-filled-card v-else class="label-large mx-4 p-2">
        <div class="flex items-center justify-between">
          <span>
            {{ $t("components.packages.form.no-results") }}
          </span>

          <md-filled-tonal-button @click.prevent="loadPackages">
            {{ $t("components.packages.form.reload") }}
          </md-filled-tonal-button>
        </div>
      </md-filled-card>
    </form>
  </md-dialog>
</template>
