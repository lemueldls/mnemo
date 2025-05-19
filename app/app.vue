<script setup lang="ts">
import "@material/web/all";
// import "@material/web/labs/card/outlined-card";
import "@material/web/labs/card/elevated-card";

const runtimeConfig = useRuntimeConfig();
const { platform } = runtimeConfig.public;

if (platform)
  onMounted(async () => {
    if (!window.__TAURI_INTERNALS__) return;

    const { onOpenUrl } = await import("@tauri-apps/plugin-deep-link");

    const unlisten = await onOpenUrl((urls) => {
      console.log("deep link:", urls);
    });

    tryOnUnmounted(() => {
      unlisten();
    });
  });

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

watchEffect(() => {
  // const name = t("site.name");

  const routeTitle = route.meta.title;
  const title = routeTitle && te(routeTitle) ? t(routeTitle) : routeTitle;

  const routeDescription = route.meta.description;
  const description =
    routeDescription && te(routeDescription)
      ? t(routeDescription)
      : t("site.description");

  const localeHead = head.value;

  useHead({
    title,
    htmlAttrs: localeHead.htmlAttrs,
    // base: { href: siteUrl },
    link: localeHead.link,
    meta: localeHead.meta,
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
});

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
</script>

<template>
  <div>
    <NuxtLayout>
      <NuxtPage />
    </NuxtLayout>
  </div>
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
