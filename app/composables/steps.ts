export interface Step {
  id: string;
  status: "pending" | "loading" | "done";
}

interface UseStepsOptions {
  initialSteps: string[];
  onComplete?: () => void;
}

export function useSteps({ initialSteps, onComplete }: UseStepsOptions) {
  const { t } = useSharedI18n();

  const steps = ref<Step[]>(
    initialSteps.map((id) => ({
      id,
      status: "pending",
    })),
  );

  const currentStep = computed(() => {
    const loadingStep = steps.value.find((s) => s.status === "loading");
    if (!loadingStep) return "";
    return t(`components.splashscreen.steps.${loadingStep.id}`);
  });

  const completedSteps = computed(
    () => steps.value.filter((s) => s.status === "done").length,
  );

  const isComplete = computed(
    () => completedSteps.value === steps.value.length,
  );

  // Watch for completion and call the callback if provided
  whenever(isComplete, () => {
    onComplete?.();
  });

  function updateStep(id: string, status: Step["status"]) {
    const step = steps.value.find((s) => s.id === id);
    if (step) step.status = status;
  }

  async function startStep(id: string) {
    updateStep(id, "loading");
    return {
      complete: () => updateStep(id, "done"),
      error: () => updateStep(id, "pending"),
    };
  }

  return {
    steps: readonly(steps),
    currentStep,
    completedSteps,
    isComplete,
    updateStep,
    startStep,
  };
}
