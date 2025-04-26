<script setup lang="ts">
import symbols from "~/assets/symbols.json";
import type { MaterialSymbol } from "material-symbols";

import type { Space } from "~/composables/space";

import { UseVirtualList } from "@vueuse/components";

const space = defineModel<Space>();

const colors = [
  { name: "Red", hex: "#fb2c36" }, // oklch(.637 .237 25.331)
  { name: "Orange", hex: "#ff6900" }, // oklch(.705 .213 47.604)
  { name: "Amber", hex: "#fd9a00" }, // oklch(.769 .188 70.08)
  { name: "Yellow", hex: "#efb100" }, // oklch(.795 .184 86.047)
  { name: "Lime", hex: "#7ccf00" }, // oklch(.768 .233 130.85)
  { name: "Green", hex: "#00c951" }, // oklch(.723 .219 149.579)
  { name: "Emerald", hex: "#00bc7d" }, // oklch(.696 .17 162.48)
  { name: "Teal", hex: "#00bba7" }, // oklch(.704 .14 182.503)
  { name: "Cyan", hex: "#00b8db" }, // oklch(.715 .143 215.221)
  { name: "Sky", hex: "#00a6f4" }, // oklch(.685 .169 237.323)
  { name: "Blue", hex: "#2b7fff" }, // oklch(.623 .214 259.815)
  { name: "Indigo", hex: "#615fff" }, // oklch(.585 .233 277.117)
  { name: "Violet", hex: "#8e51ff" }, // oklch(.606 .25 292.717)
  { name: "Purple", hex: "#ad46ff" }, // oklch(.627 .265 303.9)
  { name: "Fuchsia", hex: "#e12afb" }, // oklch(.667 .295 322.15)
  { name: "Pink", hex: "#f6339a" }, // oklch(.656 .241 354.308)
  { name: "Rose", hex: "#ff2056" }, // oklch(.645 .246 16.439)
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
