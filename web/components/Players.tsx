import { RowCollapse24, RowExpand24 } from "@carbon/icons-react";
import { PlayerData } from "lib/common";
import React from "react";
import { Button } from "./Button";
import { Player } from "./Player";
import styles from "./Players.module.css";

interface IProps {
    currentPlayerId: number;
    playerData: PlayerData[];
    selectedTileIdx: number | null;
    /** Called when a rack tile is selected: either a drag event starts or it's clicked */
    onSelect: (idx: number) => void;
}

export const Players: React.FC<IProps> = ({currentPlayerId, playerData, selectedTileIdx, onSelect}) => {
    const [isExpanded, setIsExpanded] = React.useState(false);
    return (
        <div className={ styles.players }>
            <div style={ {columnCount: playerData.length} }>
                { playerData.map((player, id) => (
                    <Player player={ player }
                        key={ player.name }
                        id={ id }
                        selectedTileIdx={ selectedTileIdx }
                        isCurrentTurn={ id === currentPlayerId }
                        isCpu={ player.isCpu }
                        isExpanded={ isExpanded }
                        onSelect={ onSelect }
                    />
                )) }
            </div>
            <Button className={ styles.expandCollapse }
                enabled={ playerData[0].scores.length > 0 }
                title={ `${isExpanded ? "Collapse" : "Expand"} scores`}
                onClick={ () => setIsExpanded(!isExpanded) }
            >
                { isExpanded
                    ? <><RowCollapse24 aria-label="Collapse scores" /> Collapse </>
                    : <><RowExpand24 aria-label="Expand scores" /> Expand</> }
            </Button>
        </div>
    );
}
Players.displayName = "Players";
