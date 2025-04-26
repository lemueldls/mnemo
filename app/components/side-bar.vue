<script setup lang="ts">
// import Chat from "./chat.vue";
import Today from "./today.vue";
import Tasks from "./tasks.vue";
import Sync from "./sync.vue";

import type { MaterialSymbol } from "material-symbols";

defineProps<{ direction: "horizontal" | "vertical" }>();

const sheet = useCookie<boolean | undefined>("side-bar-sheet");
const activeItemIndex = useCookie("side-bar-active-item-index", {
  default: () => 0,
});

interface Item {
  icon?: MaterialSymbol;
  name: string;
  component?: Component;
}

const items: Item[] = [
  // { icon: "chat", name: "Chat", component: Chat },
  { icon: "calendar_today", name: "Today", component: Today },
  { icon: "pinboard", name: "Tasks", component: Tasks },
  { icon: "school", name: "Study" },
  // { icon: "quiz", name: "Quiz" },
  { icon: "sync", name: "Sync", component: Sync },
];

function handleClick(index: number) {
  if (sheet.value && activeItemIndex.value === index) sheet.value = false;
  else {
    activeItemIndex.value = index;
    sheet.value = true;
  }
}

function preloadItem(item: Item) {
  preloadComponents(item.name);
}
</script>

<template>
  <m3-nav-rail v-if="direction == 'vertical'">
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
  <m3-nav-bar v-else>
    <m3-nav-bar-item
      v-for="(item, index) in items"
      :key="index"
      :active="sheet && index === activeItemIndex"
      @hover="preloadItem(item)"
      @focus="preloadItem(item)"
      @click="handleClick(index)"
    >
      <template v-if="item.icon" #leading>
        <m3-icon :name="item.icon" :fill="sheet && index === activeItemIndex" />
      </template>

      {{ item.name }}
    </m3-nav-bar-item>
  </m3-nav-bar>

  <m3-side-sheet v-model="sheet" class="w-80">
    <div class="flex justify-end">
      <md-icon-button @click="sheet = false">
        <md-icon>close</md-icon>
      </md-icon-button>
    </div>

    <component :is="items[activeItemIndex]!.component" />
  </m3-side-sheet>
</template>
