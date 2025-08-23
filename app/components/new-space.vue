<script setup lang="ts">
import { createId } from "@paralleldrive/cuid2";

const open = defineModel<boolean>();

// const router = useRouter();

const spaces = await useSpaces();
const space = ref<Space>({
  name: "",
  icon: undefined,
  color: "#16161d", // Eigengrau
  order: Object.keys(spaces.value).length,
});

function createSpace() {
  const id = createId();
  spaces.set(id, space.value);

  // void router.push(`/space?id=${id}`);
  open.value = false;
}
</script>

<template>
  <mx-theme :color="space.color" harmonize>
    <md-dialog :open class="w-full max-w-xl" @closed="open = false">
      <span slot="headline">{{ $t("components.new-space.title") }}</span>

      <form
        id="new-space-form"
        slot="content"
        method="dialog"
        class="flex flex-col gap-8"
        @submit.prevent="createSpace"
      >
        <edit-space v-model="space" />
      </form>

      <div slot="actions">
        <md-text-button form="new-space-form">
          {{ $t("components.new-space.form.create") }}
        </md-text-button>
      </div>
    </md-dialog>
  </mx-theme>
</template>
