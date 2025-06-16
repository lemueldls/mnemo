<script setup lang="ts">
definePageMeta({ title: "Home" });

const name = await useStorageItem("name", "");

const { t, d } = useI18n();
const date = useNow({ interval: 1000 * 60 * 15 });

const newSpaceOpen = useNewSpaceOpen();

const spaces = await useSpaces();

const greeting = computed(() => {
  const hour = date.value.getHours();

  return hour >= 4 && hour <= 11
    ? t("pages.index.greeting.morning", { name: name.value })
    : hour >= 12 && hour <= 16
      ? t("pages.index.greeting.afternoon", { name: name.value })
      : hour >= 17 && hour <= 20
        ? t("pages.index.greeting.evening", { name: name.value })
        : t("pages.index.greeting.night", { name: name.value });
});
</script>

<template>
  <div id="home-page">
    <div id="main-column">
      <mx-outlined-card
        id="intro"
        class="medium:border-1! medium:p-4! p-0! border-0!"
      >
        <div>
          <h1 class="display-medium">
            {{ greeting }}
          </h1>

          <span class="text-on-surface-variant title-large">
            {{
              t("pages.index.date", { date: d(date, { dateStyle: "full" }) })
            }}
          </span>
        </div>

        <div class="flex-shrink-0">
          <div id="progress">
            <nuxt-link
              v-for="(space, id) in spaces"
              :key="id"
              :to="`/space?id=${id}`"
            >
              <mx-theme :color="space.color" harmonize>
                <mx-elevated-card class="relative gap-2">
                  <md-ripple />

                  <div class="flex items-center gap-2">
                    <mx-icon :name="space.icon" class="text-primary" />
                  </div>

                  <h3 class="title-large line-clamp-1" :title="space.name">
                    {{ space.name }}
                  </h3>
                </mx-elevated-card>
              </mx-theme>
            </nuxt-link>

            <mx-elevated-card
              v-if="Object.keys(spaces).length < 1"
              class="relative cursor-pointer gap-2"
              @click="newSpaceOpen = true"
            >
              <md-ripple />

              <div class="flex items-center gap-2">
                <mx-icon name="add" class="text-primary" />
              </div>

              <h3 class="title-large">Create a New Space</h3>
            </mx-elevated-card>
          </div>
        </div>

        <mx-filled-card>
          <h3 class="label-large">Todo</h3>

          <span class="text-on-surface-varient body-large">
            Nothing yet...
          </span>
        </mx-filled-card>
      </mx-outlined-card>
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
