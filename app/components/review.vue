<script setup lang="ts">
const container = useTemplateRef("container");
const { width } = useElementSize(container);

const reviewCardWidth = 352;
const gapSize = 12;

const amount = computed(() =>
  Math.floor(width.value / (reviewCardWidth + gapSize)),
);

const review = await useReview(amount);
</script>

<template>
  <div ref="container">
    <div v-if="review.length > 0" id="review">
      <template v-for="i in amount">
        <review-card v-if="i < review.length" :review="review[i]!" />
        <md-outlined-card v-else />
      </template>
    </div>
    <span v-else class="text-on-surface-varient body-large">
      Nothing yet..
    </span>
  </div>
</template>

<style>
#review {
  @apply grid gap-3;

  grid-template-columns: repeat(auto-fill, minmax(22rem, 1fr));
}
</style>
