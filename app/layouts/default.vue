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

const spaces = await useSpaces();

const { medium, extraLarge } = useBreakpoints(breakpointsM3);

const drawerOpen = ref<boolean>();
const settingsOpen = ref<boolean>();
const newSpaceOpen = ref<boolean>();

const { ready, loggedIn } = useUserSession();

interface Page {
  path: string;
  name: string;
  icon: MaterialSymbol;
}

const pages: Page[] = [
  { path: "/", name: "Home", icon: "home" },
  { path: "/calendar", name: "Calendar", icon: "calendar_today" },
];

const { clear, session, user } = useUserSession();

const name = await useStorageItem("name", "");
</script>

<template>
  <m3-theme :color="color" :dark="dark" class="absolute inset-0 h-full">
    <m3-page>
      <m3-nav-drawer v-model="drawerOpen">
        <div id="header" class="flex items-center justify-between">
          <div class="flex flex-col">
            <!-- <span class="m3-title-large">{{ user.email }}</span>
            <span class="m3-label-large">@{{ user.id }}</span> -->
            <!-- <span class="m3-title-medium">lemueldls@pm.me</span>
            <span class="m3-label-medium">@c0pw798239gft7cktutyt</span> -->
          </div>

          <!-- <div>
            <h2 class="text-m3-primary m3-title-medium">mnemo</h2>
          </div> -->

          <md-icon-button>
            <md-icon>sync</md-icon>
          </md-icon-button>
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
          v-for="(space, id) in spaces"
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
            'mb-3 flex flex-1 flex-col h-full overflow-auto',
            { 'ml-3': !extraLarge },
          ]"
        >
          <slot />
        </div>

        <side-bar v-if="!medium" direction="horizontal" />
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

      <side-bar v-if="medium" direction="vertical" />
    </m3-page>
  </m3-theme>
</template>

<style>
#header {
  @apply px-4 pt-2 pb-4;

  /* font-family: "Iosevka Quasi Custom", sans-serif; */
  font-family: "Iosevka Book Web", sans-serif;
}
</style>
