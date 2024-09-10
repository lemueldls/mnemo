export interface ScheduleItem {
  spaceId: string;
  from: number;
  to: number;
}

export async function useSchedule() {
  const schedule = useStorageItem<ScheduleItem[][]>("schedule.json", [
    [],
    [],
    [],
    [],
    [],
    [],
    [],
  ]);
  // schedule.value = [[], [], [], [], [], [], []];

  return schedule;
}
