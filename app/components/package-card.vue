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
  <md-elevated-card :key="pkg.name" class="flex flex-col gap-4 p-4">
    <div class="medium:flex-row flex flex-col justify-between gap-2">
      <div class="flex flex-col">
        <h1 class="text-on-surface-variant title-large line-clamp-1 flex-1">
          {{ pkg.name }}
        </h1>

        <div class="flex gap-2">
          <div
            v-for="(category, i) in pkg.categories"
            :key="i"
            class="bg-surface-container text-on-surface-variant rounded-xl px-2 py-1"
          >
            {{ category }}
          </div>
        </div>
      </div>

      <md-outlined-select
        :label="$t('components.package-card.version')"
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
              {{ $t("components.package-card.installed") }}
            </template>
          </span>
        </md-select-option>
      </md-outlined-select>
    </div>

    <div class="flex flex-1 flex-col gap-2">
      <span
        class="text-on-surface-variant body-large line-clamp-2 h-[2lh]"
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
        {{ $t("components.package-card.uninstall") }}
      </md-outlined-button>
      <md-filled-tonal-button
        v-else
        class="flex-[3]"
        @click.prevent="emit('install', pkg)"
      >
        {{ $t("components.package-card.install") }}
      </md-filled-tonal-button>
    </div>
  </md-elevated-card>
</template>
