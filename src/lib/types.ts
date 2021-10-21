export enum ActionType {
    Open,
    GitPull
}

export enum Await {
    Spawn,
    Output
}

export interface ActionSpecificActions {
    command: string,
    exit_code: Await
}

export interface Action {
    name: string,
    icon: string,
    types: ActionType,
    props: ActionSpecificActions,
    run: boolean
}