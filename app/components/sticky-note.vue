<script setup lang="ts">
import interact from "interactjs";

const model = defineModel<StickyNote>();
const note = toReactive(model);

defineProps<{ spaceId: string }>();
// const { spaceId, noteId } = toRefs(props);

defineEmits<{ (event: "close"): void }>();

const rootRef = useTemplateRef("root");
const headerRef = useTemplateRef("header");

const isDragging = ref(false);

// const notes = await useStorageItem<{ [id: string]: StickyNote }>(
//   `spaces/${spaceId.value}/sticky/notes.json`,
//   {},
// );

// watchEffect(() => {
//   console.log({
//     props,
//     spaceId: spaceId.value,
//     noteId: noteId.value,
//     notes: notes.value,
//   });
// });

// const note = reactive(notes.value[noteId.value]);
// // console.log({ note });
// watch(note, (note) => {
//   notes.value[noteId.value] = note;
// });

onMounted(() => {
  const root = rootRef.value!;

  // x = 0;
  // y = 0;

  root.style.transform = `translate(${note.x}px, ${note.y}px)`;
  root.style.width = note.width + "px";
  root.style.height = note.height + "px";

  interact(root)
    .draggable({
      inertia: true,
      autoScroll: false,
      allowFrom: headerRef.value!,
      modifiers: [
        interact.modifiers.restrictRect({
          restriction: "parent",
          endOnly: true,
        }),
        interact.modifiers.snap({
          targets: [interact.snappers.grid({ x: 8, y: 8 })],
          range: Infinity,
          relativePoints: [{ x: 0, y: 0 }],
        }),
      ],
      listeners: {
        move(event) {
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();

          isDragging.value = true;
          const { target } = event;

          note.x += event.dx;
          note.y += event.dy;

          target.style.transform = `translate(${note.x}px, ${note.y}px)`;
        },
        end(event) {
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();

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
          event.preventDefault();
          event.stopPropagation();
          event.stopImmediatePropagation();

          const { target } = event;

          note.width = event.rect.width;
          note.height = event.rect.height;

          target.style.width = `${note.width}px`;
          target.style.height = `${note.height}px`;

          note.x += event.deltaRect.left;
          note.y += event.deltaRect.top;

          target.style.transform = `translate(${note.x}px,${note.y}px)`;
        },
      },
    });
});

// watchEffect(async () => {
//   await updateStickyNote(
//     spaceId.value,
//     props.note.id,
//     x,
//     y,
//     width,
//     height,
//   );
// });

// watchEffect(async () => {
//   await renameStickyNote(spaceId.value, props.note.id, title.value);
// });
</script>

<template>
  <div
    ref="root"
    :class="['sticky-note', { 'sticky-note--dragging': isDragging }]"
  >
    <md-elevation />

    <m3-filled-card
      class="bg-m3-tertiary-container! flex flex-1 flex-col gap-4 rounded-xl p-4"
    >
      <div class="flex h-full flex-1 flex-col gap-4">
        <div ref="header" class="flex items-center gap-2">
          <input
            v-model="note.title"
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
          class="h-full flex-1"
        />
      </div>
    </m3-filled-card>
  </div>
</template>

<style lang="scss">
.sticky-note {
  @apply rounded-3 z-1 absolute flex transition-shadow;

  --md-elevation-level: 1;

  touch-action: none;
  user-select: none;

  &--dragging {
    // @apply cursor-move;

    --md-elevation-level: 2;
  }

  &__title {
    @apply m3-headline-large text-m3-on-primary-container z-1 w-0 flex-1 bg-transparent outline-none;

    touch-action: none;
    user-select: none;

    font-family: "Iosevka Book Web", sans-serif;
  }
}
</style>
