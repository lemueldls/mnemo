<script setup lang="ts">
import type { MaterialSymbol } from "material-symbols";

const route = useRoute();

const spaces = await useSpaces();

const { medium, extraLarge } = useBreakpoints(breakpointsM3);

const drawerOpen = ref<boolean>(false);
const settingsOpen = ref<boolean>(false);
const newSpaceOpen = useNewSpaceOpen();

interface Page {
  path: string;
  name: string;
  icon: MaterialSymbol;
}

const { t } = useSharedI18n();

const pages: Page[] = [
  { path: "/", name: t("layouts.default.nav.home"), icon: "home" },
  {
    path: "/calendar",
    name: t("layouts.default.nav.calendar"),
    icon: "date_range",
  },
];
</script>

<template>
  <mx-theme color="#16161d" class="absolute inset-0 h-full">
    <mx-page>
      <mx-nav-drawer v-model="drawerOpen">
        <div id="header" class="flex items-center justify-between">
          <!-- <div>
            <h2 class="text-primary title-medium">mnemo</h2>
          </div> -->

          <!-- <md-icon-button>
            <md-icon>sync</md-icon>
          </md-icon-button> -->
        </div>

        <nuxt-link v-for="page in pages" :key="page.path" :to="page.path">
          <mx-nav-drawer-item
            :active="route.path === page.path"
            :style="{
              fontVariationSettings: `'FILL' ${
                page.path === route.path ? 1 : 0
              }`,
            }"
          >
            <template #leading>
              <div class="size-6">
                <md-icon v-if="page.icon">
                  {{ page.icon }}
                </md-icon>
              </div>
            </template>

            {{ page.name }}
          </mx-nav-drawer-item>
        </nuxt-link>

        <md-divider class="my-2 px-4" />

        <h3
          class="text-on-surface-variant title-small flex items-center justify-between p-4 pb-2 pt-0"
        >
          {{ t("layouts.default.spaces") }}

          <md-icon-button @click="newSpaceOpen = true">
            <md-icon>add</md-icon>
          </md-icon-button>
        </h3>

        <nuxt-link
          v-for="(space, id) in spaces"
          :key="id"
          :to="`/space?id=${id}`"
        >
          <mx-theme :color="space.color" harmonize>
            <mx-nav-drawer-item>
              <template #leading>
                <div class="size-6">
                  <mx-icon
                    v-if="space.icon"
                    :name="space.icon"
                    class="text-primary"
                  />
                </div>
              </template>

              {{ space.name }}
            </mx-nav-drawer-item>
          </mx-theme>
        </nuxt-link>

        <!-- <template #actions>
          <md-outlined-button @click="clear"> Logout </md-outlined-button>
        </template> -->
      </mx-nav-drawer>

      <div class="flex flex-1 flex-col overflow-hidden">
        <mx-top-app-bar>
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
        </mx-top-app-bar>

        <div
          :class="[
            'flex h-full flex-1 flex-col overflow-auto',
            { 'pl-3': !extraLarge, 'pr-3': !medium },
          ]"
        >
          <div class="medium:h-full pb-3">
            <slot />
          </div>
        </div>

        <side-bar v-if="!medium" direction="horizontal" />
      </div>

      <settings v-model="settingsOpen" />

      <new-space v-model="newSpaceOpen" />

      <new-task />
      <edit-task />

      <side-bar v-if="medium" direction="vertical" />
    </mx-page>
  </mx-theme>
</template>

<style>
#header {
  @apply px-4 pb-4 pt-2;

  font-family: var(--font-mono);
}
</style>
