<script setup lang="ts">
import interact from "interactjs";

import type { StickyNote } from "~/composables/sticky";
import type { Rgba } from "~~/modules/mx/types";

const model = defineModel<StickyNote>();
const note = toReactive(model) as StickyNote;

defineProps<{ spaceId: string }>();
defineEmits<{ (event: "close"): void }>();

const rootRef = useTemplateRef("root");
const headerRef = useTemplateRef("header");

const isDragging = ref(false);

const parent = useParentElement();
const { width: parentWidth, height: parentHeight } = useElementSize(parent);

onMounted(() => {
  const root = rootRef.value!;

  watchImmediate([parentWidth, parentHeight], ([parentWidth, parentHeight]) => {
    if (parentWidth) {
      if (note.rx) note.x = Math.max(note.rx * parentWidth - note.width / 2, 0);
      note.x = Math.min(note.x, parentWidth - note.width);
    }

    if (parentHeight) {
      if (note.ry)
        note.y = Math.max(note.ry * parentHeight - note.height / 2, 0);
      note.y = Math.min(note.y, parentHeight - note.height);
    }

    root.style.transform = `translate(${note.x}px,${note.y}px)`;
  });

  watchImmediate([() => note.x, () => note.y], ([x, y]) => {
    const rWidth = parentWidth.value;
    if (rWidth) note.rx = (x + note.width / 2) / rWidth;

    const rHeight = parentHeight.value;
    if (rHeight) note.ry = (y + note.height / 2) / rHeight;
  });

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
        // interact.modifiers.snap({
        //   targets: [interact.snappers.grid({ x: 8, y: 8 })],
        //   range: Infinity,
        //   relativePoints: [{ x: 0, y: 0 }],
        // }),
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
      // margin: 16,
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

const dark = useDark();
const theme = useMaterialTheme();
const selectionBackground = computed(() => {
  const { r, g, b } = theme!.value.palette.primaryContainer;
  return `rgba(${r},${g},${b},0.5)`;
});

const stickyNoteContainer = computed(() =>
  dark.value
    ? theme!.value.palette.onTertiaryContainer
    : theme!.value.palette.tertiaryContainer,
);
// const onStickyNoteContainer = computed(() =>
//   dark.value
//     ? theme!.value.palette.tertiaryContainer
//     : theme!.value.palette.onTertiaryContainer,
// );

function encodeColor({ r, g, b, a }: Rgba) {
  return `rgb(${r},${g},${b},${a})`;
}
</script>

<template>
  <div
    ref="root"
    :class="['sticky-note', { 'sticky-note--dragging': isDragging }]"
  >
    <md-elevation />

    <mx-filled-card
      class="flex flex-1 flex-col rounded-xl p-1"
      :style="{
        backgroundColor: encodeColor(stickyNoteContainer),
        // color: encodeColor(onStickyNoteContainer),
      }"
    >
      <div class="flex h-full flex-1 flex-col">
        <div ref="header" class="flex items-center p-2">
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
          :space-id="spaceId"
          kind="sticky"
          class="h-full flex-1"
        />
      </div>
    </mx-filled-card>
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
    @apply headline-large text-on-primary-container z-1 w-0 flex-1 bg-transparent outline-none;

    touch-action: none;
    user-select: none;

    font-family: var(--font-mono);
  }

  ::selection {
    @apply text-on-primary-container;
    background-color: v-bind(selectionBackground);
  }
}
</style>
