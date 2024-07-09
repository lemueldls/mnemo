<script setup lang="ts">
import type { MaterialSymbol } from "material-symbols";

const runtimeConfig = useRuntimeConfig();
const { platform } = runtimeConfig.app;

if (platform)
  onMounted(async () => {
    const { invoke } = await import("@tauri-apps/api/core");

    await invoke("close_splashscreen");
  });

const color = "#16161d";
const dark = useDark();

const { desktop } = useBreakpoints(breakpointsM3);

const spaces = listSpaces();

const drawer = ref<boolean>();
const settings = ref<boolean>();
const packagesVisible = ref<boolean>();

interface Page {
  path: string;
  name: string;
  icon: MaterialSymbol;
}

const pages: Page[] = [
  { path: "/", name: "Home", icon: "home" },
  { path: "/calendar", name: "Calendar", icon: "calendar_today" },
];

const name = useLocalStorage("name", "");
</script>

<template>
  <m3-theme :color="color" :dark="dark" class="absolute inset-0 h-full">
    <m3-page>
      <m3-nav-drawer v-model="drawer">
        <h1 id="header">cortyp</h1>

        <nuxt-link v-for="page in pages" :key="page.path" :to="page.path">
          <m3-nav-drawer-item :active="$route.path === page.path" :style="{
    fontVariationSettings: `'FILL' ${page.path === $route.path ? 1 : 0
      }`,
  }">
            <template #leading>
              <md-icon>
                {{ page.icon }}
              </md-icon>
            </template>

            {{ page.name }}
          </m3-nav-drawer-item>
        </nuxt-link>

        <md-divider class="my-2 px-4" />

        <h3 class="mt-2 p-4 pt-0 text-m3-on-surface-variant m3-title-small">
          Spaces
        </h3>

        <nuxt-link v-for="(space, name) in spaces" :key="name" :to="`/space?id=${name}`">
          <m3-theme :color="space.color" harmonize :dark="dark">
            <m3-nav-drawer-item>
              <template #leading>
                <m3-icon rounded :name="space.icon" class="text-m3-primary" />
              </template>

              {{ name }}
            </m3-nav-drawer-item>
          </m3-theme>
        </nuxt-link>
      </m3-nav-drawer>

      <div class="flex flex-1 flex-col">
        <m3-top-app-bar>
          <template v-if="!desktop" #leading>
            <md-icon-button @click="drawer = !drawer">
              <md-icon>menu</md-icon>
            </md-icon-button>
          </template>

          <template #trailing>
            <md-icon-button @click="settings = true">
              <md-icon>settings</md-icon>
            </md-icon-button>
          </template>
        </m3-top-app-bar>

        <div class="m-6 flex flex-1 flex-col overflow-hidden">
          <slot />
        </div>
      </div>

      <md-dialog :open="settings" @closed="settings = false">
        <span slot="headline">Settings</span>

        <form slot="content" method="dialog" class="flex flex-col gap-4">
          <label class="flex items-center justify-between gap-4">
            Dark Theme

            <md-switch aria-label="Dark Theme" icons :selected="dark" @change="dark = $event.target.selected" />
          </label>

          <label class="flex items-center justify-between gap-4">
            <md-outlined-text-field label="Name" :value="name" @input="name = $event.target.value" />
          </label>
        </form>
      </md-dialog>

      <packages v-model="packagesVisible" />

      <side-bar />

      <!-- <m3-side-sheet class="w-100">
        <m3-daily-calendar />
      </m3-side-sheet> -->
    </m3-page>
  </m3-theme>
</template>

<style>
#header {
  @apply px-4 pt-4 pb-6 text-m3-primary m3-title-large;

  font-family: "Iosevka Quasi Custom", sans-serif;
}
</style>
