<script setup lang="ts">
const name = useLocalStorage("name", "");

const dark = useDark();
const date = ref(new Date());

const { d, locale } = useI18n();
// const spaces = listSpaces();

// interface Note {
//   title: string;
//   space: string;
//   preview: string;
// }
// const notes: Note[] = [];

// for (const space of Object.keys(spaces.value))
//   notes.push({
//     title: d(date.value, { dateStyle: "long" }),
//     space,
//     preview:
//       "Repudiandae quo facilis natus nemo aut dolores. Officia quis non dolore dicta autem. Libero consequatur autem nostrum nesciunt. Nesciunt enim accusamus nulla eveniet nostrum quae dolore cum. Et aperiam maxime ut ducimus commodi quos culpa.",
//   });

useIntervalFn(
  () => {
    date.value = new Date();
  },
  1000 * 60 * 15,
);

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
      <m3-outlined-card id="intro">
        <div>
          <h1 class="m3-display-medium">
            Good {{ timeOfDay }}{{ name ? ", " + name : "" }}.
          </h1>

          <span class="text-m3-on-surface-variant m3-title-large">
            Today is {{ $d(date, { dateStyle: "full" }) }}
          </span>
        </div>

        <div
          class="text-m3-on-surface-container flex flex-1 items-center justify-center m3-label-large"
        >
          Nothing yet
        </div>

        <!-- <m3-filled-card>
          <span class="text-m3-on-surface-varient m3-body-large">
            There is nothing left to do...
          </span>
        </m3-filled-card> -->

        <!-- <m3-outlined-card class="flex-shrink-0">
          <h2 class="pb-4 m3-title-large">Stuff</h2>

          <div id="spaces">
            <nuxt-link
              v-for="(note, i) in notes"
              :key="i"
              :to="`/space?id=${note.space}`"
            >
              <m3-theme
                :color="spaces[note.space].color"
                :dark="dark"
                harmonize
              >
                <m3-filled-card>
                  <div class="flex items-center justify-between">
                    <m3-icon
                      rounded
                      :name="spaces[note.space].icon"
                      :style="{ color: 'var(--m3-primary)' }"
                    />

                    {{ relativeTime.format(-3, "hours") }}
                  </div>

                  <h3 class="m3-headline-small">
                    {{ note.title }}
                  </h3>

                  <md-divider />

                  <p class="m3-body-large">
                    {{ note.preview }}
                  </p>
                </m3-filled-card>
              </m3-theme>
            </nuxt-link>
          </div>
        </m3-outlined-card> -->
      </m3-outlined-card>
    </div>
  </div>
</template>

<style>
#home-page {
  @apply flex flex-1;
}

#spaces {
  @apply grid gap-4;

  grid-template-columns: repeat(auto-fill, minmax(17.125rem, 1fr));
}

#main-column {
  @apply flex flex-1 flex-col gap-4 overflow-y-auto overflow-x-hidden;
}

#intro {
  @apply flex-1 flex gap-4 flex-col;
}
</style>
