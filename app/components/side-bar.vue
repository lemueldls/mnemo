<script setup lang="ts">
// import Chat from "./chat.vue";
import Today from "./today.vue";
import Tasks from "./tasks.vue";
import Study from "./study.vue";
import Sync from "./sync.vue";

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
  study: {
    name: t("components.side-bar.study"),
    icon: "school",
    component: Study,
  },
  sync: { name: t("components.side-bar.sync"), icon: "sync", component: Sync },
};

const router = useRouter();
const route = useRoute();

const hash = computed(() => route.hash?.slice(1));
const sheet = ref(!!items[hash.value]);

watchImmediate(hash, (hash) => {
  if (!hash) sheet.value = false;

  const item = items[hash];
  if (item) sheet.value = true;
});

let manuallyOpened = false;
whenever(sheet, () => {
  manuallyOpened = true;
});

whenever(logicNot(sheet), () => {
  if (manuallyOpened) {
    manuallyOpened = false;
    router.back();
  } else router.replace({ ...route, hash: "" });
});

function handleClick(id: string | number) {
  if (sheet.value && id === hash.value) sheet.value = false;
  else {
    if (manuallyOpened) router.replace({ ...route, hash: "#" + id });
    else router.push({ ...route, hash: "#" + id });
    sheet.value = true;
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
        <mx-icon :name="item.icon" :fill="sheet && hash == id" />
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
        <mx-icon :name="item.icon" :fill="sheet && hash == id" />
      </template>

      {{ item.name }}
    </mx-nav-bar-item>
  </mx-nav-bar>

  <mx-side-sheet v-model="sheet" class="w-80">
    <div class="flex justify-end">
      <md-icon-button @click="sheet = false">
        <md-icon>close</md-icon>
      </md-icon-button>
    </div>

    <Primitive v-if="items[hash]" :as="items[hash]!.component" />
  </mx-side-sheet>
</template>
