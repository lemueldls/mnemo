<script setup lang="ts">
import { UseVirtualList } from "@vueuse/components";

import type { Space } from "~/composables/spaces";

import symbols from "~/assets/symbols.json";
// import type { MaterialSymbol } from "material-symbols";

const space = defineModel<Space>({ required: true });

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
const iconContainerRef = useTemplateRef<HTMLElement>("icon-container");

watch(iconSearch, () => {
  const iconContainer = iconContainerRef.value;
  if (iconContainer) iconContainer.scrollTo(0, 0);
});

const filteredSymbols = computed(() => {
  const search = iconSearch.value;

  return search
    ? symbols.filter((symbol) => symbol.includes(search.toLowerCase()))
    : symbols;
});

const groupedSymbols = computed(() => {
  const groups: string[][] = [];
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
  <label class="flex gap-4">
    <md-outlined-text-field
      class="flex-1"
      :label="$t('components.edit-space.form.name')"
      :value="space.name"
      required
      @input="space.name = $event.target.value"
    />
  </label>

  <label>
    <span class="label-large">
      {{ $t("components.edit-space.form.color") }}
    </span>

    <div class="medium:mx-20 flex flex-wrap items-center justify-center gap-2">
      <div
        v-for="c in colors"
        :key="c.name"
        class="relative h-12 w-12 cursor-pointer"
        :style="{ backgroundColor: c.hex }"
        :title="c.name"
        @click="space.color = c.hex"
      >
        <md-ripple />
      </div>
    </div>
  </label>

  <md-outlined-card class="flex flex-col gap-4 p-4">
    <md-outlined-text-field
      class="w-full"
      :label="$t('components.edit-space.form.icon')"
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
      <template #default="{ data: symbolGroup }">
        <div class="mb-4 flex gap-4">
          <component
            :is="
              space.icon === symbol
                ? 'md-filled-tonal-icon-button'
                : 'md-icon-button'
            "
            v-for="symbol in symbolGroup"
            :key="symbol"
            :title="symbol"
            toggle
            :selected="space.icon === symbol"
            @click.prevent="space.icon = symbol"
          >
            <mx-icon :name="symbol" />
          </component>
        </div>
      </template>
    </UseVirtualList>
  </md-outlined-card>

  <div class="flex items-end justify-end">
    <slot name="actions" />
  </div>
</template>
