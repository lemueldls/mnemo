<script setup lang="ts">
const spaces = useSpaces();
</script>

<template>
  <nuxt-link
    v-for="(space, name) in spaces"
    :key="name"
    :to="`/space?id=${name}`"
  >
    <m3-theme as-child :color="space.color" harmonize>
      <m3-nav-drawer-item>
        <template #leading>
          <m3-icon rounded :name="space.icon" class="text-m3-primary" />
        </template>

        {{ name }}
      </m3-nav-drawer-item>
    </m3-theme>
  </nuxt-link>
</template>
