<script setup lang="ts">
// import Chat from "./chat.vue";
import Today from "./today.vue";
import Tasks from "./tasks.vue";
import Sync from "./sync.vue";

import type { MaterialSymbol } from "material-symbols";

defineProps<{ direction: "horizontal" | "vertical" }>();

const router = useRouter();

const route = useRoute();
const hash = computed(() => route.hash?.slice(1));

const sheet = ref(false);
// const sheet = useCookie<boolean | undefined>("side-bar-sheet");
// const activeItemIndex = useCookie("side-bar-active-item-index", {
//   default: () => 0,
// });

interface Item {
  name: string;
  icon?: MaterialSymbol;
  component?: Component;
}

const items: { [key: string]: Item } = {
  // chat: { name: "Chat", icon: "chat", component: Chat },
  today: { name: "Today", icon: "calendar_today", component: Today },
  tasks: { name: "Tasks", icon: "pinboard", component: Tasks },
  study: { name: "Study", icon: "school" },
  // quiz: { name: "Quiz", icon: "quiz" },
  sync: { name: "Sync", icon: "sync", component: Sync },
};

watchImmediate(hash, (hash) => {
  if (!hash) sheet.value = false;

  const item = items[hash];
  if (item) sheet.value = true;
});

function handleClick(id: string | number) {
  if (sheet.value && id === hash.value) sheet.value = false;
  else {
    router.push({ hash: "#" + id, query: route.query });
    sheet.value = true;
  }
}

whenever(logicNot(sheet), () =>
  router.replace({ hash: "", query: route.query }),
);

function preloadItem(item: Item) {
  preloadComponents(item.name);
}
</script>

<template>
  <m3-nav-rail v-if="direction == 'vertical'">
    <m3-nav-rail-item
      v-for="(item, id) in items"
      :key="id"
      :active="hash === id"
      @click="handleClick(id)"
    >
      <template v-if="item.icon" #leading>
        <m3-icon :name="item.icon" :fill="sheet && hash == id" />
      </template>

      {{ item.name }}
    </m3-nav-rail-item>
  </m3-nav-rail>

  <m3-nav-bar v-else>
    <m3-nav-bar-item
      v-for="(item, id) in items"
      :key="id"
      :active="hash === id"
      @hover="preloadItem(item)"
      @focus="preloadItem(item)"
      @click="handleClick(id)"
    >
      <template v-if="item.icon" #leading>
        <m3-icon :name="item.icon" :fill="sheet && hash == id" />
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

    <Primitive v-if="items[hash]" :id="hash" :as="items[hash]!.component" />
  </m3-side-sheet>
</template>
