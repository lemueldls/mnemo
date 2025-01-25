<script setup lang="ts">
const dark = useDark();
const spaces = listSpaces();
</script>

<template>
  <nuxt-link
    v-for="(space, name) in spaces"
    :key="name"
    :to="`/space?id=${name}`"
  >
    <m3-theme asChild :color="space.color" harmonize :dark="dark">
      <m3-nav-drawer-item>
        <template #leading>
          <m3-icon rounded :name="space.icon" class="text-m3-primary" />
        </template>

        {{ name }}
      </m3-nav-drawer-item>
    </m3-theme>
  </nuxt-link>
</template>
