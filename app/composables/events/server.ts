import type { Event, WeeklyEventsOptions } from ".";

const currentEvents: Event[][] = [[], [], [], [], [], [], []];

function addEvent(event: Event) {
  currentEvents[event.from.getDay()].push(event);
}

addEvent({
  space: "General Chemistry II",
  from: new Date(2024, 1, 5, 11, 0),
  to: new Date(2024, 1, 5, 12, 15),
});
addEvent({
  space: "US Politics and Government",
  from: new Date(2024, 1, 5, 12, 30),
  to: new Date(2024, 1, 5, 13, 20),
});
addEvent({
  space: "General Chemistry II",
  from: new Date(2024, 1, 5, 14, 0),
  to: new Date(2024, 1, 5, 16, 50),
});

addEvent({
  space: "Calculus II",
  from: new Date(2024, 1, 6, 10, 0),
  to: new Date(2024, 1, 6, 11, 40),
});
addEvent({
  space: "Logical Reasoning",
  from: new Date(2024, 1, 6, 14, 0),
  to: new Date(2024, 1, 6, 15, 20),
});

addEvent({
  space: "General Chemistry II",
  from: new Date(2024, 1, 7, 11, 0),
  to: new Date(2024, 1, 7, 12, 15),
});
addEvent({
  space: "US Politics and Government",
  from: new Date(2024, 1, 7, 12, 30),
  to: new Date(2024, 1, 7, 13, 20),
});
addEvent({
  space: "US Politics and Government",
  from: new Date(2024, 1, 7, 14, 0),
  to: new Date(2024, 1, 7, 14, 50),
});
addEvent({
  space: "General Chemistry II",
  from: new Date(2024, 1, 7, 16, 0),
  to: new Date(2024, 1, 7, 17, 20),
});

addEvent({
  space: "Calculus II",
  from: new Date(2024, 1, 8, 10, 0),
  to: new Date(2024, 1, 8, 11, 40),
});
addEvent({
  space: "Logical Reasoning",
  from: new Date(2024, 1, 8, 14, 0),
  to: new Date(2024, 1, 8, 15, 20),
});

export function useEvents() {
  return { currentEvents };
}

const weeklyEvents = [];

export function useWeeklyEvents(options: WeeklyEventsOptions) {
  return weeklyEvents;
}
