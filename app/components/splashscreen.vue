<script setup lang="ts">
const emit = defineEmits<{ (e: "ready", isReady: boolean): void }>();

const { steps, currentStep, completedSteps, isComplete, startStep } = useSteps({
  initialSteps: [
    "mount",
    "fonts",
    "typst",
    // "nuxt",
  ],
  onComplete: () => emit("ready", true),
});

const ready = isComplete;

// Font progress
const fontTotal = ref(0);
const fontLoaded = ref(0);

const currentLoadingId = computed(
  () => steps.value.find((s: any) => s.status === "loading")?.id ?? "",
);

onMounted(async () => {
  // Initialize app
  const mount = await startStep("mount");
  // await nextTick();
  mount.complete();

  // Load fonts
  const fontsStep = await startStep("fonts");
  const fontImports = getTypstFontImports().flat();
  fontTotal.value = fontImports.length;
  fontLoaded.value = 0;

  for (const imp of fontImports) {
    await imp;
    fontLoaded.value++;
  }

  fontsStep.complete();

  // Initialize Typst
  const typst = await startStep("typst");
  const typstState = useTypst();
  await typstState;
  typst.complete();
});

// onNuxtReady(async () => {
//   const nuxt = await startStep("nuxt");
//   await nextTick();
//   nuxt.complete();
// });
</script>

<template>
  <mx-theme id="splashscreen" color="#16161d" dark :class="{ ready }">
    <div class="m-9 flex w-full max-w-sm flex-col gap-6">
      <div class="flex flex-col gap-3">
        <div class="flex items-baseline justify-between">
          <span class="title-large font-bold tracking-tight">
            {{ currentStep || $t("components.splashscreen.loading") }}
            <span v-if="currentLoadingId === 'fonts'">
              ({{ fontLoaded }}/{{ fontTotal }})
            </span>
          </span>
          <span class="text-primary-fixed/80 font-mono text-sm">
            {{ completedSteps }}/{{ steps.length }}
          </span>
        </div>

        <div class="flex h-2 gap-1.5">
          <div
            v-for="step in steps"
            :key="step.id"
            class="flex-1 rounded transition-all duration-300 ease-out"
            :class="{
              'bg-primary-fixed scale-y-100': step.status === 'done',
              'bg-primary-fixed/80 scale-y-100 animate-pulse':
                step.status === 'loading',
              'bg-on-primary-fixed/30 scale-y-75': step.status === 'pending',
            }"
          />
        </div>
      </div>

      <div
        class="border-primary-fixed/20 label-large flex flex-col border-l-2 pl-4 font-mono"
      >
        <div
          v-for="step in steps"
          :key="step.id"
          class="transition-all duration-300"
          :class="{
            'text-primary-fixed translate-x-1': step.status === 'loading',
            'text-primary-fixed/60': step.status === 'done',
            'text-primary-fixed/40': step.status === 'pending',
          }"
        >
          <div class="flex items-baseline gap-3">
            <span
              class="mt-2 h-2 w-2 shrink-0 rounded-sm"
              :class="{
                'bg-primary-fixed animate-pulse': step.status === 'loading',
                'bg-primary-fixed/60': step.status === 'done',
                'bg-primary-fixed/20': step.status === 'pending',
              }"
            />
            <span>
              {{ $t(`components.splashscreen.steps.${step.id}`) }}
              <span v-if="step.id === 'fonts'">
                ({{ fontLoaded }}/{{ fontTotal }})
              </span>
            </span>
          </div>
        </div>
      </div>
    </div>
  </mx-theme>
</template>

<style lang="scss">
#splashscreen {
  position: fixed;
  inset: 0;
  z-index: 12;
  display: flex;
  height: 100%;
  width: 100%;
  align-items: center;
  justify-content: center;
  overflow: hidden;
  opacity: 1;
  transition: all 400ms cubic-bezier(0.4, 0, 0.2, 1);
  animation: fade-in 600ms cubic-bezier(0.4, 0, 0.2, 1);
  background-color: #4c4d72;
  color: #cecefa;

  &.ready {
    pointer-events: none;
    opacity: 0;
    user-select: none;
    transform: scale(1.05);
    filter: blur(4px);
  }

  @keyframes fade-in {
    from {
      opacity: 0;
      transform: scale(0.98);
    }
    to {
      opacity: 1;
      transform: scale(1);
    }
  }
}
</style>
