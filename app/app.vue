<script setup lang="ts">
import "@material/web/all";
import "@material/web/labs/card/outlined-card";
import "@material/web/labs/card/filled-card";
import "@material/web/labs/card/elevated-card";

const { t, te } = useI18n();

const route = useRoute();
const head = useLocaleHead({
  dir: true,
  lang: true,
  key: "id",
  seo: true,
});

declare module "vue-router" {
  interface RouteMeta {
    title?: string;
    description?: string;
    label?: string;
    icon?: string;
  }
}

// const name = t("site.name");

const routeTitle = computed(() => route.meta.title);
const title = computed(() => {
  const title = routeTitle.value;

  return title && te(title) ? t(title) : title;
});

const routeDescription = computed(() => route.meta.description);
const description = computed(() => {
  const description = routeDescription.value;

  return description && te(description)
    ? t(description)
    : t("site.description");
});

useHead({
  title,
  // base: { href: siteUrl },
  htmlAttrs: computed(() => head.value.htmlAttrs),
  link: computed(() => head.value.link),
  meta: computed(() => head.value.meta),
});

useSeoMeta({
  title,
  ogTitle: title,
  description,
  ogDescription: description,
});

// useSchemaOrg([
//   defineWebSite({ name, description }),
//   defineWebPage({ name, description }),
//   defineSoftwareApp({ name, description }),
// ]);

const x = ref(0);
const y = ref(0);

function resize() {
  x.value = window.innerWidth / 2;
  y.value = window.innerHeight / 2;
}

onMounted(() => {
  resize();

  window.addEventListener("resize", resize);
  window.addEventListener("mousedown", (event) => {
    x.value = event.clientX;
    y.value = event.clientY;
  });
});

const ready = ref(false);

// const session = useCookie("nuxt-session");

// watchEffect(() => {
//   console.log({ session: session.value });
// });
</script>

<template>
  <div v-show="ready">
    <ClientOnly>
      <NuxtLayout>
        <NuxtPage />
      </NuxtLayout>
    </ClientOnly>
  </div>

  <Splashscreen @ready="ready = true" />
</template>

<style>
@keyframes conjure {
  0% {
    clip-path: circle(0% at calc(1px * v-bind(x)) calc(1px * v-bind(y)));
  }

  100% {
    clip-path: circle(100%);
  }
}

.conjure-enter-active {
  @apply z-2;

  animation: conjure 1000ms ease-out;
}

.conjure-leave-active {
  @apply z-[-2];

  animation: none 1000ms;
}
</style>
