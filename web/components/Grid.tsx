import React from "react";
import styles from "./Grid.module.css"

export const GridCell: React.FC = ({...props}) => (
    <td>
        { props.children }
    </td>
)
GridCell.displayName = "GridCell";

export const Grid: React.FC<{width: number}> = ({width, ...props}) => (
    <div className={ styles.grid } style={ {width} }>
        { props.children }
    </div>
)
Grid.displayName = "Grid";
