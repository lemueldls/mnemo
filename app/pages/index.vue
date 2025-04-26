<script setup lang="ts">
const name = await useStorageItem("name", "");

const dark = useDark();
const date = ref(new Date());

const { d, locale } = useI18n();
// const spaces = useSpaces();

const { $api } = useNuxtApp();

// const stream = await $api<ReadableStream>("/api/chat", { method: "post",responseType: "stream"});
// const intro = ref("...");

// const reader = stream.pipeThrough(new TextDecoderStream()).getReader();

// while (true) {
//   const { value, done } = await reader.read();
//   if (done) break;

//   intro.value += JSON.parse(`{${value}}`).data;
// }

const spaces = await useSpaces();

const spacesProgress = computed(() =>
  Object.entries(spaces.value).map((space) => ({
    space,
    progress: {
      current: Math.random() * 5,
      total: 5,
    },
  })),
);

// console.log({ spaces: spaces.value, spacesProgress: spacesProgress.value });

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

        <!-- <m3-filled-card>
          <span class="text-m3-on-surface-varient m3-body-large">
            There is nothing left to do...
          </span>
        </m3-filled-card> -->

        <!-- <m3-outlined-card>
          <h3 class="m3-label-large">Recommended</h3>

          <span class="text-m3-on-surface-varient m3-body-large">
            Nothing yet...
          </span>
        </m3-outlined-card> -->

        <div class="flex-shrink-0">
          <!-- <h2 class="pb-4 m3-title-large">Stuff</h2> -->

          <!-- <pre>
            <code>
              {{ intro }}
            </code>
          </pre> -->

          <div id="progress">
            <nuxt-link
              v-for="({ space: [id, space], progress }, i) in spacesProgress"
              :key="i"
              :to="`/space?id=${id}`"
            >
              <m3-theme :color="space.color" :dark="dark" harmonize>
                <m3-elevated-card class="gap-2 relative">
                  <md-ripple />

                  <div class="flex items-center gap-2">
                    <m3-icon
                      rounded
                      :name="space.icon"
                      :style="{ color: 'var(--md-sys-color-primary)' }"
                    />
                  </div>

                  <h3 class="m3-title-large">
                    {{ space.name }}
                  </h3>

                  <!-- <div class="flex flex-col">
                    <div
                      class="flex items-center justify-between m3-label-large"
                    >
                      <span>Weekly Study Hours</span>

                      <span
                        >{{ progress.current.toFixed(2) }} /
                        {{ progress.total.toFixed(2) }}</span
                      >
                    </div>

                    <md-linear-progress
                      :value="progress.current"
                      :max="progress.total"
                    />
                  </div> -->

                  <!-- <h3 class="m3-headline-small">
                    {{ space.name }}
                  </h3> -->

                  <!-- <md-divider /> -->

                  <!-- <p class="m3-body-large">
                    {{ note.preview }}
                  </p> -->
                </m3-elevated-card>
              </m3-theme>
            </nuxt-link>
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
  @apply flex-1 flex gap-4 flex-col;
}
</style>
