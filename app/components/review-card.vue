<script setup lang="ts">
const { review } = defineProps<{ review: Review }>();

const spaces = await useSpaces();
const space = computed(() => spaces.value[review.spaceId]!);

const stages = useReviewStages().length;
</script>

<template>
  <nuxt-link :to="`/space?id=${review.spaceId}&note=${review.noteId}`">
    <mx-theme :color="space.color">
      <md-elevated-card class="h-70 relative flex flex-col">
        <md-ripple />

        <div
          class="text-on-primary-container flex w-full items-center justify-between gap-2 bg-transparent p-2 pb-0 font-mono outline-none"
        >
          <md-divider class="w-2" />

          <span class="label-large">
            {{ review.date }}
          </span>

          <!-- <md-icon class="text-primary text-2xl!">
            {{ space.icon }}
          </md-icon> -->

          <md-divider class="flex-1" />

          <md-icon v-if="space?.icon" class="text-primary">
            {{ space.icon }}
          </md-icon>
        </div>

        <div class="flex-1 overflow-hidden">
          <editor
            :space-id="review.spaceId"
            kind="daily"
            :model-value="review.noteId"
            readonly
            locked
          />
        </div>

        <div
          v-if="review.lastReviewed"
          class="text-on-primary-container flex gap-3 p-2 pt-0 font-mono"
        >
          <div class="m-1.5 flex h-2 flex-1 gap-1 self-end">
            <div
              v-for="i in stages"
              :key="i"
              :class="['flex-1 rounded', i > review.stage ? 'bg-surface' : 'bg-primary']"
            />
          </div>

          <span class="label-large self-end text-right">
            {{ $t("components.review-card.reviewed") }}
            {{ useRelativeTime(review.lastReviewed) }}
          </span>
        </div>
      </md-elevated-card>
    </mx-theme>
  </nuxt-link>
</template>
