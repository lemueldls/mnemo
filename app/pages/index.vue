<script setup lang="ts">
definePageMeta({ title: "Home" });

const name = await useStorageItem("name", "");

const { d } = useI18n();
const date = useNow({ interval: 1000 * 60 * 15 });

const newSpaceOpen = useNewSpaceOpen();

const spaces = await useSpaces();

const timeOfDay = computed(() => {
  const hour = date.value.getHours();

  return hour >= 4 && hour <= 11
    ? "morning"
    : hour >= 12 && hour <= 16
      ? "afternoon"
      : hour >= 17 && hour <= 20
        ? "evening"
        : "night";
});
</script>

<template>
  <div id="home-page">
    <div id="main-column">
      <m3-outlined-card
        id="intro"
        class="medium:border-1! medium:p-4! p-0! border-0!"
      >
        <div>
          <h1 class="m3-display-medium">
            Good {{ timeOfDay }}{{ name ? ", " + name : "" }}.
          </h1>

          <span class="text-m3-on-surface-variant m3-title-large">
            Today is {{ d(date, { dateStyle: "full" }) }}
          </span>
        </div>

        <div class="flex-shrink-0">
          <div id="progress">
            <nuxt-link
              v-for="(space, id) in spaces"
              :key="id"
              :to="`/space?id=${id}`"
            >
              <m3-theme :color="space.color" harmonize>
                <m3-elevated-card class="relative gap-2">
                  <md-ripple />

                  <div class="flex items-center gap-2">
                    <m3-icon :name="space.icon" class="text-m3-primary" />
                  </div>

                  <h3 class="m3-title-large line-clamp-1" :title="space.name">
                    {{ space.name }}
                  </h3>
                </m3-elevated-card>
              </m3-theme>
            </nuxt-link>

            <m3-elevated-card
              v-if="Object.keys(spaces).length < 1"
              class="relative cursor-pointer gap-2"
              @click="newSpaceOpen = true"
            >
              <md-ripple />

              <div class="flex items-center gap-2">
                <m3-icon name="add" class="text-m3-primary" />
              </div>

              <h3 class="m3-title-large">Create a New Space</h3>
            </m3-elevated-card>
          </div>
        </div>

        <m3-filled-card>
          <h3 class="m3-label-large">Todo</h3>

          <span class="text-m3-on-surface-varient m3-body-large">
            Nothing yet...
          </span>
        </m3-filled-card>
      </m3-outlined-card>
    </div>
  </div>
</template>

<style>
#home-page {
  @apply flex flex-1;
}

#progress {
  @apply grid gap-4;

  /* grid-template-columns: repeat(auto-fill, minmax(26rem, 1fr)); */
  grid-template-columns: repeat(auto-fill, minmax(20rem, 1fr));
}

#notes {
  @apply grid gap-4;

  grid-template-columns: repeat(auto-fill, minmax(17.125rem, 1fr));
  /* grid-template-columns: repeat(auto-fill, minmax(6rem, 1fr)); */
}

#main-column {
  @apply flex flex-1 flex-col gap-4 overflow-y-auto overflow-x-hidden;
}

#intro {
  @apply flex flex-1 flex-col gap-4;
}
</style>
