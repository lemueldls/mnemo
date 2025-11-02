<script setup lang="ts">
const container = useTemplateRef("container");
const { width } = useElementSize(container);

const reviewCardWidth = 352;
const gapSize = 12;

const amount = computed(
  () => Math.floor(width.value / (reviewCardWidth + gapSize)) * 2,
);

const spaces = await useSpaces();
const review = await useReview(amount);
</script>

<template>
  <div ref="container">
    <div v-if="review.length > 0" id="review">
      <nuxt-link
        v-for="{ spaceId, date, noteId, lastReviewed, stage } in review"
        :key="noteId"
        :to="`/space?id=${spaceId}&note=${noteId}`"
      >
        <mx-theme :color="spaces[spaceId]!.color">
          <md-elevated-card class="h-70 relative flex flex-col">
            <md-ripple />

            <div
              class="text-on-primary-container flex w-full items-center justify-between gap-2 bg-transparent p-2 pb-0 font-mono outline-none"
            >
              <md-divider class="w-2" />

              <!-- <span class="label-large">
                      Reviewed {{ useRelativeTime(lastReviewed) }}
                    </span> -->

              <span class="label-large">
                {{ date }}
              </span>

              <!-- <md-icon class="text-primary text-2xl!">
                      {{ spaces[spaceId]!.icon }}
                    </md-icon> -->

              <md-divider class="flex-1" />

              <md-icon v-if="spaces[spaceId]?.icon" class="text-primary">
                {{ spaces[spaceId].icon }}
              </md-icon>
            </div>

            <editor
              class="flex-1"
              :space-id="spaceId"
              kind="daily"
              :model-value="noteId"
              readonly
              locked
            />
          </md-elevated-card>
        </mx-theme>
      </nuxt-link>
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
