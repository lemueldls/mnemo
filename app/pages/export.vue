<script setup lang="ts">
definePageMeta({ layout: "empty" });

const spaceId = usePageRouteQuery("space");

const spaces = await useSpaces();
const space = computed(() => spaces.value[spaceId.value]!);
</script>

<template>
  <mx-theme :color="space.color" :dark="false">
    <mx-page>
      <div class="flex h-full flex-1">
        <div class="flex size-full flex-1 flex-col">
          <mx-top-app-bar>
            <template #leading>
              <nuxt-link-locale :to="`/space?id=${spaceId}`">
                <md-icon-button>
                  <md-icon>arrow_back</md-icon>
                </md-icon-button>
              </nuxt-link-locale>
            </template>
          </mx-top-app-bar>

          <pdf-renderer />
        </div>
      </div>
    </mx-page>
  </mx-theme>
</template>

<style lang="scss">
/* #space-page {
  @apply absolute inset-0;
} */
</style>
