<script setup lang="ts">
import "@material/web/all";
import "material-symbols";

// import { signInAnonymously } from "firebase/auth";

// const auth = useFirebaseAuth();
// await signInAnonymously(auth!);

// const { auth } = useSupabaseClient();
// const {
//   data: { user },
// } = await auth.getUser();
// console.log({ user });
// if (!user) console.log(await auth.signInAnonymously());
// watchEffect(() => {
//   // console.log({ client: useSupabaseClient() });
//   console.log({ session: useSupabaseSession().value });
//   console.log({ user: useSupabaseUser().value });
// });
// // watch(useSupabaseUser(), console.log);

// const supabase = useSupabaseClient();
// console.log({ anon: await supabase.auth.signInAnonymously() });

const { t } = useI18n();

const route = useRoute();
const head = useLocaleHead({
  addDirAttribute: true,
  identifierAttribute: "id",
  addSeoAttributes: true,
});

const title = "LMFAO";
// const title = computed(() => {
//   const title = route.meta.title as string;

//   return t("app.title", { title: title ? t(title) : "" });
// });

watchEffect(() => {
  const routeTitle = route.meta.title as string | undefined;
  const title = routeTitle ? t(routeTitle) : "";

  const localeHead = head.value;

  useHead({
    title,
    htmlAttrs: localeHead.htmlAttrs,
    link: localeHead.link,
    meta: localeHead.meta,
  });
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
