<script setup lang="ts">
import "@material/web/all";
import "@material/web/labs/card/elevated-card";
import "@material/web/labs/card/filled-card";
import "@material/web/labs/card/outlined-card";
import { isTauri } from "@tauri-apps/api/core";
import { onOpenUrl } from "@tauri-apps/plugin-deep-link";

if (isTauri()) {
  const unlisten = await onOpenUrl(async (urls) => {
    const [callback] = urls;
    if (!callback)
      throw createError({
        statusCode: 400,
        statusMessage: "No callback URL provided",
      });

    const callbackUrl = new URL(callback);
    const token = callbackUrl.searchParams.get("token")!;
    const redirect = callbackUrl.searchParams.get("redirect")!;

    if (token)
      await navigateTo({ path: "/auth/confirm", query: { token, redirect } });
  });

  await checkForAppUpdates();

  onUnmounted(() => {
    unlisten();
  });
}

const { t, te } = useI18n();

const route = useRoute();
const head = useLocaleHead({
  dir: true,
  lang: true,
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

  return description && te(description) ? t(description) : t("app.description");
});

useHead(() => ({
  title,
  htmlAttrs: head.value.htmlAttrs,
  link: [...(head.value.link || [])],
  meta: [...(head.value.meta || [])],
}));

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

  useEventListener(window, "resize", resize);
  useEventListener(window, "mouseup", (event) => {
    x.value = event.clientX;
    y.value = event.clientY;
  });
});

const ready = ref(false);
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

  animation: conjure 1s ease-out;
}

.conjure-leave-active {
  @apply z-[-2];

  animation: none 1s;
}
</style>
