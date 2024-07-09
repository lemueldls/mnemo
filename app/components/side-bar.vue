<script setup lang="ts">
import Today from "./today.vue";

import type { MaterialSymbol } from "material-symbols";

const sheet = useCookie<boolean | undefined>("side-bar-sheet");
const activeItemIndex = useCookie("side-bar-active-item-index", {
  default: () => 0,
});

interface Item {
  icon?: MaterialSymbol;
  name?: string;
  component?: Component;
}

const items: Item[] = [
  { icon: "chat", name: "Chat" },
  { icon: "calendar_today", name: "Today", component: Today },
  { icon: "school", name: "Study" },
  { icon: "quiz", name: "Quiz" },
];

function handleClick(index: number) {
  if (sheet.value && activeItemIndex.value === index) sheet.value = false;
  else {
    activeItemIndex.value = index;
    sheet.value = true;
  }
}
</script>

<template>
  <m3-nav-rail>
    <m3-nav-rail-item
      v-for="(item, index) in items"
      :key="index"
      :active="sheet && index === activeItemIndex"
      @click="handleClick(index)"
    >
      <template v-if="item.icon" #leading>
        <m3-icon :name="item.icon" :fill="sheet && index === activeItemIndex" />
      </template>

      {{ item.name }}
    </m3-nav-rail-item>
  </m3-nav-rail>

  <m3-side-sheet v-model="sheet" class="w-80">
    <div class="flex justify-end">
      <md-icon-button @click="sheet = false">
        <md-icon>close</md-icon>
      </md-icon-button>
    </div>

    <component :is="items[activeItemIndex].component" />
  </m3-side-sheet>
</template>
