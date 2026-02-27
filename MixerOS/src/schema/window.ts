export type display = {
    name: string
    id: number
    height: number
    width: number
    state: displayState
}

export enum displayState {
    Loading,
    StartScreen,
    Blank,
    MainDisplay,
    Routing,
    Communication,
    Meters,
    FX,
    Settings,
    System
}