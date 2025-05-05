<script lang="ts" setup>
import type { Package } from "~~/server/api/list-packages";

const { spaceId, versionedPackage } = defineProps<{
  spaceId: string;
  versionedPackage: Package[];
}>();
const emit = defineEmits<{
  (e: "install", pkg: Package): void;
  (e: "uninstall", pkg: Package): void;
}>();

const selectedIndex = ref(0);
const pkg = computed(() => versionedPackage[selectedIndex.value]!);

const installedPackages = await useInstalledPackages(spaceId);
const installedPackageByName = computed(() =>
  installedPackages.value.filter((pkgItem) => pkg.value.name === pkgItem.name),
);
const installedPackage = computed(() =>
  installedPackageByName.value.find(
    (pkgItem) => pkg.value.version === pkgItem.version,
  ),
);
</script>

<template>
  <m3-elevated-card :key="pkg.name" class="flex flex-col gap-4">
    <div class="flex justify-between gap-2">
      <div class="flex flex-col">
        <h1
          class="text-m3-on-surface-variant m3-title-large line-clamp-1 flex-1"
        >
          {{ pkg.name }}
        </h1>

        <div class="flex gap-2">
          <div
            v-for="(category, i) in pkg.categories"
            :key="i"
            class="bg-m3-surface-container text-m3-on-surface-variant rounded-xl px-2 py-1"
          >
            {{ category }}
          </div>
        </div>
      </div>

      <md-outlined-select
        :value="selectedIndex"
        @input="selectedIndex = $event.target.value"
      >
        <md-select-option
          v-for="({ version }, i) in versionedPackage"
          :key="i"
          :value="i"
          :selected="i === selectedIndex"
        >
          <span slot="headline">
            v{{ version }}

            <template
              v-if="
                installedPackageByName.find(
                  (pkgItem) => version === pkgItem.version,
                )
              "
            >
              (Installed)
            </template>
          </span>
        </md-select-option>
      </md-outlined-select>
    </div>

    <div class="flex flex-1 flex-col gap-2">
      <span
        class="text-m3-on-surface-variant m3-body-large line-clamp-2 h-[2lh]"
        :title="pkg.description"
      >
        {{ pkg.description }}
      </span>
    </div>

    <div class="flex gap-2">
      <div class="flex-[2]" />
      <md-outlined-button
        v-if="installedPackage"
        class="flex-[3]"
        @click.prevent="emit('uninstall', pkg)"
      >
        Uninstall
      </md-outlined-button>
      <md-outlined-button
        v-else
        class="flex-[3]"
        @click.prevent="emit('install', pkg)"
      >
        Install
      </md-outlined-button>
    </div>
  </m3-elevated-card>
</template>
