<script setup lang="ts">
definePageMeta({ title: "Home" });

const name = await useStorageItem("name", "");

const { t, d } = useI18n();
const date = useNow({ interval: 1000 * 60 * 15 });

const greeting = computed(() => {
  const nameValue = name.value;
  const hour = date.value.getHours();

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
    <div class="flex h-full flex-1 flex-col gap-4">
      <mx-outlined-card
        class="medium:border-1! medium:p-4! p-0! border-0! medium:overflow-auto flex h-full flex-1 flex-col gap-4"
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
          <div id="spaces">
            <nuxt-link
              v-for="(space, id) in spaces"
              :key="id"
              :to="`/space?id=${id}`"
            >
              <mx-theme :color="space.color" harmonize>
                <md-elevated-card class="relative gap-2 p-4">
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
                </md-elevated-card>
              </mx-theme>
            </nuxt-link>

            <md-elevated-card
              v-if="Object.keys(spaces).length < 1"
              class="relative cursor-pointer gap-2 p-4"
              @click="newSpaceOpen = true"
            >
              <md-ripple />

              <div class="flex items-center gap-2">
                <mx-icon name="add" class="text-primary" />
              </div>

              <h3 class="title-large">Create a New Space</h3>
            </md-elevated-card>
          </div>
        </div>

        <md-filled-card class="p-4">
          <h3 class="label-large">Todo</h3>

          <span class="text-on-surface-varient body-large">
            Nothing yet...
          </span>
        </md-filled-card>
      </mx-outlined-card>
    </div>
  </div>
</template>

<style>
#spaces {
  @apply grid gap-4;

  grid-template-columns: repeat(auto-fill, minmax(18rem, 1fr));
}
</style>
