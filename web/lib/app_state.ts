export interface IState {
    playerNames: string[];
    hasConfirmed: boolean;
    cpuPlayerCount: number;
    gameNumber: number;
    showShortcutsModal: boolean;
    showNewGameForm: boolean;
}

export type Action =
    | {type: "addPlayer"}
    | {type: "removePlayer"}
    | {type: "updatePlayer", idx: number, name: string}
    | {type: "addCpuPlayer"}
    | {type: "removeCpuPlayer"}
    | {type: "startGame"}
    | {type: "newGame"}
    | {type: "setShowShortcutsModal", showShortcutsModal: boolean}
    | {type: "setShowNewGameForm", showNewGameForm: boolean};

export const initState = (): IState => ({
    playerNames: [""],
    hasConfirmed: false,
    cpuPlayerCount: 1,
    gameNumber: 1,
    showShortcutsModal: false,
    showNewGameForm: false,
});

export const reducer: React.Reducer<IState, Action> = (prevState, action) => {
    switch (action.type) {
        case "addPlayer":
            return {...prevState, playerNames: [...prevState.playerNames, ""]};
        case "removePlayer":
            return {
                ...prevState,
                playerNames: prevState.playerNames.filter((_, i) =>
                    i !== prevState.playerNames.length - 1)
            };
        case "updatePlayer":
            return {
                ...prevState,
                playerNames: prevState.playerNames.map((name, i) =>
                    i === action.idx ? action.name : name),
            }
        case "addCpuPlayer":
            return {...prevState, cpuPlayerCount: prevState.cpuPlayerCount + 1};
        case "removeCpuPlayer":
            return {...prevState, cpuPlayerCount: prevState.cpuPlayerCount - 1};
        case "startGame":
            return {...prevState, hasConfirmed: true};
        case "newGame":
            return {...prevState, hasConfirmed: false, gameNumber: prevState.gameNumber + 1};
        case "setShowShortcutsModal":
            return {...prevState, showShortcutsModal: action.showShortcutsModal};
        case "setShowNewGameForm":
            return {...prevState, showNewGameForm: action.showNewGameForm};
        default:
            return prevState;
    }
}
