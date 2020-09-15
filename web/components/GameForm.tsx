import React from "react";
import { PlayerNameInput } from "./PlayerNameInput";
import { Button } from "./Button";
import { AddFilled16, Subtract16 } from "@carbon/icons-react";
import styles from "./GameForm.module.css";

interface IProps {
    playerNames: string[];
    cpuPlayerCount: number;

    onPlayerNameChange: (text: string, i: number) => void;
    onAddPlayer: () => void;
    onRemovePlayer: () => void;
    onAddCpuPlayer: () => void;
    onRemoveCpuPlayer: () => void;
    onClickStart: () => void;
}

export const GameForm: React.FC<IProps> = ({
    playerNames, cpuPlayerCount,
    onPlayerNameChange, onAddPlayer, onRemovePlayer, onAddCpuPlayer, onRemoveCpuPlayer,
    onClickStart,
}) => {
    const totalPlayerCount = playerNames.length + cpuPlayerCount;
    const canStart = totalPlayerCount >= 2 && totalPlayerCount <= 4;
    return (
        <form>
            { playerNames.map((name, i) => (
                <PlayerNameInput key={ i }
                    i={ i }
                    name={ name }
                    onChange={ (t) => onPlayerNameChange(t, i) }
                />
            )) }
            <Button title="Add player"
                onClick={ onAddPlayer }
                enabled={ totalPlayerCount < 4 }
            >
                <AddFilled16 aria-label="Add player" />
            </Button>
            <Button title="Remove player"
                // Remove last player
                onClick={ onRemovePlayer }
                enabled={ totalPlayerCount > 1 && playerNames.length > 1 }
            >
                <Subtract16 aria-label="Remove player" />
            </Button>
            <br/>
            <span className={ styles.cpuCount }>CPU players: { cpuPlayerCount }</span>
            <Button title="Add CPU player"
                onClick={ onAddCpuPlayer }
                enabled={ totalPlayerCount < 4 }
            >
                <AddFilled16 aria-label="Add CPU player" />
            </Button>
            <Button title="Remove CPU player"
                onClick={ onRemoveCpuPlayer }
                enabled={ totalPlayerCount > 1 && cpuPlayerCount > 1 }
            >
                <Subtract16 aria-label="Remove CPU player" />
            </Button>
            <br/>
            <Button title={ canStart ? "Start new game" : "Need at least two players" }
                onClick={ onClickStart }
                enabled={ canStart }
            >
                Start
            </Button>
        </form>
    );
}