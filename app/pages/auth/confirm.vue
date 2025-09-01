<script lang="ts" setup>
import { match } from "ts-pattern";

definePageMeta({ layout: "empty" });

const token = useRouteQuery("token").value as string;
const redirect = useRouteQuery("redirect").value as string;
const platform = useRouteQuery("platform").value as string;

if (token) {
  useApiToken().value = encodeURIComponent(token);

  const auth = useAuth();
  await auth.fetchSession();
}

const bearerToken = useApiToken().value!;

match(platform)
  .with("windows", "darwin", "linux", () => {
    window.location.href = `mnemo://auth/confirm?token=${bearerToken}&redirect=${encodeURIComponent(redirect)}`;
  })
  .with("android", "ios", () => {
    window.open(
      `/auth/confirm?token=${bearerToken}&redirect=${encodeURIComponent(redirect)}`,
      "_blank",
    );
  })
  .otherwise(async () => {
    const redirectUrl = new URL(redirect);

    if (redirectUrl.origin === useRequestURL().origin)
      await navigateTo(redirectUrl.href.replace(redirectUrl.origin, ""));
    else {
      redirectUrl.searchParams.set("token", bearerToken);

      window.location.href = new URL(
        `/auth/confirm?token=${bearerToken}&redirect=${encodeURIComponent(redirect)}`,
        redirectUrl,
      ).href;
    }
  });
</script>

<template>
  <div />
</template>
