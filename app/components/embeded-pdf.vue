<script setup lang="ts">
import VuePdfEmbed from "vue-pdf-embed";

const source = defineModel<string | Uint8Array>();

const props = defineProps<{
  monochrome?: boolean;
}>();

const container = useTemplateRef("container");
const { width, height } = useElementSize(container);

async function loaded() {
  if (props.monochrome) {
    const canvases = container.value!.querySelectorAll("canvas");
    for (const canvas of canvases) rerender(canvas);
  }
}

const dark = useDark();

const theme = useMaterialTheme()!.value!;
const { surface } = theme.palette;

function rerender(canvas: HTMLCanvasElement) {
  const ctx = canvas.getContext("2d")!;
  const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
  const data = imageData.data;

  for (let i = 0; i < data.length; i += 4) {
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
  <div ref="container" class="size-full overflow-auto">
    <VuePdfEmbed
      :source="source"
      :width="width"
      :height="height"
      @rendered="loaded"
    />
  </div>
</template>

<style lang="scss">
@import "vue-pdf-embed/dist/styles/annotationLayer.css";
@import "vue-pdf-embed/dist/styles/textLayer.css";
</style>
