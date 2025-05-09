<script setup lang="ts">
import { VuePDF, usePDF } from "@tato30/vue-pdf";

import { UseVirtualList } from "@vueuse/components";

const { pdf, pages, info } = usePDF("article2.pdf");
const list = computed(() =>
  pages.value ? Array.from({ length: pages.value }, (_, i) => i + 1) : [1],
);

const containerRef = useTemplateRef("container");

const width = ref(0);
const height = ref(0);

useResizeObserver(containerRef, (entries) => {
  const { contentRect } = entries[0]!;

  width.value = contentRect.width;
  height.value = contentRect.height;
});

const dark = useDark();

const { palette } = useMaterialTheme()!;
const { surface, primary } = palette;

// console.log(primary.r, primary.g, primary.b);

function rerender(page: number) {
  const container = containerRef.value!;
  const canvases = container.querySelectorAll("canvas");
  const canvas = canvases[page - 1] as HTMLCanvasElement;

  const ctx = canvas.getContext("2d")!;
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  const data = imageData.data;

  for (let i = 0; i < data.length; i += 4) {
    // greyscale and invert + primary
    const avg = (data[i]! + data[i + 1]! + data[i + 2]!) / 3;
    let scale = (255 - avg) / 1.25;
    if (dark.value) scale = -scale;

    data[i] = surface.r - scale;
    data[i + 1] = surface.g - scale;
    data[i + 2] = surface.b - scale;
  }

  ctx.putImageData(imageData, 0, 0);
}
</script>

<template>
  <div id="pdf-viewer" ref="container">
    <!-- <div v-for="page in pages" :key="page">
      <VuePDF :pdf="pdf" :page="page" text-layer />
    </div> -->
    <!-- <VuePDF :pdf="pdf" fit-parent text-layer /> -->
    <UseVirtualList
      v-if="height"
      :list="list"
      :options="{ itemHeight: height }"
      :width="width"
      :height="height"
    >
      <template #default="{ data: page }">
        <VuePDF
          :pdf="pdf"
          :page="page"
          fit-parent
          text-layer
          @loaded="() => rerender(page)"
        />
      </template>
    </UseVirtualList>
    <!-- <VuePDF
      v-for="page in pages"
      :pdf="pdf"
      :page="page"
      fit-parent
      text-layer
      @loaded="() => rerender(page)"
    /> -->
  </div>
</template>

<style lang="scss">
@import "@tato30/vue-pdf/style.css";

#pdf-viewer {
  @apply h-full w-full overflow-auto;

  /* canvas {
    visibility: hidden;
  } */

  span {
    /* color: v-bind(onSurface) !important; */
    // font-family: ui-sans-serif, system-ui, sans-serif, "Apple Color Emoji",
    //   "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji" !important;
  }
}
</style>
