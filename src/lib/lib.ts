import type { Action } from "$lib/types";
import { ActionType, ExecType } from "$lib/types";

export const new_action = (): Action => ({
    name: "",
    icon: "",
    action_type: ActionType.Open,
    config: {
        program: "",
        exec_type: ExecType.Spawn
    },
    run: false
})