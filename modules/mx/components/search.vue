<script setup lang="ts"></script>

<template>
  <div class="m3-search">
    <m3-container class="m3-search-container">
      <m3-icon name="search" class="leading-icon" />

      <input />
    </m3-container>
  </div>
</template>

<style scoped lang="scss">
.m3-search-container {
  @apply m3-body-large flex gap-4 rounded-full p-4;

  width: 250px;
  min-width: 22.5rem;
  max-width: 45rem;
  height: 3.5rem;

  // > {
  //   input {
  //     @apply bg-transparent outline-none;
  //   }

  //   .leading-icon {
  //     font-size: 1.5625rem;

  //     color: v-bind(leadingColor);
  //   }

  //   .trailing-icon {
  //     color: v-bind(textColor);
  //   }
  // }
}
</style>
