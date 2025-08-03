<script lang="ts" setup>
import { match } from "ts-pattern";

definePageMeta({ layout: "empty" });

const token = useRouteQuery("token").value as string;
const redirect = useRouteQuery("redirect").value as string;
const platform = useRouteQuery("platform").value as string;

if (token) {
  useApiToken().value = token;

  const auth = useAuth();
  await auth.fetchSession();
}

const activeToken = useApiToken().value!;

match(platform)
  .with("windows", "darwin", "linux", () => {
    window.location.href = `mnemo://auth/confirm?token=${encodeURIComponent(activeToken)}&redirect=${encodeURIComponent(redirect)}`;
  })
  .with("android", "ios", () => {
    window.location.href = `/auth/confirm?token=${encodeURIComponent(activeToken)}&redirect=${encodeURIComponent(redirect)}`;
  })
  .otherwise(async () => {
    const redirectUrl = new URL(redirect);

    if (redirectUrl.origin === useRequestURL().origin)
      await navigateTo(redirectUrl.href.replace(redirectUrl.origin, ""));
    else {
      redirectUrl.searchParams.set("token", activeToken);

      window.location.href = new URL(
        `/auth/confirm?token=${encodeURIComponent(activeToken)}&redirect=${encodeURIComponent(redirect)}`,
        redirectUrl,
      ).href;
    }
  });
</script>

<template>
  <div />
</template>
