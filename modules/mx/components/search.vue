<script setup lang="ts"></script>

<template>
  <div class="search">
    <mx-container class="search-container">
      <mx-icon name="search" class="leading-icon" />

      <input />
    </mx-container>
  </div>
</template>

<style scoped lang="scss">
.search-container {
  @apply body-large flex gap-4 rounded-full p-4;

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
