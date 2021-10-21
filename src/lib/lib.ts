import type { Action } from "$lib/types";
import { ActionType, Await } from "$lib/types";

export const tauri = (window as any).__TAURI__ as typeof import('@tauri-apps/api/tauri');

export const new_action = (): Action => ({
    name: "",
    icon: "",
    types: ActionType.Open,
    props: {
        command: "",
        exit_code: Await.Spawn
    },
    run: false
})