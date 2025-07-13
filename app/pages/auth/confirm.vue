<script lang="ts" setup>
definePageMeta({ layout: "empty" });

const session = useRouteQuery("session").value as string;
const redirect = useRouteQuery("redirect").value as string;
const platform = useRouteQuery("platform").value as string;

if (session) {
  useApiSession().value = session;
  await useAuth().fetch();
}

const activeSession = useApiSession().value!;

if (platform === "true") {
  await navigateTo(
    `mnemo:///auth/confirm?session=${encodeURIComponent(activeSession)}&redirect=${encodeURIComponent(redirect)}`,
    { external: true },
  );
} else {
  const redirectUrl = new URL(redirect);

  if (redirectUrl.origin === useRequestURL().origin)
    await navigateTo(redirectUrl.pathname, { external: true });
  else {
    redirectUrl.searchParams.set("session", activeSession);

    await navigateTo(
      new URL(
        `/auth/confirm?session=${encodeURIComponent(activeSession)}&redirect=${encodeURIComponent(redirect)}`,
        redirectUrl,
      ),
      { external: true },
    );
  }
}
</script>

<template>
  <div />
</template>
