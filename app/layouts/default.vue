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

// const { auth } = useSupabaseClient();
// const user = useSupabaseUser();

// watchEffect(() => {
//   console.log({ user: user.value });
// });

// if (!user.value) await auth.sign();

const name = await useStorageItem("name", "");
</script>

<template>
  <m3-theme :color="color" :dark="dark" class="absolute inset-0 h-full">
    <m3-page>
      <m3-nav-drawer v-model="drawerOpen">
        <div id="header" class="flex items-center justify-between">
          <!-- <div class="flex flex-col">
            <span class="m3-title-large">{{ user.email }}</span>
            <span class="m3-label-large">@{{ user.id }}</span>
          </div> -->

          mnemo
        </div>

        <nuxt-link v-for="page in pages" :key="page.path" :to="page.path">
          <m3-nav-drawer-item
            :active="$route.path === page.path"
            :style="{
              fontVariationSettings: `'FILL' ${
                page.path === $route.path ? 1 : 0
              }`,
            }"
          >
            <template #leading>
              <md-icon>
                {{ page.icon }}
              </md-icon>
            </template>

            {{ page.name }}
          </m3-nav-drawer-item>
        </nuxt-link>

        <md-divider class="my-2 px-4" />

        <h3
          class="flex items-center justify-between p-4 pb-2 pt-0 text-m3-on-surface-variant m3-title-small"
        >
          Spaces

          <md-icon-button @click="newSpaceOpen = true">
            <md-icon>add</md-icon>
          </md-icon-button>
        </h3>

        <nuxt-link
          v-for="[id, space] in spaces"
          :key="id"
          :to="`/space?id=${id}`"
        >
          <m3-theme :color="space.color" harmonize :dark="dark">
            <m3-nav-drawer-item>
              <template #leading>
                <m3-icon rounded :name="space.icon" class="text-m3-primary" />
              </template>

              {{ space.name }}
            </m3-nav-drawer-item>
          </m3-theme>
        </nuxt-link>
      </m3-nav-drawer>

      <div class="flex flex-1 flex-col">
        <m3-top-app-bar>
          <template v-if="!desktop" #leading>
            <md-icon-button @click="drawerOpen = !drawerOpen">
              <md-icon>menu</md-icon>
            </md-icon-button>
          </template>

          <template #trailing>
            <md-icon-button @click="settingsOpen = true">
              <md-icon>settings</md-icon>
            </md-icon-button>
          </template>
        </m3-top-app-bar>

        <div class="m-6 flex flex-1 flex-col">
          <slot />
        </div>

        <side-bar direction="horizontal" v-if="mobile" />
      </div>

      <md-dialog :open="settingsOpen" @closed="settingsOpen = false">
        <span slot="headline">Settings</span>

        <form slot="content" method="dialog" class="flex flex-col gap-4">
          <label class="flex items-center justify-between gap-4">
            Dark Theme

            <md-switch
              aria-label="Dark Theme"
              icons
              :selected="dark"
              @change="dark = $event.target.selected"
            />
          </label>

          <label class="flex items-center justify-between gap-4">
            <md-outlined-text-field
              label="Name"
              :value="name"
              @input="name = $event.target.value"
            />
          </label>
        </form>
      </md-dialog>

      <new-space v-model="newSpaceOpen" />

      <side-bar direction="vertical" v-if="!mobile" />
    </m3-page>
  </m3-theme>
</template>

<style>
#header {
  @apply px-4 pt-4 pb-6 text-m3-primary-fixed-dim m3-title-large;

  /* font-family: "Iosevka Quasi Custom", sans-serif; */
  font-family: "Iosevka Book Web", sans-serif;
}
</style>
