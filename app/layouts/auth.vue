<script setup lang="ts">
// import "mnemo-wasm";

import type { MaterialSymbol } from "material-symbols";

// const runtimeConfig = useRuntimeConfig();
// const { platform } = runtimeConfig.app;

// if (platform && import.meta.env.PROD)
//   onMounted(async () => {
//     await checkForAppUpdates();
//   });

const color = "#16161d";
const dark = useDark();

const spaces = await listSpaces();

const { desktop, smallerOrEqual } = useBreakpoints(breakpointsM3);
const mobile = smallerOrEqual("medium");

const drawerOpen = ref<boolean>();
const settingsOpen = ref<boolean>();
const newSpaceOpen = ref<boolean>();

interface Page {
  path: string;
  name: string;
  icon: MaterialSymbol;
}

const pages: Page[] = [
  { path: "/", name: "Home", icon: "home" },
  { path: "/calendar", name: "Calendar", icon: "calendar_today" },
];

const { auth } = useSupabaseClient();
const user = useSupabaseUser();

// if (!user.value) await auth.sign();

const name = useLocalStorage("name", "");
</script>

<template>
  <m3-theme :color="color" :dark="dark" class="absolute inset-0 h-full">
    <m3-page class="flex-1 flex flex-col items-center justify-center">
      <slot />
    </m3-page>
  </m3-theme>
</template>
