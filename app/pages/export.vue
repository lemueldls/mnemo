<script setup lang="ts">
definePageMeta({ layout: "blank" });

const spaceId = usePageRouteQuery("space");

const spaces = await useSpaces();
const space = computed(() => spaces.value[spaceId.value]!);

useHead({ title: () => space.value.name });

const dark = useDark();
</script>

<template>
  <mx-theme :color="space.color">
    <div
      class="text-on-background bg-background flex min-h-screen flex-1 flex-col pt-16"
    >
      <mx-top-app-bar compress class="print:hidden! z-1 fixed! top-0">
        <template #leading>
          <nuxt-link :to="`/space?id=${spaceId}`">
            <md-icon-button>
              <md-icon>arrow_back</md-icon>
            </md-icon-button>
          </nuxt-link>
        </template>

        <div class="flex flex-1 items-center justify-center gap-2">
          <md-icon v-if="space.icon">{{ space.icon }}</md-icon>

          <h1 class="line-clamp-1" :title="space.name">
            {{ space.name }}
          </h1>
        </div>

        <template #trailing>
          <md-icon-button @click="dark = !dark">
            <md-icon v-if="dark">light_mode</md-icon>
            <md-icon v-else>dark_mode</md-icon>
          </md-icon-button>
        </template>
      </mx-top-app-bar>

      <html-renderer />
    </div>
  </mx-theme>
</template>

<style>
@page {
  size: portrait;
  margin: 0;
}
</style>
