export enum ActionType {
    Open,
    GitPull
}

export enum ExecType {
    Spawn,
    Output
}

export interface Config {
    program: string,
    exec_type: ExecType
}

export interface Action {
    name: string,
    icon: string,
    action_type: ActionType,
    config: Config,
    run: boolean
}