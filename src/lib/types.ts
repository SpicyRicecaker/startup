export interface Icon {
    path: string
}

enum ActionType {
    Open,
    GitPull
}

enum Await {
    Spawn,
    Output
}

interface ActionSpecificActions {
    command?: string,
    exit_code: Await
}

export interface Action {
    name: string,
    icon: Icon,
    type: ActionType
    props: ActionSpecificActions
}