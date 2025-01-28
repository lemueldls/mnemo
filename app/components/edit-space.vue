<script setup lang="ts">
import symbols from "~/assets/symbols.json";
import type { MaterialSymbol } from "material-symbols";

import type { Space } from "~/composables/space";

import { UseVirtualList } from "@vueuse/components";

const space = defineModel<Space>();

const colors = [
  { name: "Red", hex: "#ef4444" },
  { name: "Orange", hex: "#f97316" },
  { name: "Amber", hex: "#f59e0b" },
  { name: "Yellow", hex: "#eab308" },
  { name: "Lime", hex: "#84cc16" },
  { name: "Green", hex: "#22c55e" },
  { name: "Emerald", hex: "#10b981" },
  { name: "Teal", hex: "#14b8a6" },
  { name: "Cyan", hex: "#06b6d4" },
  { name: "Sky", hex: "#0ea5e9" },
  { name: "Blue", hex: "#3b82f6" },
  { name: "Indigo", hex: "#6366f1" },
  { name: "Violet", hex: "#8b5cf6" },
  { name: "Purple", hex: "#a855f7" },
  { name: "Fuchsia", hex: "#d946ef" },
  { name: "Pink", hex: "#ec4899" },
  { name: "Rose", hex: "#f43f5e" },
];

// const name = ref<string>("");
// const color = ref<(typeof colors)[number]>(colors[0]!);
// const icon = ref<MaterialSymbol>("home");

const iconSearch = ref<string>("");
const iconContainerRef = useTemplateRef("icon-container");

watch(iconSearch, () => {
  const iconContainer = iconContainerRef.value;
  if (iconContainer) iconContainer.scrollTo(0);
});

const filteredSymbols = computed(() => {
  const search = iconSearch.value;

  return search
    ? symbols.filter((symbol) => symbol.includes(search.toLowerCase()))
    : symbols;
});

const groupedSymbols: string[][] = computed(() => {
  const groups = [];
  const entries = Object.entries(filteredSymbols.value);

  for (const [i, symbol] of entries) {
    const group = Math.floor(Number(i) / 9);
    groups[group] ||= [];
    groups[group]!.push(symbol);
  }

  return groups;
});
</script>

<template>
  <form method="dialog" class="flex flex-col gap-8">
    <label class="flex gap-4">
      <md-outlined-text-field
        class="flex-1"
        label="Name"
        :value="space.name"
        @input="space.name = $event.target.value"
      />
    </label>

    <label>
      <span class="m3-label-large">Color</span>

      <div class="flex gap-2 px-20 flex-wrap items-center justify-center">
        <div
          v-for="c in colors"
          :key="c.name"
          class="w-12 h-12 relative cursor-pointer"
          :style="{ backgroundColor: c.hex }"
          :title="c.name"
          @click="space.color = c.hex"
        >
          <md-ripple />
        </div>
      </div>
    </label>

    <m3-outlined-card class="flex flex-col gap-4 px-4">
      <md-outlined-text-field
        class="w-full"
        label="Icon"
        type="search"
        @input="iconSearch = $event.target.value"
      >
        <md-icon slot="leading-icon">search</md-icon>
      </md-outlined-text-field>

      <UseVirtualList
        ref="icon-container"
        :list="groupedSymbols"
        :options="{ itemHeight: 56 }"
        height="200px"
      >
        <template #default="{ data: symbols }">
          <div class="flex gap-4 mb-4">
            <md-icon-button
              v-for="symbol in symbols"
              :key="symbol"
              :title="symbol"
              @click.prevent="space.icon = symbol"
            >
              <m3-icon :name="symbol" />
            </md-icon-button>
          </div>
        </template>
      </UseVirtualList>
    </m3-outlined-card>

    <label>
      <span class="m3-label-large">Preview</span>

      <div class="flex justify-between items-end">
        <m3-nav-drawer-item class="w-84">
          <template #leading>
            <m3-icon rounded :name="space.icon" class="text-m3-primary" />
          </template>

          {{ space.name }}
        </m3-nav-drawer-item>

        <slot name="actions" />
      </div>
    </label>
  </form>
</template>
