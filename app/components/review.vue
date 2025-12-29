<script setup lang="ts">
const container = useTemplateRef("container");
const { width } = useElementSize(container);

const reviewCardWidth = 352;
const gapSize = 12;

const amount = computed(() => Math.floor((width.value + gapSize) / (reviewCardWidth + gapSize)));

const review = await useReview(amount);
</script>

<template>
  <div ref="container">
    <div v-if="review.length > 0" id="review">
      <template v-for="(review, i) in review" :key="review.noteId">
        <review-card v-if="i < amount" :review />
      </template>
    </div>
    <span v-else class="text-on-surface-varient body-large">
      {{ $t("components.review.no-review") }}
    </span>
  </div>
</template>

<style>
#review {
  @apply grid gap-3;

  grid-template-columns: repeat(auto-fill, minmax(22rem, 1fr));
}
</style>
