<script setup lang="ts">
definePageMeta({ title: "Home" });

const name = await useStorageText("name", "");

const { t, d } = useI18n();
const now = useNow({ interval: 1000 * 60 * 15 });

const greeting = computed(() => {
  const nameValue = name.value;
  const hour = now.value.getHours();

  return hour >= 4 && hour <= 11
    ? nameValue
      ? t("pages.index.greeting.with-name.morning", { name: nameValue })
      : t("pages.index.greeting.morning")
    : hour >= 12 && hour <= 16
      ? nameValue
        ? t("pages.index.greeting.with-name.afternoon", { name: nameValue })
        : t("pages.index.greeting.afternoon")
      : hour >= 17 && hour <= 20
        ? nameValue
          ? t("pages.index.greeting.with-name.evening", { name: nameValue })
          : t("pages.index.greeting.evening")
        : nameValue
          ? t("pages.index.greeting.with-name.night", { name: nameValue })
          : t("pages.index.greeting.night");
});

useHead({ title: greeting });

const spaces = await useSpaces();
const review = [];
const tasks = [];

const newSpaceOpen = useNewSpaceOpen();
</script>

<template>
  <div class="flex h-full flex-1">
    <div class="flex size-full flex-1 flex-col gap-3">
      <mx-outlined-card
        class="medium:border-1! medium:p-3! medium:overflow-auto border-0! p-0! flex h-full flex-1 flex-col gap-3"
      >
        <div>
          <h1 class="display-medium overflow-hidden">
            {{ greeting }}
          </h1>

          <span class="text-on-surface-variant title-large">
            {{ t("pages.index.date", { date: d(now, { dateStyle: "full" }) }) }}
          </span>
        </div>

        <div class="flex-shrink-0">
          <div id="spaces">
            <nuxt-link
              v-for="(space, id) in spaces"
              :key="id"
              :to="`/space?id=${id}`"
            >
              <mx-theme :color="space.color" harmonize>
                <md-outlined-card class="relative gap-2 p-3">
                  <md-ripple />

                  <div class="flex h-6 items-center gap-2">
                    <mx-icon
                      v-if="space.icon"
                      :name="space.icon"
                      class="text-primary"
                    />
                  </div>

                  <h3 class="title-large line-clamp-1" :title="space.name">
                    {{ space.name }}
                  </h3>

                  <md-linear-progress />
                </md-outlined-card>
              </mx-theme>
            </nuxt-link>

            <md-outlined-card
              v-if="Object.keys(spaces).length < 1"
              class="relative cursor-pointer gap-2 p-3"
              @click="newSpaceOpen = true"
            >
              <md-ripple />

              <div class="flex items-center gap-2">
                <mx-icon name="add" class="text-primary" />
              </div>

              <h3 class="title-large">Create a New Space</h3>
            </md-outlined-card>
          </div>
        </div>

        <!-- <div class="flex flex-col gap-3 lg:flex-row" /> -->

        <md-outlined-card class="flex flex-1 flex-col gap-3 p-3">
          <h3 class="title-large text-on-surface-variant">Tasks</h3>

          <div v-if="tasks.length > 0" id="tasks">
            <task-item v-for="(task, i) in tasks" :key="i" :task="task" />
          </div>
          <span v-else class="text-on-surface-varient body-large">
            Nothing yet..
          </span>
        </md-outlined-card>

        <md-elevated-card class="flex flex-1 flex-col gap-3 p-3">
          <h3 class="title-large text-on-surface-variant">Review</h3>

          <div v-if="review.length > 0" id="review">
            <nuxt-link
              v-for="{ spaceId, date, note, lastReviewed } in review"
              :key="note.id"
              :to="`/space?id=${spaceId}&note=${note.id}`"
            >
              <mx-theme :color="spaces[spaceId]!.color" harmonize>
                <md-elevated-card class="relative flex flex-col p-2">
                  <md-ripple />

                  <div
                    class="text-on-primary-container flex w-full items-center justify-between gap-2 bg-transparent font-mono outline-none"
                  >
                    <md-divider class="w-2" />

                    <span class="label-large">
                      {{ date }}
                    </span>

                    <md-divider class="flex-1" />

                    <!-- <span class="label-large">
                      Reviewed {{ useRelativeTime(lastReviewed) }}
                    </span> -->
                  </div>

                  <div class="p-2">
                    <editor
                      class="h-50"
                      :space-id="spaceId"
                      kind="daily"
                      :model-value="note.id"
                      readonly
                    />
                  </div>
                </md-elevated-card>
              </mx-theme>
            </nuxt-link>
          </div>
          <span v-else class="text-on-surface-varient body-large">
            Nothing yet..
          </span>
        </md-elevated-card>
      </mx-outlined-card>
    </div>
  </div>
</template>

<style>
#spaces {
  @apply grid gap-3;

  grid-template-columns: repeat(auto-fill, minmax(18rem, 1fr));
}

#review {
  @apply grid gap-3;

  grid-template-columns: repeat(auto-fill, minmax(22rem, 1fr));
}

#tasks {
  @apply grid gap-3;

  grid-template-columns: repeat(auto-fill, minmax(14rem, 1fr));
}
</style>
