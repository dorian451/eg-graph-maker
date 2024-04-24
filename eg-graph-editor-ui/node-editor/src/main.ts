import App from "./App.svelte";

let app: App | null = null;

export function init(element_name: string) {
  app = new App({
    target: document.getElementById(element_name)!,
  });
}

export function new_node(new_id: string, parent: string) {
  app?.nodes.update((v) => {
    v.push({
      id: new_id,
      position: { x: 0, y: 0 },
      data: [],
    });

    return v;
  });
}
