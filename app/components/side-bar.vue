<script setup lang="ts">
// import Chat from "./chat.vue";
import Study from "./study.vue";
import Storage from "./storage.vue";
import Tasks from "./tasks.vue";
import Today from "./today.vue";

import type { MaterialSymbol } from "material-symbols";

defineProps<{ direction: "horizontal" | "vertical" }>();

const { t } = useI18n();

interface Item {
  name: string;
  icon?: MaterialSymbol;
  component?: Component;
}

const items: { [key: string]: Item } = {
  // chat: { name: t("components.side-bar.chat"), icon: "chat", component: Chat },
  today: {
    name: t("components.side-bar.today"),
    icon: "calendar_today",
    component: Today,
  },
  tasks: {
    name: t("components.side-bar.tasks"),
    icon: "pinboard",
    component: Tasks,
  },
  // study: {
  //   name: t("components.side-bar.study"),
  //   icon: "school",
  //   component: Study,
  // },
  storage: {
    name: t("components.side-bar.storage"),
    icon: "home_storage",
    component: Storage,
  },
};

const route = useRoute();
const router = useRouter();

const hash = computed(() => route.hash?.slice(1));
const sheetOpened = ref(!!items[hash.value]);

watchImmediate(hash, (hash) => {
  if (!hash) sheetOpened.value = false;

  const item = items[hash];
  if (item) sheetOpened.value = true;
});

let manuallyOpened = false;

watch(sheetOpened, (sheet) => {
  if (sheet) manuallyOpened = true;
  else if (manuallyOpened) {
    manuallyOpened = false;
    router.back();
  } else router.replace({ ...route, hash: "" });
});

watch(
  () => route.query,
  () => {
    manuallyOpened = false;
  },
);

function handleClick(id: string | number) {
  if (sheetOpened.value && id === hash.value) sheetOpened.value = false;
  else {
    if (manuallyOpened) router.replace({ ...route, hash: "#" + id });
    else router.push({ ...route, hash: "#" + id });
    sheetOpened.value = true;
  }
}

function preloadItem(item: Item) {
  preloadComponents(item.name);
}
</script>

<template>
  <mx-nav-rail v-if="direction == 'vertical'">
    <mx-nav-rail-item
      v-for="(item, id) in items"
      :key="id"
      :active="hash === id"
      @click="handleClick(id)"
    >
      <template v-if="item.icon" #leading>
        <mx-icon :name="item.icon" :fill="sheetOpened && hash == id" />
      </template>

      {{ item.name }}
    </mx-nav-rail-item>
  </mx-nav-rail>

  <mx-nav-bar v-else>
    <mx-nav-bar-item
      v-for="(item, id) in items"
      :key="id"
      :active="hash === id"
      @hover="preloadItem(item)"
      @focus="preloadItem(item)"
      @click="handleClick(id)"
    >
      <template v-if="item.icon" #leading>
        <mx-icon :name="item.icon" :fill="sheetOpened && hash == id" />
      </template>

      {{ item.name }}
    </mx-nav-bar-item>
  </mx-nav-bar>

  <mx-side-sheet v-model="sheetOpened" class="w-80">
    <div class="mb-3 flex justify-between">
      <h3 v-if="items[hash]" class="headline-medium text-on-surface-variant">
        {{ items[hash]!.name }}
      </h3>

      <md-icon-button @click="sheetOpened = false">
        <md-icon>close</md-icon>
      </md-icon-button>
    </div>

    <Primitive v-if="items[hash]" :as="items[hash]!.component" />
  </mx-side-sheet>
</template>
