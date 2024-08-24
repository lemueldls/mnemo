<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

const color = "#16161d";
const dark = useDark();

const props = defineProps<{ spaceId: string; note: StickyNote }>();
defineEmits<{ (event: "close"): void }>();

const el = ref<HTMLElement | null>(null);

// const { id, name, x, y, width, height } = props.note;

// `style` will be a helper computed for `left: ?px; top: ?px;`
const { x, y, style, isDragging } = useDraggable(el, {
  initialValue: { x: props.note.x, y: props.note.y },
});
const { width, height } = useElementSize(el);

console.log({ props });

watchEffect(() => {
  const { width: windowWidth, height: windowHeight } = useWindowSize();

  const minX = width.value / -2;
  const minY = height.value / -2;
  const maxX = windowWidth.value - width.value / 2;
  const maxY = windowHeight.value - height.value / 2;

  if (x.value < minX) x.value = minX;
  if (y.value < minY) y.value = minY;
  if (x.value > maxX) x.value = maxX;
  if (y.value > maxY) y.value = maxY;
});

watchEffect(async () => {
  await updateStickyNote(
    props.spaceId,
    props.note.id,
    x.value,
    y.value,
    width.value,
    height.value,
  );
});

const title = ref(props.note.name);

watchEffect(async () => {
  console.log(title.value);
  await renameStickyNote(props.spaceId, props.note.id, title.value);
});
</script>

<template>
  <div
    :class="['sticky-note', { 'sticky-note--dragging': isDragging }]"
    :style="style"
    ref="el"
  >
    <md-elevation />

    <m3-filled-card
      class="flex-1 flex flex-col gap-4 p-4 rounded-xl bg-m3-tertiary-container!"
    >
      <div class="flex flex-col gap-4">
        <div class="flex items-center gap-2">
          <!-- <span class="m3-headline-large text-m3-on-primary-container">
            {{ useShortDate(new Date()) }}
          </span> -->
          <input
            type="text"
            class="sticky-note__title"
            placeholder="Title"
            v-model="title"
          />

          <md-icon-button @click="$emit('close')">
            <md-icon>close</md-icon>
          </md-icon-button>
        </div>

        <editor
          kind="sticky"
          :space-id="spaceId"
          v-model="note.id"
          class="flex-1"
        />
      </div>
    </m3-filled-card>
  </div>
</template>

<style lang="scss">
.sticky-note {
  @apply fixed flex rounded-3 z-1 w-100 h-100 transition-shadow;

  --md-elevation-level: 1;

  &--dragging {
    @apply cursor-move;

    --md-elevation-level: 2;
  }

  &__title {
    @apply w-0 flex-1 m3-headline-large text-m3-on-primary-container bg-transparent outline-none;
  }
}
</style>
