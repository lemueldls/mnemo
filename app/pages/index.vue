<script setup lang="ts">
definePageMeta({ title: "Home" });

const name = await useStorageText("name");

const { t, d } = useSharedI18n();
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

const newSpaceOpen = useNewSpaceOpen();
</script>

<template>
  <div class="flex h-full flex-1">
    <div class="flex size-full flex-1 flex-col gap-3">
      <mx-outlined-card
        class="medium:border-1! medium:p-3! medium:overflow-auto flex h-full flex-1 flex-col gap-3 border-0! p-0!"
      >
        <div class="expanded:flex-row flex flex-col gap-3">
          <div class="flex-1">
            <h1 class="display-medium overflow-hidden">
              {{ greeting }}
            </h1>

            <span class="text-on-surface-variant title-large">
              {{ t("pages.index.date", { date: d(now, { dateStyle: "full" }) }) }}
            </span>
          </div>

          <activity-graph class="expanded:max-w-sm w-full" />
        </div>

        <md-divider class="flex-shrink-0" />

        <div class="flex-shrink-0">
          <div id="spaces">
            <nuxt-link v-for="(space, id) in spaces" :key="id" :to="`/space?id=${id}`">
              <mx-theme :color="space.color" harmonize>
                <md-outlined-card class="relative p-3">
                  <md-ripple />

                  <div class="flex h-12 justify-between">
                    <div>
                      <md-icon v-if="space.icon" class="text-primary">
                        {{ space.icon }}
                      </md-icon>
                    </div>

                    <md-icon-button disabled @click.prevent>
                      <md-icon>more_vert</md-icon>
                    </md-icon-button>
                  </div>

                  <h3 class="title-large line-clamp-1" :title="space.name">
                    {{ space.name }}
                  </h3>

                  <!-- <span class="label-large"></span> -->

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

              <div class="flex h-12 justify-between">
                <mx-icon name="add" class="text-primary text-2xl!" />
              </div>

              <h3 class="title-large">Create a New Space</h3>
            </md-outlined-card>
          </div>
        </div>

        <md-elevated-card class="flex flex-1 flex-col gap-3 p-3">
          <h3 class="title-large text-on-surface-variant">Review</h3>

          <review />
        </md-elevated-card>

        <md-outlined-card class="flex flex-1 flex-col gap-3 p-3">
          <h3 class="title-large text-on-surface-variant">Tasks</h3>

          <tasks-masonry />
        </md-outlined-card>
      </mx-outlined-card>
    </div>
  </div>
</template>

<style>
#spaces {
  @apply grid gap-3;

  grid-template-columns: repeat(auto-fill, minmax(18rem, 1fr));
}
</style>
