<script setup lang="ts">
import VuePdfEmbed from "vue-pdf-embed";

const source = defineModel<string | Uint8Array>();

const props = withDefaults(
  defineProps<{
    filename?: string;
    monochrome?: boolean;
  }>(),
  {
    filename: "document.pdf",
    monochrome: false,
  },
);

const container = useTemplateRef("container");
const pdfEmbed = useTemplateRef("pdfEmbed");

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

const downloading = ref(false);
async function download() {
  downloading.value = true;
  await pdfEmbed.value?.download(props.filename);
  downloading.value = false;
}

const printing = ref(false);
async function print() {
  printing.value = true;
  await pdfEmbed.value?.print(undefined, props.filename);
  printing.value = false;
}
</script>

<template>
  <div class="relative size-full overflow-hidden">
    <div ref="container" class="size-full overflow-auto">
      <VuePdfEmbed
        ref="pdfEmbed"
        :source="source"
        :width="width"
        :height="height"
        @rendered="loaded"
      />
    </div>

    <m3-toolbar type="floating" class="justify-end">
      <md-icon-button :disabled="downloading" @click="download">
        <md-icon>download</md-icon>
      </md-icon-button>

      <md-icon-button :disabled="printing" @click="print">
        <md-icon>print</md-icon>
      </md-icon-button>
    </m3-toolbar>
  </div>
</template>

<style lang="scss">
@import "vue-pdf-embed/dist/styles/annotationLayer.css";
@import "vue-pdf-embed/dist/styles/textLayer.css";
</style>
