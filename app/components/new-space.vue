<script setup lang="ts">
import symbols from "~/assets/symbols.json";
import type { MaterialSymbol } from "material-symbols";

import { UseVirtualList } from "@vueuse/components";

const groupedSymbols: string[][] = [];
for (const [i, symbol] of Object.entries(symbols)) {
  const group = Math.floor(Number(i) / 9);
  groupedSymbols[group] ||= [];
  groupedSymbols[group]!.push(symbol);
}

const open = defineModel();

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

const name = ref<string>("");
const color = ref<(typeof colors)[number]>(colors[0]!);
const icon = ref<MaterialSymbol>("home");

const iconSearch = ref<string>("");

const dark = useDark();
</script>

<template>
  <m3-theme :color="color.hex" harmonize :dark="dark">
    <md-dialog :open="open" @closed="open = false">
      <span slot="headline">New Space</span>

      <form slot="content" method="dialog" class="flex flex-col gap-8">

        <label class="flex gap-4">
          <md-outlined-text-field class="flex-1" label="Name" :value="name" @input="name = $event.target.value" />
        </label>

        <label>
          <span class="m3-label-large">Color</span>

          <div class="flex gap-2 px-20 flex-wrap items-center justify-center">
            <div class="w-12 h-12 relative cursor-pointer" v-for="c in colors" :key="c.name"
              :style="{ backgroundColor: c.hex }" @click="color = c">
              <md-ripple />
            </div>
          </div>
        </label>

        <m3-outlined-card class="px-4">
          <md-outlined-text-field class="w-full" label="Icon" type="search" @input="iconSearch = $event.target.value">
            <md-icon slot="leading-icon">search</md-icon>
          </md-outlined-text-field>


          <UseVirtualList :list="groupedSymbols" :options="{ itemHeight: 56 }" height="200px">
            <template #default="{ data: symbols }">
              <div class="flex gap-4 mb-4">
                <md-icon-button v-for="symbol in symbols" :key="symbol" @click.prevent="icon = symbol">
                  <m3-icon :name="symbol" />
                </md-icon-button>
              </div>
            </template>
          </UseVirtualList>
        </m3-outlined-card>

        <label>
          <span class="m3-label-large">Preview</span>

          <div class="flex justify-between items-end">

            <m3-nav-drawer-item class="w-84" @click.prevent>
              <template #leading>
                <m3-icon rounded :name="icon" class="text-m3-primary" />
              </template>

              {{ name }}
            </m3-nav-drawer-item>

            <md-text-button @click="open = false">Create</md-text-button>
          </div>
        </label>
      </form>
    </md-dialog>
  </m3-theme>
</template>
