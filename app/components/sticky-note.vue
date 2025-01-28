<script setup lang="ts">
import interact from "interactjs";
import { rawListeners } from "process";

const props = defineProps<{ spaceId: string; note: StickyNote }>();
defineEmits<{ (event: "close"): void }>();

const rootRef = useTemplateRef("root");
const headerRef = useTemplateRef("header");

const x = ref(props.note.x);
const y = ref(props.note.y);
const width = ref(props.note.width);
const height = ref(props.note.height);

const isDragging = ref(false);

const notes = await useStorageItem<StickyNote[]>(
  `spaces/${props.spaceId}/sticky/notes.json`,
  [],
);

const storageItem = await useStorageItem<string>(
  `spaces/${props.spaceId}/sticky/${props.note.id}.typ`,
  "",
);

onMounted(() => {
  const root = rootRef.value!;

  // x.value = 0;
  // y.value = 0;

  root.style.transform = `translate(${x.value}px, ${y.value}px)`;
  root.style.width = width.value + "px";
  root.style.height = height.value + "px";

  interact(root)
    .draggable({
      inertia: true,
      autoScroll: true,
      // allowFrom: headerRef.value!,
      modifiers: [
        interact.modifiers.restrictRect({
          restriction: "parent",
          endOnly: true,
        }),
      ],
      listeners: {
        move(event) {
          isDragging.value = true;
          const { target } = event;

          x.value += event.dx;
          y.value += event.dy;

          target.style.transform = `translate(${x.value}px, ${y.value}px)`;
        },
        end() {
          isDragging.value = false;
        },
      },
    })
    .resizable({
      inertia: true,
      edges: { left: true, right: true, bottom: true, top: true },
      modifiers: [
        interact.modifiers.restrictEdges({
          outer: "parent",
        }),
        interact.modifiers.restrictSize({
          min: { width: 100, height: 100 },
        }),
      ],
      listeners: {
        move(event) {
          const { target } = event;

          width.value = event.rect.width;
          height.value = event.rect.height;

          target.style.width = `${width.value}px`;
          target.style.height = `${height.value}px`;

          x.value += event.deltaRect.left;
          y.value += event.deltaRect.top;

          target.style.transform = `translate(${x.value}px,${y.value}px)`;
        },
      },
    });
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
  await renameStickyNote(props.spaceId, props.note.id, title.value);
});
</script>

<template>
  <div
    ref="root"
    :class="['sticky-note', { 'sticky-note--dragging': isDragging }]"
  >
    <md-elevation />

    <m3-filled-card
      class="flex-1 flex flex-col gap-4 p-4 rounded-xl bg-m3-tertiary-container!"
    >
      <div class="flex-1 h-full flex flex-col gap-4">
        <div ref="header" class="flex items-center gap-2">
          <input
            v-model="title"
            type="text"
            class="sticky-note__title"
            placeholder="Title"
          />

          <md-icon-button @click.prevent="$emit('close')">
            <md-icon>close</md-icon>
          </md-icon-button>
        </div>

        <editor
          v-model="note.id"
          kind="sticky"
          :space-id="spaceId"
          class="flex-1 h-full"
        />
      </div>
    </m3-filled-card>
  </div>
</template>

<style lang="scss">
.sticky-note {
  @apply absolute flex rounded-3 z-1 transition-shadow;

  --md-elevation-level: 1;

  touch-action: none;
  user-select: none;

  &--dragging {
    // @apply cursor-move;

    --md-elevation-level: 2;
  }

  &__title {
    @apply w-0 flex-1 m3-headline-large text-m3-on-primary-container bg-transparent outline-none z-1;

    touch-action: none;
    user-select: none;

    font-family: "Iosevka Book Web", sans-serif;
  }
}
</style>
