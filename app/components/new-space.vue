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
    <md-dialog :open="open" @closed="open = false">
      <span slot="headline">{{ $t("components.new-space.title") }}</span>

      <form
        slot="content"
        method="dialog"
        class="flex flex-col gap-8"
        @submit.prevent="createSpace"
      >
        <edit-space v-model="space">
          <template #actions>
            <md-text-button>
              {{ $t("components.new-space.form.create") }}
            </md-text-button>
          </template>
        </edit-space>
      </form>
    </md-dialog>
  </mx-theme>
</template>
