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
const installedPackages = await useInstalledPackages(() => props.spaceId);
const installedFilter = ref<"all" | "installed" | "not-installed">("all");

async function loadPackages() {
  try {
    packages.value = await $api("/api/list-packages", {
      query: { namespace },
    });
  } catch (error) {
    console.error("Error loading package list:", error);
  }
}

whenever(open, () => {
  loadPackages();
});

const filteredPackages = computed(() => {
  const packageList = packages.value;
  if (!packageList) return [];

  if (search.value === "") {
    let entries = Object.entries(packageList);

    if (installedFilter.value !== "all") {
      const installedNames = new Set(
        installedPackages.value.map((p) => p.name),
      );

      if (installedFilter.value === "installed") {
        entries = entries.filter(([name]) => installedNames.has(name));
      } else {
        entries = entries.filter(([name]) => !installedNames.has(name));
      }
    }

    return entries.map(([, pkgs]) => pkgs);
  }

  const searchLower = search.value.toLowerCase();

  return Object.entries(packageList)
    .filter(
      ([name, [pkg]]) =>
        name.toLowerCase().includes(searchLower) ||
        pkg!.description.toLowerCase().includes(searchLower),
    )
    .filter(([name]) => {
      if (installedFilter.value === "all") return true;

      const installedNames = new Set(
        installedPackages.value.map((p) => p.name),
      );

      return installedFilter.value === "installed"
        ? installedNames.has(name)
        : !installedNames.has(name);
    })
    .map(([_, pkgs]) => pkgs);
});

const packagesVirtualizer = useVirtualizer(
  computed(() => ({
    count: filteredPackages.value.length || 0,
    // count: 5,
    getScrollElement: () => containerRef.value,
    estimateSize: () => (medium.value ? 220 : 264),
    getItemKey: (index) => {
      const pkgs = filteredPackages.value[index];
      if (!pkgs) return "";

      const pkg = pkgs[0]!;

      return "@" + namespace + "/" + pkg.name;
    },
  })),
);

const virtualPackages = computed(() =>
  packagesVirtualizer.value.getVirtualItems(),
);
</script>

<template>
  <md-dialog :open="open" class="size-xl" @closed="open = false">
    <div slot="headline" class="flex flex-col items-stretch gap-4">
      <div class="flex items-center justify-between gap-6">
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

      <div class="flex items-center gap-2">
        <!-- Segmented control implemented with buttons to match existing MD components -->
        <div class="flex gap-2">
          <md-filled-tonal-button
            v-if="installedFilter === 'all'"
            @click.prevent="installedFilter = 'all'"
          >
            {{ $t("components.packages.form.filters.all") }}
          </md-filled-tonal-button>
          <md-outlined-button v-else @click.prevent="installedFilter = 'all'">
            {{ $t("components.packages.form.filters.all") }}
          </md-outlined-button>

          <md-filled-tonal-button
            v-if="installedFilter === 'installed'"
            @click.prevent="installedFilter = 'installed'"
          >
            {{ $t("components.packages.form.filters.installed") }}
          </md-filled-tonal-button>
          <md-outlined-button
            v-else
            @click.prevent="installedFilter = 'installed'"
          >
            {{ $t("components.packages.form.filters.installed") }}
          </md-outlined-button>

          <!-- <md-filled-tonal-button
            v-if="installedFilter === 'not-installed'"
            @click.prevent="installedFilter = 'not-installed'"
          >
            {{ $t("components.packages.form.filters.not-installed") }}
          </md-filled-tonal-button>
          <md-outlined-button
            v-else
            @click.prevent="installedFilter = 'not-installed'"
          >
            {{ $t("components.packages.form.filters.not-installed") }}
          </md-outlined-button> -->
        </div>
      </div>
    </div>

    <form
      ref="container"
      slot="content"
      method="dialog"
      class="flex h-full flex-col gap-4"
    >
      <template
        v-if="filteredPackages.length > 0"
        v-for="{ key, index } in virtualPackages"
        :key
      >
        <package-card
          v-if="filteredPackages[index]"
          :space-id
          :namespace
          :versioned-package="filteredPackages[index]"
          class="flex flex-col gap-2"
        />
      </template>

      <md-filled-card v-else class="label-large p-3">
        <div class="flex items-center justify-between">
          <span>
            {{ $t("components.packages.form.no-results") }}
          </span>

          <md-outlined-button @click.prevent="loadPackages">
            {{ $t("components.packages.form.reload") }}
          </md-outlined-button>
        </div>
      </md-filled-card>
    </form>
  </md-dialog>
</template>
