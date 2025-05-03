<script lang="ts" setup>
import type { Package } from "~~/server/api/list-packages";

const { packages } = defineProps<{ packages: Package[] }>();
const emit = defineEmits<{ (e: "install", pkg: Package): void }>();

const selectedIndex = ref(0);
const pkg = computed(() => packages[selectedIndex.value]!);

const installedPackages = useInstalledPackages();
</script>

<template>
  <m3-elevated-card :key="pkg.name" class="flex flex-col gap-2">
    <h1 class="text-m3-on-surface-variant m3-title-large">
      {{ pkg.name }} {{ pkg.version }}
    </h1>

    <md-outlined-select
      :value="selectedIndex"
      @input="selectedIndex = $event.target.value"
    >
      <md-select-option
        v-for="({ version }, i) in packages"
        :key="i"
        :value="i"
        :selected="i === selectedIndex"
      >
        <span slot="headline">{{ version }}</span>
      </md-select-option>
    </md-outlined-select>

    <div class="flex flex-1 flex-col gap-2">
      <span class="text-m3-on-surface-variant m3-body-large">
        {{ pkg.description }}
      </span>
    </div>

    <div class="flex gap-2">
      <div class="flex-[2]" />
      <md-outlined-button
        class="flex-[3]"
        @click.prevent="emit('install', pkg)"
      >
        {{ installedPackages.has(pkg.name) ? "Installed" : "Install" }}
      </md-outlined-button>
    </div>
  </m3-elevated-card>
</template>
