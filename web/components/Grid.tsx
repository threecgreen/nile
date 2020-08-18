import React from "react";
import styles from "./Grid.module.css"

interface IProps {
    row: number,
    column: number,
}

export const GridCell: React.FC<IProps> = ({row, column, ...props}) => (
    <div style={ {
        gridColumn: `${column + 1}`,
        gridRow: row + 1 } }
    >
        { props.children }
    </div>
)
GridCell.displayName = "GridCell";

export const Grid: React.FC<{width: number}> = ({width, ...props}) => (
    <div className={ styles.grid } style={ {width} }>
        { props.children }
    </div>
)
Grid.displayName = "Grid";
