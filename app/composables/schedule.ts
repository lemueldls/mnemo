export interface ScheduleItem {
  spaceId: string;
  from: number;
  to: number;
}

export async function useSchedule() {
  const schedule = await useStorageMap<Record<number, ScheduleItem[]>>("schedule.json", {
    0: [],
    1: [],
    2: [],
    3: [],
    4: [],
    5: [],
    6: [],
  });

  return schedule;
}
