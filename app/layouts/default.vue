<script setup lang="ts">
// import "mnemo-wasm";

import type { MaterialSymbol } from "material-symbols";

// const runtimeConfig = useRuntimeConfig();
// const { platform } = runtimeConfig.public;

// if (platform && import.meta.env.PROD)
//   onMounted(async () => {
//     await checkForAppUpdates();
//   });

const route = useRoute();

const spaces = await useSpaces();

const { medium, extraLarge } = useBreakpoints(breakpointsM3);

const drawerOpen = ref<boolean>(false);
const settingsOpen = ref<boolean>(false);
const newSpaceOpen = useNewSpaceOpen();

// const { ready, loggedIn } = useUserSession();

interface Page {
  path: string;
  name: string;
  icon: MaterialSymbol;
}

const pages: Page[] = [
  { path: "/", name: "Home", icon: "home" },
  { path: "/calendar", name: "Calendar", icon: "calendar_today" },
];

const { user } = useAuth();
</script>

<template>
  <m3-theme color="#16161d" class="absolute inset-0 h-full">
    <m3-page>
      <m3-nav-drawer v-model="drawerOpen">
        <div id="header" class="flex items-center justify-between">
          <div class="flex flex-col">
            User: {{ user?.name }} Email: {{ user?.email }}
          </div>

          <!-- <div>
            <h2 class="text-m3-primary m3-title-medium">mnemo</h2>
          </div> -->

          <!-- <md-icon-button>
            <md-icon>sync</md-icon>
          </md-icon-button> -->
        </div>

        <nuxt-link v-for="page in pages" :key="page.path" :to="page.path">
          <m3-nav-drawer-item
            :active="route.path === page.path"
            :style="{
              fontVariationSettings: `'FILL' ${
                page.path === route.path ? 1 : 0
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
          class="text-m3-on-surface-variant m3-title-small flex items-center justify-between p-4 pb-2 pt-0"
        >
          Spaces

          <md-icon-button @click="newSpaceOpen = true">
            <md-icon>add</md-icon>
          </md-icon-button>
        </h3>

        <nuxt-link
          v-for="(space, id) in spaces"
          :key="id"
          :to="`/space?id=${id}`"
        >
          <m3-theme :color="space.color" harmonize>
            <m3-nav-drawer-item>
              <template #leading>
                <m3-icon :name="space.icon" class="text-m3-primary" />
              </template>

              {{ space.name }}
            </m3-nav-drawer-item>
          </m3-theme>
        </nuxt-link>

        <!-- <template #actions>
          <md-outlined-button @click="clear"> Logout </md-outlined-button>
        </template> -->
      </m3-nav-drawer>

      <div class="flex flex-1 flex-col">
        <m3-top-app-bar>
          <template v-if="!extraLarge" #leading>
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

        <div
          :class="[
            'mb-3 flex h-full flex-1 flex-col overflow-auto',
            { 'ml-3': !extraLarge, 'mr-3': !medium },
          ]"
        >
          <slot />
        </div>

        <side-bar v-if="!medium" direction="horizontal" />
      </div>

      <settings v-model="settingsOpen" />
      <new-space v-model="newSpaceOpen" />

      <side-bar v-if="medium" direction="vertical" />
    </m3-page>
  </m3-theme>
</template>

<style>
#header {
  @apply px-4 pb-4 pt-2;

  font-family: "Iosevka Book", sans-serif;
}
</style>
