import { writable, type Writable } from "svelte/store";
import type { Event } from "../../bindings/Event";

const key = "msgs";

export function eventStore() {
  const storedValue = sessionStorage.getItem(key);
  const initial = storedValue ? (JSON.parse(storedValue) as Event) : null;

  const store: Writable<Event | null> = writable<Event | null>(initial, () => {
    const unsubscribe = store.subscribe((value) => {
      sessionStorage.setItem(key, JSON.stringify(value));
    });
    return unsubscribe;
  });

  return store;
}
